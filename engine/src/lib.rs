use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;

#[wasm_bindgen]
pub fn init(){
    // Step 1: get window and document

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    // get a webGL2 context

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // Step 4: set clear color to dark teal (r, g, b, a)
    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    // Step 5: clear the screen
    gl.clear(GL::COLOR_BUFFER_BIT);

}
