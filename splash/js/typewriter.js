import THREE from "three";

class Typewriter {
    constructor(opts) {
        this.text = opts.text || "Sample text.";
        this.mesh = new THREE.Object3D();

        this.make_text = (text) => {
            return new THREE.TextGeometry(text, {
                font: new THREE.Font(opts.font),
                size: opts.size,
                height: opts.height,
                curveSegments: opts.curveSegments,
                material: 0,
                extrudeMaterial: 1,
            });
        };

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

        this.geometry = this.make_text("");
        this.final_geometry = this.make_text(this.text);
        this.text_mesh = new THREE.Mesh(this.geometry, this.material);

        this.mesh.add(this.text_mesh);

        this.progress = 0;

        this.speed = opts.speed || 200;
    }

    type_more() {
        // Add one more letter
        this.progress += 1;
        if (this.progress > this.text.length) {
            return;
        }

        // Cleanup old mesh
        this.mesh.remove(this.text_mesh);
        this.text_mesh.geometry.dispose();

        // Create new mesh
        const current_text = this.text.substr(0, this.progress);
        this.geometry = this.make_text(current_text);
        this.text_mesh = new THREE.Mesh(this.geometry, this.material);

        // Add it back to the group
        this.mesh.add(this.text_mesh);
    }

    update(global) {
        if (this.progress == 0) {
            this.start_time = global.ticks;
        }
        if ((global.ticks - this.start_time) % this.speed == 0) {
            this.type_more();
        }
    }
}

export default Typewriter;

