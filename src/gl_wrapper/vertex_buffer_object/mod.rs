//! WebGL VBO Object abstraction. 

use web_sys::WebGlBuffer;
use super::super::runtime_error::{SWGLResult, SWGLRuntimeError};

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum DataStorageType {
    StaticDraw,
    DynamicDraw,
    StreamDraw,
}

impl DataStorageType {
    pub fn get_gl_property_value(&self) -> u32 {
        match self {
            Self::StaticDraw => crate::AppContext::STATIC_DRAW,
            Self::DynamicDraw => crate::AppContext::DYNAMIC_DRAW,
            Self::StreamDraw => crate::AppContext::STREAM_DRAW,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------

pub struct VertexBufferObject {
    vbo: WebGlBuffer,
    vertex_count: usize,
    #[allow(dead_code)]
    data_storage_type: DataStorageType,
}

// -----------------------------------------------------------------------------------------------------------

impl VertexBufferObject {
    pub fn new(
        context: &crate::AppContext,
        data: &[f32],
        data_storage_type: DataStorageType,
    ) -> SWGLResult<VertexBufferObject> {
        unsafe {
            if let Some(buffer) = context.create_buffer() {
                let vbo = VertexBufferObject {
                    vbo: buffer,
                    vertex_count: data.len(),
                    data_storage_type,
                };
                vbo.bind(context);

                let buffer_data = js_sys::Float32Array::view(&data);
                context.buffer_data_with_array_buffer_view(
                    crate::AppContext::ARRAY_BUFFER,
                    &buffer_data,
                    data_storage_type.get_gl_property_value(),
                );
                return Ok(vbo);
            }
        }

        Err(SWGLRuntimeError::new("VertexBufferObject::CreationError"))
    }

    pub fn new_empty(
        context: &crate::AppContext,
        vertex_count: usize,
        data_storage_type: DataStorageType,
    ) -> SWGLResult<VertexBufferObject> {
        if let Some(buffer) = context.create_buffer() {
            let vbo = VertexBufferObject {
                vbo: buffer,
                vertex_count,
                data_storage_type,
            };
            vbo.bind(context);
            // i am not sure
            context.buffer_data_with_f64(
                crate::AppContext::ARRAY_BUFFER,
                vertex_count as f64 * std::mem::size_of::<f32>() as f64,
                data_storage_type.get_gl_property_value(),
            );

            return Ok(vbo);
        }

        Err(SWGLRuntimeError::new("VertexBufferObject::CreationError"))
    }

    pub fn bind(&self, context: &crate::AppContext) {
        context.bind_buffer(crate::AppContext::ARRAY_BUFFER, Some(&self.vbo));
    }

    pub fn get_bytes_count(&self) -> usize {
        self.vertex_count * std::mem::size_of::<f32>()
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// This method gets offset parameter, that reffers to n-element (float item), not to n-byte. 
    pub fn update_data(&self, context: &crate::AppContext, data: &[f32], offset: u32) {
        self.bind(context);

        unsafe {
            let buffer_data = js_sys::Float32Array::view(&data);
            context.buffer_sub_data_with_f64_and_array_buffer_view(
                crate::AppContext::ARRAY_BUFFER,
                offset as f64 * std::mem::size_of::<f32>() as f64,
                &buffer_data,
            );
        }
    }

    pub fn update_data_safe(&self, context: &crate::AppContext, data: &[f32], offset: u32) -> SWGLResult<()> {
        if offset + data.len() as u32 > self.get_vertex_count() as u32 {
            return Err(SWGLRuntimeError::new(
                "VertexBufferObject::UpdateDataOverflow",
            ));
        }
        self.update_data(context, data, offset);
        Ok(())
    }

    /// This method returns object that can be used directly inside WebGL low level calls.
    pub fn get_raw_id(&self) -> &WebGlBuffer {
        &self.vbo
    }
}
