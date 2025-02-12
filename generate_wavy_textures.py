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
    xxs, yys = np.meshgrid(
        np.linspace(0.0, 1.0, resolution),
        np.linspace(0.0, 1.0, resolution),
    )
    factor = 0.3
    amplitude = 0.1
    phis = 8.0 * np.pi * (xxs + amplitude * np.cos(2.0 * np.pi * yys))
    hhs = np.cos(phis)
    gxs = -np.sin(phis) * factor
    gys = np.sin(phis) * amplitude * 2.0 * np.pi * -np.sin(2.0 * np.pi * yys) * factor

    assert (hhs >= -1.0).all()
    assert (hhs <= 1.0).all()
    height_colors = np.array(
        [
            0.5 + hhs * 0.5,
            0.5 + hhs * 0.5,
            0.5 + hhs * 0.5,
        ]
    ).transpose()
    save_texture(Path("wavy_height.png"), height_colors)
    plt.figure()
    plt.imshow(height_colors)

    grad_squared_norms = np.square(gxs) + np.square(gys)
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
            0.5 + gys * 0.5,
            0.5 + gzs * 0.5,
        ]
    ).transpose()
    save_texture(Path("wavy_normalmap.png"), normalmap_colors)
    plt.figure()
    plt.imshow(normalmap_colors)

    plt.show()
