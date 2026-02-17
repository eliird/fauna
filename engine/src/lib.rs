use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(){
    // Step 1: get window and document
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor{
        backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
        ..Default::default()
    });

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let surface_target = wgpu::SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface_target).unwrap();





}
