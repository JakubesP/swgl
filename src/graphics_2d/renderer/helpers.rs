use super::super::super::gl_wrapper::vertex_array_object::VertexArrayObject;
use super::super::super::gl_wrapper::vertex_buffer_object;
use super::super::super::gl_wrapper::vertex_buffer_object::VertexBufferObject;
use super::super::super::gl_wrapper::element_buffer_object::ElementBufferObject;
use super::super::super::runtime_error::SWGLResult;
use super::super::vertex_2d::interface::VertexType;

pub fn create_buffers<T>(
    context: &crate::AppContext,
    max_vertices_number: usize,
) -> SWGLResult<(VertexBufferObject, VertexArrayObject)>
where
    T: VertexType,
{
    let vbo = VertexBufferObject::new_empty(
        context,
        max_vertices_number * T::get_row_width() as usize,
        vertex_buffer_object::DataStorageType::DynamicDraw,
    )?;

    let vao = VertexArrayObject::new(
        context,
        &vbo,
        T::get_row_width(),
        T::get_vao_signature(),
    )?;

    Ok((vbo, vao))
}

pub fn create_ebo_buffer(context: &crate::AppContext, count: usize) -> SWGLResult<ElementBufferObject> {
    let mut indices = vec![];
    for i in 0..count {
        let i = i as u32;
        indices.push(i * 4 + 0);
        indices.push(i * 4 + 1);
        indices.push(i * 4 + 3);
        indices.push(i * 4 + 3);
        indices.push(i * 4 + 2);
        indices.push(i * 4 + 1);
    }

    ElementBufferObject::new(context, &indices)
} 