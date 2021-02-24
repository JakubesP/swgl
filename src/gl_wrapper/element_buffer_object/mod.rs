//! WebGL EBO Object abstraction. 

use web_sys::WebGlBuffer;
use super::super::runtime_error::{SWGLResult, SWGLRuntimeError};

// -----------------------------------------------------------------------------------------------------------

pub struct ElementBufferObject {
    ebo: WebGlBuffer,
    size: usize,
}

// -----------------------------------------------------------------------------------------------------------

impl ElementBufferObject {
    pub fn new(context: &crate::AppContext, indices: &[u32]) -> SWGLResult<Self> {
        let buffer;
        let try_buffer = context.create_buffer();

        if let Some(val) = try_buffer {
            buffer = val;
        } else {
            return Err(SWGLRuntimeError::new(
                "ElementBufferObject::CreationError",
            ));
        }

        let ebo = ElementBufferObject {
            ebo: buffer,
            size: indices.len(),
        };
        ebo.bind(context);

        unsafe {
            let vert_array = js_sys::Uint32Array::view(&indices);
            context.buffer_data_with_array_buffer_view(
                crate::AppContext::ELEMENT_ARRAY_BUFFER,
                &vert_array,
                crate::AppContext::STATIC_DRAW,
            );
        }

        Ok(ebo)
    }

    pub fn bind(&self, context: &crate::AppContext) {
        context.bind_buffer(crate::AppContext::ELEMENT_ARRAY_BUFFER, Some(&self.ebo));
    }

    /// This method returns size in elements (not in bytes). 
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// This method returns object that can be used directly inside WebGL low level calls.
    pub fn get_raw_id(&self) -> &WebGlBuffer {
        &self.ebo
    }
}
