import Cube from "./cube.js";
import Renderer from "./renderer.js";
import Typewriter from "./typewriter.js";
import * as DroidMono from "../assets/droid_sans_mono_regular.typeface.json";
import THREE from "three";

window.onload = () => {
    "use strict";

    const engine = new Renderer({
        camera: {
            fov: 30,
            near: 0.1,
            far: 1500,
        },
        debug: true,
    });

    engine.camera.position.set(0, 0, 1000);
    engine.camera.lookAt(new THREE.Vector3(0, 0, 0));

    const text = new Typewriter({
        text: "Illuminati Δ",
        font: DroidMono,
        size: 70,
        height: 20,
        curveSegments: 4,
        speed: 5,
        color: {
            font: 0x00ff00,
            font_side: 0x009900,
        },
    });

    // Get the middle of the text to offset.
    text.final_geometry.computeBoundingBox();
    const text_bb = text.final_geometry.boundingBox;
    const middle = -0.5 * (text_bb.max.x - text_bb.min.x);

    // Position and rotate to face us.
    text.mesh.position.x = middle;
    text.mesh.position.y = 0;
    text.mesh.position.z = 0;
    text.mesh.rotation.x = 0;
    text.mesh.rotation.y = Math.PI * 2;

    engine.add(text);
};

