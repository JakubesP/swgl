use std::marker::PhantomData;
use nalgebra_glm as glm;

use super::super::super::runtime_error::SWGLResult;
use super::super::super::gl_wrapper::vertex_array_object;
use super::super::super::gl_wrapper::vertex_buffer_object;
use super::super::vertex_2d::interface::{DedicatedShader, VertexType};
use super::super::super::camera2d::interface::CameraType;
use super::super::super::gl_wrapper::shader::{self, Program};
use super::super::super::gl_wrapper::texture::texture_2d::Texture2D;
use super::super::super::gl_wrapper::vertex_array_object::PrimitiveType;

use super::renderer_conf::RendererConf;
use super::helpers::create_buffers;

// -----------------------------------------------------------------------------------------------------------

/// This type is used for renders geometry direct from vertices. 
pub struct GeometryRenderer<T>
where
    T: VertexType,
{
    phantom: PhantomData<T>,
    program: Program,
    vao: vertex_array_object::VertexArrayObject,
    vbo: vertex_buffer_object::VertexBufferObject,
    conf: RendererConf,
}

// -----------------------------------------------------------------------------------------------------------

impl<T> GeometryRenderer<T>
where
    T: VertexType + DedicatedShader,
{
    pub fn init(context: &crate::AppContext, max_vertices_number: usize) -> SWGLResult<Self> {
        let program = Program::new(
            context,
            &shader::split_vfshader_to_shader_source(T::get_dedicated_shader()),
        )?;

        let (vbo, vao) = create_buffers::<T>(context, max_vertices_number)?;

        Ok(Self {
            phantom: PhantomData,
            program,
            vbo,
            vao,
            conf: RendererConf::default(),
        })
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> GeometryRenderer<T>
where
    T: VertexType,
{
    pub fn init_with_custom_shader(context: &crate::AppContext, max_vertices_number: usize, program: Program, conf: RendererConf) -> SWGLResult<Self> {
        let (vbo, vao) = create_buffers::<T>(context, max_vertices_number)?;

        Ok(Self {
            phantom: PhantomData,
            program,
            vbo,
            vao,
            conf,
        })
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> GeometryRenderer<T>
where
    T: VertexType,
{

    /// This method prepares given number of textures for shader. 
    pub fn prepare_textures(&self, context: &crate::AppContext, count: usize) -> SWGLResult<()> {
        for i in 0..count {
            self.program.set1i(context, &format!("{}{}", self.conf.texture_uniform_prefix, i), i as i32)?;
        }
        Ok(())
    }

    /// This method activates textures for shader before draw. 
    pub fn bind_textures(&self, context: &crate::AppContext, texture_set: &[&Texture2D]) {
        for (i, tex) in texture_set.iter().enumerate() {
            tex.active(context, i as u32);
        }
    }

    pub fn draw(&self, context: &crate::AppContext, vertices: &[T], draw_type: PrimitiveType, camera: &dyn CameraType) -> SWGLResult<()> {
        self.program.use_program(context);
        self.program.set_mat_4x4f(
            context,
            &self.conf.projection_matrix_uniform_name,
            glm::value_ptr(&camera.matrix()),
        ).unwrap();

        self.program.set_mat_4x4f(
            context,
            &self.conf.model_matrix_uniform_name,
            glm::value_ptr(&camera.dedicated_model()),
        ).unwrap();

        self.vao.bind(context);
        let mut vbo_data = vec![];
        for v in vertices {
            let mut portion = v.to_vec();
            vbo_data.append(&mut portion);
        }

        self.vbo.update_data_safe(context, &vbo_data, 0)?;
        self.vao.draw_arrays(context, draw_type, 0, vertices.len() as u32);

        Ok(())
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
