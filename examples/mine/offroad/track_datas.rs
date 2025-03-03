use crate::track::{Track, TrackPiece, StraightData, CornerData, TrackData};
use bevy::math::Vec3;
use bevy::asset::weak_handle;
use bevy::prelude::Handle;

static TRACK_ADVANCED_PIECES: &[TrackPiece] = &[
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::from_length(4.0)),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_length(1.0)),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Straight(StraightData::from_length(10.0)),
    TrackPiece::Checkpoint,
    // TrackPiece::Straight(StraightData::from_length(14.0)),
    // TrackPiece::Checkpoint,
    // TrackPiece::Corner(CornerData::right_turn()),
    // TrackPiece::Straight(StraightData::from_length(11.0)),
    // TrackPiece::Checkpoint,
    // TrackPiece::Corner(CornerData::right_turn()),
    // TrackPiece::Straight(StraightData::default()),
    // TrackPiece::Checkpoint,
    // TrackPiece::Corner(CornerData::right_turn()),
    // TrackPiece::Straight(StraightData::from_length(3.0)),
    TrackPiece::Finish,
];

pub static TRACK_ADVANCED_DATA: TrackData = TrackData {
    pieces: &TRACK_ADVANCED_PIECES,
    initial_position: Vec3::new(-12.0, 0.0, 0.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
    initial_left: -1.0,
    initial_right: 1.0,
    num_segments: 4,
};

pub const TRACK_ADVANCED_HANDLE: Handle<Track> = weak_handle!("1347c9b7-c46a-48e7-0000-023a354b7cac");


static TRACK_BEGINNER_PIECES: [TrackPiece; 22] = [
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_length(8.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_length(14.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(11.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(3.0)),
    TrackPiece::Finish,
];

pub static TRACK_BEGINNER_DATA: TrackData = TrackData {
    pieces: &TRACK_BEGINNER_PIECES,
    initial_position: Vec3::new(-12.0, 0.0, 0.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
    initial_left: -1.0,
    initial_right: 1.0,
    num_segments: 4,
};

pub const TRACK_BEGINNER_HANDLE: Handle<Track> = weak_handle!("1347c9b7-c46a-48e7-1111-023a354b7cac");

static TRACK_VERTICAL_PIECES: [TrackPiece; 14] = [
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::from_length(5.0)),
    TrackPiece::Straight(StraightData::from_left_right(-1.0, 0.5)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(2.0)),
    TrackPiece::Straight(StraightData::from_left_right(-2.0, 1.0)),
    TrackPiece::Straight(StraightData::from_left_right_length(-2.0, 1.0, 4.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_left_right(-2.0, 1.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_left_right_length(-2.0, 1.0, 1.0)),
    TrackPiece::Finish,
];

pub static TRACK_VERTICAL_DATA: TrackData = TrackData {
    pieces: &TRACK_VERTICAL_PIECES,
    initial_position: Vec3::new(1.0, 2.0, 0.0),
    initial_forward: Vec3::new(-1.0, 0.0, 0.0),
    initial_up: Vec3::Z,
    initial_left: -2.0,
    initial_right: 1.0,
    num_segments: 4,
};

pub const TRACK_VERTICAL_HANDLE: Handle<Track> = weak_handle!("1347c9b7-c46a-48e7-2222-023a354b7cac");
