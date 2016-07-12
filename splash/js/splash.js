import Cube from "./cube.js";
import Renderer from "./renderer.js";
import Typewriter from "./typewriter.js";
import GlitchPass from "../lib/postprocessing/GlitchPass.js";
import DotScreenShader from "../lib/shaders/DotScreenShader.js";
import ShaderPass from "../lib/postprocessing/ShaderPass.js";
import * as OxygenMono from "../assets/fonts/oxygen_mono_regular.json";
import THREE from "three";

window.onload = () => {
    "use strict";

    const engine = new Renderer({
        camera: {
            fov: 30,
            near: 0.1,
            far: 1500,
        },
        antialias: true,
        debug: true,
    });

    engine.camera.position.set(0, 0, 1000);
    engine.camera.lookAt(new THREE.Vector3(0, 0, 0));
    engine.render();

    const text = new Typewriter({
        text: "Capture the Flag (Y/n)? ",
        font: OxygenMono,
        size: 20,
        height: 0,
        curveSegments: 4,
        speed: 4,
        color: {
            font: 0x00ff00,
            font_side: 0x009900,
        },
        cursor: {
            text: "_",
            speed: 20,
        },
        after: () => {
            // Add glitch effect
            console.log("Animation Finished");
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

    const dotScreenPass = new ShaderPass(DotScreenShader);
    engine.composer.addPass(dotScreenPass);

    const glitchPass = new GlitchPass();
    glitchPass.renderToScreen = true;
    engine.composer.addPass(glitchPass);
};

