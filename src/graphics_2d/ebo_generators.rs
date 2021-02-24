use super::super::gl_wrapper::element_buffer_object::ElementBufferObject;
use super::super::runtime_error::{SWGLResult};

pub fn get_rectangle_ebo(context: &crate::AppContext, count: usize) -> SWGLResult<ElementBufferObject> {

    // 0 1 2 2 3 1
    // 4 5 6 6 7 5

    let nodes_per_cube = 4;
    let mut indices: Vec<u32> = Vec::with_capacity(nodes_per_cube * count);  
    
    for i in (0..count * nodes_per_cube).step_by(nodes_per_cube) {
        let i = i as u32;
        indices.push(i);
        indices.push(i + 1);
        indices.push(i + 3);
        indices.push(i + 1);
        indices.push(i + 2);
        indices.push(i + 3);
    }

    ElementBufferObject::new(context, indices.as_slice())
}