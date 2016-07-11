import THREE from "three";

class Typewriter {
    constructor(opts) {
        this.text = opts.text || "Sample text.";
        this.cursor = opts.cursor || {};
        this.cursor.text = this.cursor.text || "|";
        this.cursor.speed = this.cursor.speed || 20;
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
        this.final_geometry = this.make_text(this.text + this.cursor.text);
        this.text_mesh = new THREE.Mesh(this.geometry, this.material);

        this.mesh.add(this.text_mesh);

        this.progress = 0;

        this.speed = opts.speed || 200;
    }

    type_more(portion, cursor_visible) {
        // Add one more letter
        if (portion > this.text.length) {
            portion = this.text.length;
        }

        // Cleanup old mesh
        this.mesh.remove(this.text_mesh);
        this.text_mesh.geometry.dispose();

        // Create new mesh
        let current_text = this.text.substr(0, portion);
        if (cursor_visible) {
            current_text += this.cursor.text;
        }
        this.geometry = this.make_text(current_text);
        this.text_mesh = new THREE.Mesh(this.geometry, this.material);

        // Add it back to the group
        this.mesh.add(this.text_mesh);
    }

    update(global) {
        if (this.progress == 0) {
            this.start_time = global.ticks;
        }
        if ((global.ticks - this.start_time) % this.cursor.speed == 0) {
            this.cursor_visible = !this.cursor_visible;
        }
        if ((global.ticks - this.start_time) % this.speed == 0) {
            this.progress += 1;
        }
        this.type_more(this.progress, this.cursor_visible);
    }
}

export default Typewriter;

