use std::time::{Duration, Instant};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use red::{
    render::*,
    render::Material as Mat,
};

use gltf::*;

use specs::{
    Component, VecStorage,
    World, WorldExt, Builder,
    System, ReadStorage,
    RunNow,
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Color (f32, f32, f32);

struct PositionSys;
impl <'a> System<'a> for PositionSys {
    type SystemData = ReadStorage<'a, Position>;
    fn run(&mut self, pos: Self::SystemData) {
        use specs::Join;
        for position in pos.join() {
            println!("Hello, {:#?}", &position);
        }
    }
}

use cgmath::SquareMatrix;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    uTime: f32,
    // MVP: cgmath::Matrix4<f32>,
}
unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];



fn main() {
    println!("hello");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Color>();

    let ball = world.create_entity().with(
        Position {x: 1.0, y: 1.0, z: 1.0},
    ).build();

    let mut ps = PositionSys;
    // ps.run_now(&world);
    // ps.run_now(&world);
    // ps.run_now(&world);
    // ps.run_now(&world);
    // ps.run_now(&world);
    // ps.run_now(&world);
    world.maintain();



    // let (gltf,
    //     buffers,
    //     images) = import("examples/demo/DamagedHelmet.glb").unwrap();
    // println!("Buffers Length :{}", buffers.len());
    // for mesh in gltf.meshes() {
    //     println!("Mesh #{}", mesh.index());
    //     for primitive in mesh.primitives() {
    //         println!("\t - Primitive #{}", primitive.index());
    //         for (semantic, _) in primitive.attributes() {
    //             println!("\t\t -- {:#?}", semantic);
    //         }
    //     }
    // }



    // println!("Img Len :{}", images.len());
    // for mesh in gltf.meshes() {
    //     println!("Mesh #{}", mesh.index());
    //     for primitive in mesh.primitives() {
    //         println!("- Primitive #{}", primitive.index());
    //         let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    //         if let Some(iter) = reader.read_positions() {
    //             for vertex_position in iter {
    //                 println!("{:?}", vertex_position);
    //             }
    //         }
    //     }
    //  }
    // println!("{:#?}", document.meshes());


    let event_loop = EventLoop::new();

    // Window
    let win = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 2048,
            height: 1440,
        })
        .with_title("Hello rust")
        .build(&event_loop).unwrap();

    // Contex
    let mut ctx = futures::executor::block_on(Contex::new(&win));

    // let mut vs = Shader::new()
    //     .load_code(include_str!("shader.vert"))
    //     .set_type(ShaderKind::Vertex);
    // let mut fs = Shader::new()
    //     .load_code(include_str!("shader.frag"))
    //     .set_type(ShaderKind::Fragment);

    // let vs_module = &vs.compile(&mut ctx).unwrap();

    // let fs_module = &fs.compile(&mut ctx).unwrap();



    // Pipeline
    // let render_pipeline2 = MeshRenderer::new(
    //     &mut ctx,
    //     &mut Shader::new().load_code(include_str!("shader2.vert")),
    //     &mut Shader::new().load_code(include_str!("shader2.frag")),
    // );

    // let mats = vec![
    //     Mat::new(
    //         &"test",
    //         include_str!("shader2.vert"),
    //         include_str!("shader2.frag"),
    //         // include_str!("shader2.frag"),
    //     ),
    // ];

    let mut mat = Mat::new(
        &"test",
        include_str!("shader2.vert"),
        include_str!("shader2.frag"),
    );

    // This bind group layout associate with particular shader
    let uniform_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                },
            }
        ],
        label: Some("uniform_bind_group_layout"),
    });

    let mut uni = Uniforms {
        uTime: 0.,
        // MVP: cgmath::Matrix4::identity(),
    };

    // NOTE: Create uniform buffer which contains uTime
    let ubo = ctx.device.create_buffer_with_data(
        bytemuck::cast_slice(&[uni]),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );


    let uniform_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &ubo,
                    // FYI: you can share a single buffer between bindings.
                    range: 0..std::mem::size_of_val(&uni) as wgpu::BufferAddress,
                }
            }
        ],
        label: Some("uniform_bind_group"),
    });


    let num_ver = VERTICES.len() as u32;
    let num_ind = INDICES.len() as u32;
    // NOTE: VAO
    let vertex_buffer = ctx.device.create_buffer_with_data(
        bytemuck::cast_slice(VERTICES),
        wgpu::BufferUsage::VERTEX,
    );

    let index_buffer = ctx.device.create_buffer_with_data(
        bytemuck::cast_slice(INDICES),
        wgpu::BufferUsage::INDEX,
    );

    let vbo = wgpu::VertexBufferDescriptor {
        stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float3,
            },
            wgpu::VertexAttributeDescriptor {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float3,
            }
        ]
    };


    let mR = MeshRenderer::new(&mut ctx, &mut mat);
    // let render_pipeline2 = mR.get_pipeline(
    //     &mut ctx,
    //     &[&uniform_bind_group_layout],
    //     &[vbo],
    // ).unwrap();
    mR.update_pipeline(
        &mut ctx,
        &[&uniform_bind_group_layout],
        &[vbo],
    );
    // let render_pipeline = MeshRenderer::new(
    //     &mut ctx,
    //     &mut Shader::new().load_code(include_str!("shader.vert")),
    //     &mut Shader::new().load_code(include_str!("shader.frag")),
    // );



    let mut last_update = Instant::now();
    let (mut pool, spawner) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();
        (local_pool, spawner)
    };

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returens,
        // therefore we must do this to ensure the resources are properly cleaned up.
        let _ = (
            // &ctx,
            &ctx,
            &mR,
            // &vs,
            // &fs,
            // &pipeline_layout,
        );

        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10));

        match event {
            Event::MainEventsCleared => {
                // Render call
                if last_update.elapsed() > Duration::from_millis(20) {
                    win.request_redraw();
                    last_update = Instant::now();
                }

                pool.run_until_stalled();

            }

            Event::WindowEvent {
                event,
                window_id,
            } if window_id == win.id() => if true {
                match event {
                    // WindowEvent::KeyboardInput {
                    // } => {

                    // }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        println!("{:#?}", size);
                        // NOTE: Resize
                        ctx.sc_desc.width = size.width;
                        ctx.sc_desc.height = size.height;
                        ctx.swapchain = ctx.device.create_swap_chain(&ctx.surface, &ctx.sc_desc);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                // println!("{:#?}", _);
                let bg = Instant::now();
                let frame = ctx.swapchain.get_next_texture()
                    .expect("Timeout...");
                let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some(&"My command encoder")
                });

                uni.uTime += 1.;
                let staging_buffer = ctx.device.create_buffer_with_data(
                    bytemuck::cast_slice(&[uni]),
                    wgpu::BufferUsage::COPY_SRC,
                );
                encoder.copy_buffer_to_buffer(
                    &staging_buffer,
                    0,
                    &ubo,
                    0,
                    std::mem::size_of::<Uniforms>() as wgpu::BufferAddress
                    // std::mem::size_of::<Uniforms>() as wgpu::BufferAddress
                );


                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::BLACK,
                        }],
                        depth_stencil_attachment: None,
                    });
                    // rpass.set_pipeline(mR.pipeline.as_ref().unwrap());
                    mR.draw(&mut rpass, &vertex_buffer, &index_buffer, &uniform_bind_group, num_ind);
                    // rpass.draw(0..3, 0..1);
                    // rpass.set_pipeline(&render_pipeline2);
                    // rpass.set_vertex_buffer(0, &vertex_buffer, 0, 0);
                    // rpass.set_index_buffer(&index_buffer, 0, 0);
                    // rpass.set_bind_group(0, &uniform_bind_group, &[]);

                    // // rpass.draw(0..num_ver, 0..1);
                    // rpass.draw_indexed(0..num_ind, 0, 0..1);

                }

                ctx.queue.submit(&[encoder.finish()]);
                let dt = bg.elapsed().as_micros();
                let fps = if dt != 0 {
                    1000000/dt
                } else {
                    0
                };
                println!("Redraw FPS: {}", fps);


            }
            _ => {}
        }
    });


}