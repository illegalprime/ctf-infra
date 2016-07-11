import THREE from "three";

class Cube {
    constructor(opts) {
        this.geometry = new THREE.BoxGeometry(
            opts.width || 1,
            opts.height || 1,
            opts.depth || 1
        );

        this.material = new THREE.MeshBasicMaterial({
            color: opts.color || 0x00ff00,
        });

        this.mesh = new THREE.Mesh(this.geometry, this.material);

        this.state = {};

        this.update = (global) => {
            opts.update(this.mesh, this.state, global);
        };
    }
}

export default Cube;

