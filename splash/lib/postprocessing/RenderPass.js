/**
 * @author alteredq / http://alteredqualia.com/
 */

import THREE from "three";
import Pass from "./Pass.js";

const RenderPass = function ( scene, camera, overrideMaterial, clearColor, clearAlpha ) {

    Pass.call( this );

    this.scene = scene;
    this.camera = camera;

    this.overrideMaterial = overrideMaterial;

    this.clearColor = clearColor;
    this.clearAlpha = ( clearAlpha !== undefined ) ? clearAlpha : 0;

    this.clear = true;
    this.needsSwap = false;

};

RenderPass.prototype = Object.assign( Object.create( Pass.prototype ), {

    constructor: RenderPass,

    render: function ( renderer, writeBuffer, readBuffer, delta, maskActive ) {

        this.scene.overrideMaterial = this.overrideMaterial;

        var oldClearColor, oldClearAlpha;

        if ( this.clearColor ) {

            oldClearColor = renderer.getClearColor().getHex();
            oldClearAlpha = renderer.getClearAlpha();

            renderer.setClearColor( this.clearColor, this.clearAlpha );

        }

        renderer.render( this.scene, this.camera, this.renderToScreen ? null : readBuffer, this.clear );

        if ( this.clearColor ) {

            renderer.setClearColor( oldClearColor, oldClearAlpha );

        }

        this.scene.overrideMaterial = null;

    }

} );

export default RenderPass;

