use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wgpu::util::DeviceExt;
use std::rc::Rc;
use std::cell::RefCell;


struct State {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    pipeline: wgpu::RenderPipeline,
    frame: RefCell<u64>,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}


impl State{
    fn render(&self){

        let mut frame = self.frame.borrow_mut();
        *frame += 1;
        let t = (*frame as f64) / 60.0;
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color{
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}


#[wasm_bindgen]
pub async fn init() {
    console_error_panic_hook::set_once();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::GL,
        ..Default::default()
    });

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let surface_target = wgpu::SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface_target).unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            ..Default::default()
        },
    ).await.unwrap();

    // Configure the surface
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: 800,
        height: 600,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    // Load the shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    // Create the render pipeline
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout{
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute{
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    wgpu::VertexAttribute{
                        offset: std::mem::size_of::<[f32; 3]>() as u64,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x3,
                    }
                ]
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    let axes: [Vertex; 6] = [
        // X axis — red
        Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
        // Y axis — green
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 1.0, 0.0] },
        // Z axis — blue
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [0.0, 0.0, 1.0], color: [0.0, 0.0, 1.0] },
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("Vertex buffer"),
        contents: bytemuck::cast_slice(&axes),
        usage: wgpu::BufferUsages::VERTEX,
    });
    // Render a frame — clear to dark teal
    let state = Rc::new(State{
        device,
        queue,
        surface,
        pipeline,
        frame: RefCell::new(0),
        vertex_buffer,
        num_vertices: axes.len() as u32,
    });

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let state_clone = state.clone();


    *g.borrow_mut() = Some(Closure::new(move || {
        state_clone.render();

        let window = web_sys::window().unwrap();
        window.request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        ).unwrap();
    }));

    let window = web_sys::window().unwrap();
    window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).unwrap();



}
