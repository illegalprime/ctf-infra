import Cube from "./cube.js";
import Renderer from "./renderer.js";
import THREE from "three";

window.onload = () => {
    "use strict";

    const engine = new Renderer({
        camera: {
            fov: 75,
            near: 0.1,
            far: 1000,
        },
    });

    const cube = new Cube({
        width: 1,
        height: 1,
        depth: 1,
        color: 0xff0000,
        update: (mesh) => {
            mesh.rotation.x += 0.1;
            mesh.rotation.y += 0.1;
        },
    });

    engine.add(cube);

    engine.camera.position.z = 5;
};

