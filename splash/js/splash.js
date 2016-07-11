import Cube from "./cube.js";
import Renderer from "./renderer.js";
import Typewriter from "./typewriter.js";
import * as DroidMono from "../assets/droid_sans_mono_regular.typeface.json";
import * as OptimerBold from "../assets/optimer_bold.typeface.json";
import THREE from "three";

window.onload = () => {
    "use strict";

    const engine = new Renderer({
        camera: {
            fov: 75,
            near: 0.1,
            far: 1000,
        },
        debug: true,
    });

    engine.camera.position.z = 5;

    const cube = new Cube({
        width: 1,
        height: 1,
        depth: 1,
        color: 0xff0000,
        update: () => {},
    });

    const text = new Typewriter({
        font: OptimerBold,
        size: 70,
        height: 20,
        curveSegments: 4,
        speed: 200,
        color: {
            font: 0x00ff00,
        },
    });

    engine.add(text);

    engine.add(cube);
};

