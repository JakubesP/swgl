use super::super::runtime_error::{SWGLResult};
use super::super::gl_wrapper::vertex_array_object;
use super::super::gl_wrapper::vertex_buffer_object;
use super::vertex_2d::interface::VertexType;

/// This type is simple box for VAO & VBO & Data.
pub struct GeometryObject<T: VertexType + Copy> {
    vao: vertex_array_object::VertexArrayObject,
    vbo: vertex_buffer_object::VertexBufferObject,
    data: Vec<T>,
}

impl<T: VertexType + Copy> GeometryObject<T> {

    pub fn new(context: &crate::AppContext, vertices: &[T], storage: vertex_buffer_object::DataStorageType) -> SWGLResult<Self> {
        let mut vbo_data = vec![];
        for v in vertices {
            let mut portion = v.to_vec();
            vbo_data.append(&mut portion);
        }

        let vbo = vertex_buffer_object::VertexBufferObject::new(
            context,
            &vbo_data,
            storage,
        )?;

        let vao = vertex_array_object::VertexArrayObject::new(
            context,
            &vbo,
            T::get_row_width(),
            T::get_vao_signature(),
        )?;

        Ok(GeometryObject { vao, vbo, data: vertices.iter().map(|v| *v).collect() })
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn new_empty_dynamic(context: &crate::AppContext, vertex_count: usize) -> SWGLResult<Self> {
        let vbo = vertex_buffer_object::VertexBufferObject::new_empty(
            context,
            vertex_count * T::get_row_width() as usize,
            vertex_buffer_object::DataStorageType::DynamicDraw,
        )?;

        let vao = vertex_array_object::VertexArrayObject::new(
            context,
            &vbo,
            T::get_row_width(),
            T::get_vao_signature(),
        )?;

        Ok(GeometryObject { vao, vbo, data: Vec::new() })
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn get_vao(&self) -> &vertex_array_object::VertexArrayObject {
        &self.vao
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn get_vbo(&self) -> &vertex_buffer_object::VertexBufferObject {
        &self.vbo
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn borrow_data(&self) -> &[T] {
        &self.data
    }

    pub fn borrow_data_mut(&mut self) -> &mut[T] {
        &mut self.data
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn flush_data(&mut self, context: &crate::AppContext) -> SWGLResult<()> {
        let mut vbo_data = vec![];
        for v in &self.data {
            let mut portion = v.to_vec();
            vbo_data.append(&mut portion);
        }
        self.vbo.update_data_safe(context, &vbo_data, 0)?;
        Ok(())
    }
}
