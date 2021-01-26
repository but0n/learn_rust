pub use shaderc::{
    Compiler,
    ShaderKind,
};

use super::Contex;

use std::collections::HashMap;

// #[derive(Debug)]
pub struct ShaderManager{
    cp: Compiler,
    // pool: HashMap<String, Shader<'a>>,
}


static mut compiler:Option<Compiler> = None;
// const cp:Option<Compiler> = Compiler::new();


impl ShaderManager {
    // const compiler: Option<Compiler> = Compiler::new();

    pub fn new() -> Self {
        let cp = Compiler::new()
            .expect("Unable to create shader compiler");
        // let pool: HashMap<String, Shader> = HashMap::new();
        ShaderManager {
            cp,
            // pool,
        }
    }

    pub fn load_str(code: &'static str) -> Shader{
        // TODO: Cache
        Shader::new().load_code(code)
    }
}

pub struct Shader<'a> {
    pub ctx: Option<&'a mut Contex>,
    pub label: &'a str,
    pub code: Option<&'static str>,
    pub entry: &'static str,
    pub tp: Option<ShaderKind>,
    pub module: Option<wgpu::ShaderModule>,
}

impl<'a> Shader<'a> {
    pub fn new() -> Self {
        Shader {
            ctx: None,
            label: "Name",
            code: None,
            entry: "main",
            tp: None,
            module: None,
        }
    }
    pub fn new_vs() -> Self {
        Self::new()
            .set_type(ShaderKind::Vertex)
    }

    pub fn set_type(mut self, tp: ShaderKind) -> Self {
        self.tp = Some(tp);
        self
    }
    pub fn set_label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }
    pub fn load_code(mut self, code: &'static str) -> Self {
        self.code = Some(code);
        self
    }

    /// Compile and create a new shader module
    pub fn create_module(&self, ctx: &mut Contex) -> Option<wgpu::ShaderModule> {
        if self.code.is_none() {
            println!("Shader is empty!");
            return None
        }
        if let Some(code) = self.code {
            let mut cp = Compiler::new().unwrap(); // Hope this crate has cache inside :)
            let bin = cp.compile_into_spirv(
                code,
                self.tp.unwrap(),
                self.label,
                self.entry,
                None,
            ).expect("Failed to compile shader!");

            let data = wgpu::read_spirv(std::io::Cursor::new(bin.as_binary_u8())).unwrap();

            Some(ctx.device.create_shader_module(&data))
        } else {
            None
        }
    }

    pub fn get_stage_from_module (&self, module: &'a wgpu::ShaderModule) -> wgpu::ProgrammableStageDescriptor {
        wgpu::ProgrammableStageDescriptor {
            module: module,
            entry_point: self.entry,
        }
    }

    /// Create module and descriptor (always create a new one with same module)
    pub fn get_stage(&mut self, ctx: &mut Contex) -> wgpu::ProgrammableStageDescriptor {
        if self.module.is_none() {
            self.module = self.create_module(ctx);
        }
        self.get_stage_from_module(self.module.as_ref().unwrap())
    }
}

pub struct Material<'a> {
    pub label: &'static str,
    pub vs: Shader<'a>,
    pub fs: Shader<'a>,
}

impl<'a> Material<'a> {
    // pub fn new(ctx: &mut Contex, name: &'static str, vs_code: &'static str, fs_code: &'static str) -> Self {
    pub fn new(name: &'static str, vs_code: &'static str, fs_code: &'static str) -> Self {
        Material {
            label: name,

            vs: Shader::new()
                // .set_label(format!("{}.vert", &name))
                .set_type(ShaderKind::Vertex)
                .load_code(vs_code),

            fs: Shader::new()
                // .set_label(format!("{}.frag", &name))
                .set_type(ShaderKind::Fragment)
                .load_code(fs_code),
        }
    }

    pub fn get_stage(&mut self, ctx: &mut Contex) -> (wgpu::ProgrammableStageDescriptor, wgpu::ProgrammableStageDescriptor) {
        (
            self.vs.get_stage(ctx),
            self.fs.get_stage(ctx),
        )
    }

    pub fn get_uniform_layout(&self, ctx: &Contex, uniforms: Vec<i32>) -> wgpu::BindGroupLayout {
        ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        })
    }
}