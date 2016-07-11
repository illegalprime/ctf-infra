import THREE from "three";

class Typewriter {
    constructor(opts) {
        this.text = opts.text || "Sample text.";

        this.geometry = new THREE.TextGeometry(this.text, {
            font: new THREE.Font(opts.font),
            size: opts.size,
            height: opts.height,
            curveSegments: opts.curveSegments,
            material: 0,
            extrudeMaterial: 1,
        });

        this.material = new THREE.MultiMaterial([
            // Font face material
            new THREE.MeshBasicMaterial({
                color: opts.color.font,
                shading: THREE.FlatShading,
            }),

            // Side of font material
            new THREE.MeshBasicMaterial({
                color: opts.color.font_side || opts.color.font,
                shading: THREE.SmoothShading,
            }),
        ]);

        this.mesh = new THREE.Mesh(this.geometry, this.material);

        this.progress = 0;

        this.speed = opts.speed || 200;
    }

    update(global) {
        if (this.progress == 0) {
            this.start_time = global.ticks;
        }
        if ((global.ticks - this.start_time) % this.speed == 0) {
            const current_text = this.text.substr(0, this.progress);
            this.progress += 1;
        }
    }
}

export default Typewriter;

