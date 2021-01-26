pub mod shader;
pub use shader::*;

// pub mod material;
// pub use material::*;

// #[derive(Debug)]
pub struct Contex {
    pub surface: wgpu::Surface,
    pub adp: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub shaderManager: shader::ShaderManager,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swapchain: wgpu::SwapChain,
}

impl Contex {
    pub async fn new(window: &winit::window::Window) -> Self {

        let surface = wgpu::Surface::create(window);
        let adp = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface)
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .expect("Failed");
        let (device, queue) = adp.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        })
        .await;

        let size = window.inner_size();

        let mut sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let mut swapchain = device.create_swap_chain(&surface, &sc_desc);

        let shaderManager = shader::ShaderManager::new();

        Contex {
            surface,
            adp,
            device,
            queue,
            shaderManager,
            sc_desc,
            swapchain,
        }
    }
}

/// Manage multiple materials associate with mesh
pub struct MeshRenderer<'a> {
    pub pipeline: Option<wgpu::RenderPipeline>,
    // pub materials: Vec<Material<'a>>,
    // Material is mutable reference due to modules cache
    pub material: &'a mut Material<'a>,
    // pub ctx: &'a mut Contex,
}

impl<'a> MeshRenderer<'a> {
    pub fn new(ctx: &'a mut Contex, mat: &'a mut Material<'a>) -> Self {
        MeshRenderer {
            // ctx,
            pipeline: None,
            material: mat,
        }
    }

    pub fn get_pipeline(&mut self, ctx: &mut Contex, layouts: &[&wgpu::BindGroupLayout], vbo: &[wgpu::VertexBufferDescriptor]) -> Option<wgpu::RenderPipeline> {
        // if self.materials.is_empty() {
        //     println!("Material is empty!");
        //     return None
        // }

        // let mat = self.materials.first_mut().unwrap();
        // let vs = &mut mat.vs;
        // let fs = &mut mat.fs;
        // vs.tp = Some(ShaderKind::Vertex);
        // fs.tp = Some(ShaderKind::Fragment);
        // let vs_stage = vs.get_stage(ctx);
        // let fs_stage = fs.get_stage(ctx);
        let (vs_stage, fs_stage) = self.material.get_stage(ctx);

        let ctx: &Contex = ctx;

        let pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: layouts,
            }),

            vertex_stage: vs_stage,
            fragment_stage: Some(fs_stage),

            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                },
            ],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: vbo,
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Some(pipeline)
    }

    pub fn update_pipeline(&mut self, ctx: &mut Contex, layouts: &[&wgpu::BindGroupLayout], vbo: &[wgpu::VertexBufferDescriptor]) {
        self.pipeline = self.get_pipeline(ctx, layouts, vbo);
    }

    pub fn draw<'d>(&'a self, pass: &'d mut wgpu::RenderPass<'a>, vbo: &'a wgpu::Buffer, ebo: &'a wgpu::Buffer, ubo: &'a wgpu::BindGroup, len: u32) {
        // pub fn draw<'c>(&'c self, pass: &'a mut wgpu::RenderPass<'c>, vbo: &'c wgpu::Buffer, ebo: &'c wgpu::Buffer, ubo: &'c wgpu::BindGroup, len: u32) {
        // if self.pipeline.is_none() {return}
        match &self.pipeline {
            Some(p) => {
                // pass.set_pipeline(self.pipeline.as_ref().unwrap());
                pass.set_pipeline(p);
                pass.set_vertex_buffer(0, vbo, 0, 0);
                pass.set_index_buffer(ebo, 0, 0);
                pass.set_bind_group(0, ubo, &[]);
                pass.draw_indexed(0..len, 0, 0..1);

            }
            _ => {}
        }
    }
}