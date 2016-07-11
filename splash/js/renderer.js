import _ from "underscore";
import THREE from "three";

class Renderer {
    constructor(opts) {
        // Create scene
        this.scene = new THREE.Scene();
        this.renderer = new THREE.WebGLRenderer();
        this.camera = new THREE.PerspectiveCamera(
            opts.camera.fov || 75,
            window.innerWidth / window.innerHeight,
            opts.camera.near || 0.1,
            opts.camera.far || 1000
        );

        // Append to DOM
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(this.renderer.domElement);

        // Start to render
        this.renderables = {};
        this.state = {
            ticks: 0,
        };
        this.render();

        // Manage resizing
        window.addEventListener('resize', () => {
            this.camera.aspect = window.innerWidth / window.innerHeight;
            this.camera.updateProjectionMatrix();
            this.renderer.setSize(window.innerWidth, window.innerHeight);
        }, false);
    }

    render() {
        // Render
        requestAnimationFrame(this.render.bind(this));
        this.renderer.render(this.scene, this.camera);

        // Update objects
        setImmediate(() => {
            _.each(this.renderables, (update) => {
                update(this.state);
            });
            this.state.ticks += 1;
        });
    }

    add(object) {
        if (object.mesh && object.mesh.id && object.update) {
            this.renderables[object.mesh.id] = object.update.bind(object);
            this.scene.add(object.mesh);
        } else {
            throw new Error("Object did not have a mesh id or update.");
        }
    }
}

export default Renderer;

