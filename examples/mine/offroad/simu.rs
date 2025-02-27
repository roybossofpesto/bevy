use bevy::render::extract_component::{
    ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_asset::{RenderAssetUsages, RenderAssets};
use bevy::render::render_graph::{Node, RenderGraph, RenderLabel};
use bevy::render::render_resource::{
    binding_types::texture_storage_2d, binding_types::uniform_buffer, *,
};
use bevy::render::texture::GpuImage;
use bevy::render::{Render, RenderApp, RenderSet};

use std::borrow::Cow;

use bevy::prelude::*;

const SHADER_ASSET_PATH: &str = "shaders/offroad/simu_compute.wgsl";
const TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba32Float;
const SIMU_SIZE: (u32, u32) = (64, 64);
const WORKGROUP_SIZE: u32 = 8;

//////////////////////////////////////////////////////////////////////

#[derive(Component, ShaderType, ExtractComponent, Clone)]
struct SimuSettings {
    rng_seed: u32,
}

#[derive(Hash, Clone, Eq, PartialEq, Debug, RenderLabel)]
struct SimuLabel;

pub struct SimuPlugin;

impl Plugin for SimuPlugin {
    fn build(&self, app: &mut App) {
        info!("** build_simu **");

        app.add_plugins((
            // The settings will be a component that lives in the main world but will
            // be extracted to the render world every frame.
            // This makes it possible to control the effect from the main world.
            // This plugin will take care of extracting it automatically.
            // It's important to derive [`ExtractComponent`] on [`PostProcessingSettings`]
            // for this plugin to work correctly.
            ExtractComponentPlugin::<SimuSettings>::default(),
            // The settings will also be the data used in the shader.
            // This plugin will prepare the component for the GPU by creating a uniform buffer
            // and writing the data to that buffer every frame.
            UniformComponentPlugin::<SimuSettings>::default(),
        ));

        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<SimuImages>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(SimuLabel, SimuNode::default());
        render_graph.add_node_edge(SimuLabel, bevy::render::graph::CameraDriverLabel);

        app.add_systems(Startup, setup_simu);
    }
    fn finish(&self, app: &mut App) {
        info!("** simu_finish **");
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SimuPipeline>();
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Resource)]
struct SimuPipeline {
    group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for SimuPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let group_layout = render_device.create_bind_group_layout(
            None,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TEXTURE_FORMAT, StorageTextureAccess::ReadOnly),
                    texture_storage_2d(TEXTURE_FORMAT, StorageTextureAccess::WriteOnly),
                    uniform_buffer::<SimuSettings>(true),
                ),
            ),
        );

        let shader: Handle<Shader> = world.load_asset(SHADER_ASSET_PATH);

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("init_pipeline")),
            layout: vec![group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("update_pipeline")),
            layout: vec![group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            zero_initialize_workgroup_memory: false,
        });

        SimuPipeline {
            group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

#[derive(Resource, Clone, ExtractResource)]
struct SimuImages {
    image_a: Handle<Image>,
    image_b: Handle<Image>,
}

#[derive(Resource)]
struct SimuBindGroups {
    group_a: BindGroup,
    group_b: BindGroup,
}

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<SimuPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    simu_images: Res<SimuImages>,
    render_device: Res<bevy::render::renderer::RenderDevice>,
    simu_settings: Res<ComponentUniforms<SimuSettings>>,
) {
    let simu_binding = simu_settings.uniforms().binding();
    assert!(simu_binding.is_some());

    let view_a = gpu_images.get(&simu_images.image_a).unwrap();
    let view_b = gpu_images.get(&simu_images.image_b).unwrap();
    let group_a = render_device.create_bind_group(
        Some("group_a"),
        &pipeline.group_layout,
        &BindGroupEntries::sequential((
            &view_a.texture_view,
            &view_b.texture_view,
            simu_binding.clone().unwrap(),
        )),
    );
    let group_b = render_device.create_bind_group(
        Some("group_b"),
        &pipeline.group_layout,
        &BindGroupEntries::sequential((
            &view_b.texture_view,
            &view_a.texture_view,
            simu_binding.unwrap(),
        )),
    );
    let bind_groups = SimuBindGroups { group_a, group_b };
    commands.insert_resource(bind_groups);
}

//////////////////////////////////////////////////////////////////////

enum SimuState {
    Loading,
    Init,
    Update(bool),
}

struct SimuNode {
    state: SimuState,
}

impl Default for SimuNode {
    fn default() -> Self {
        Self {
            state: SimuState::Loading,
        }
    }
}

impl Node for SimuNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<SimuPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            SimuState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = SimuState::Init;
                }
            }
            SimuState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = SimuState::Update(true);
                }
            }
            SimuState::Update(aa) => {
                self.state = SimuState::Update(!aa);
            }
        }
    }

    fn run(
        &self,
        _graph_context: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let bind_groups = world.resource::<SimuBindGroups>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline_simu = world.resource::<SimuPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            SimuState::Loading => {}
            SimuState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline_simu.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups.group_a, &[0]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIMU_SIZE.0 / WORKGROUP_SIZE,
                    SIMU_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
            SimuState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline_simu.update_pipeline)
                    .unwrap();
                pass.set_bind_group(
                    0,
                    if !index {
                        &bind_groups.group_a
                    } else {
                        &bind_groups.group_b
                    },
                    &[0],
                );
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIMU_SIZE.0 / WORKGROUP_SIZE,
                    SIMU_SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
        }

        Ok(())
    }
}

fn setup_simu(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** setup_simu **");

    let mut image = Image::new_fill(
        Extent3d {
            width: SIMU_SIZE.0,
            height: SIMU_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TEXTURE_FORMAT,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image.sampler = bevy::image::ImageSampler::nearest();

    let image_a = images.add(image.clone());
    let image_b = images.add(image);

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image_a.clone()),
            ..StandardMaterial::default()
        })),
        Transform::from_xyz(-7.0, 3.0, -11.0).with_scale(Vec3::ONE * 6.0),
        SimuSettings { rng_seed: 42 },
    ));

    commands.insert_resource(SimuImages { image_a, image_b });
}
