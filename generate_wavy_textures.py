#!/usr/bin/env python

import numpy as np

import matplotlib.pyplot as plt
import matplotlib.image as img

from pathlib import Path


def save_texture(path: Path, ffs):
    path_ = Path("assets/textures") / path
    print(f'saving "{path_}"')
    img.imsave(path_, ffs)


if __name__ == "__main__":
    resolution: int = 512
    ww0: float = 8.0 * np.pi
    ww1: float = 2.0 * np.pi
    aa: float = 0.05
    bb: float = 0.03

    xxs, yys = np.meshgrid(
        np.linspace(0.0, 1.0, resolution),
        np.linspace(0.0, 1.0, resolution),
    )

    phis = ww0 * (xxs + aa * np.cos(ww1 * yys))
    hhs = bb * np.cos(phis)

    # dh/dx(p) = bb * -sin(phi(p)) * dphi/dx(p)
    # dh/dy(p) = bb * -sin(phi(p)) * dphi/dy(p)
    #          = bb * -sin(phi(p)) * ww0 * aa * ww1 * -np.sin(ww1 * yys)
    gxs = bb * -np.sin(phis) * ww0
    gys = bb * -np.sin(phis) * ww0 * aa * ww1 * -np.sin(ww1 * yys)

    height_colors = np.array([hhs, hhs, hhs]).transpose()
    height_colors = (height_colors - height_colors.min()) / (
        height_colors.max() - height_colors.min()
    )
    assert (height_colors >= 0.0).all()
    assert (height_colors <= 1.0).all()
    save_texture(Path("wavy_depth.png"), 1 - height_colors)
    plt.figure()
    plt.imshow(height_colors)

    grad_squared_norms = np.square(gxs) + np.square(gys)
    print(f"grad_norm_min {np.sqrt(grad_squared_norms.min())}")
    print(f"grad_norm_max {np.sqrt(grad_squared_norms.max())}")
    assert (grad_squared_norms >= 0.0).all()
    assert (grad_squared_norms <= 1.0).all()
    gzs = np.ones_like(gxs) - grad_squared_norms
    gzs = np.sqrt(gzs)
    assert np.abs(np.square(gxs) + np.square(gys) + np.square(gzs) - 1).max() < 1e-5

    assert (gxs >= -1.0).all()
    assert (gxs <= 1.0).all()
    assert (gys >= -1.0).all()
    assert (gys <= 1.0).all()
    assert (gzs >= -1.0).all()
    assert (gzs <= 1.0).all()
    normalmap_colors = np.array(
        [
            0.5 + gxs * 0.5,
            0.5 - gys * 0.5,
            0.5 + gzs * 0.5,
        ]
    ).transpose()
    save_texture(Path("wavy_normal.png"), normalmap_colors)
    plt.figure()
    plt.imshow(normalmap_colors)

    plt.show()
