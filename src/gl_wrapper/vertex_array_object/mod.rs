//! WebGL VAO Object abstraction. 

use web_sys::WebGlVertexArrayObject;
use super::super::runtime_error::{SWGLResult, SWGLRuntimeError};
use super::vertex_buffer_object::VertexBufferObject;
use super::element_buffer_object::ElementBufferObject;

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl PrimitiveType {
    pub fn get_gl_property_value(&self) -> u32 {
        match self {
            Self::Points => crate::AppContext::POINTS,
            Self::Lines => crate::AppContext::LINES,
            Self::LineStrip => crate::AppContext::LINE_STRIP,
            Self::LineLoop => crate::AppContext::LINE_LOOP,
            Self::Triangles => crate::AppContext::TRIANGLES,
            Self::TriangleStrip => crate::AppContext::TRIANGLE_STRIP,
            Self::TriangleFan => crate::AppContext::TRIANGLE_FAN,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct VertexBufferAttribute {
    element_count: u32,
}

impl VertexBufferAttribute {
    pub fn create(element_count: u32) -> VertexBufferAttribute {
        VertexBufferAttribute { element_count }
    }

    pub fn get_element_count(&self) -> u32 {
        self.element_count
    }
}

// -----------------------------------------------------------------------------------------------------------

pub struct VertexArrayObject {
    row_width: u32,
    physical_vertex_count: usize,
    vao: WebGlVertexArrayObject,
    attributes: Vec<VertexBufferAttribute>,
}

impl VertexArrayObject {
    pub fn new(
        context: &crate::AppContext,
        vbo: &VertexBufferObject,
        row_width: u32,
        attributes: Vec<VertexBufferAttribute>,
    ) -> SWGLResult<Self> {

        if let Some(vertex_array) = context.create_vertex_array() {
            let vao = VertexArrayObject {
                vao: vertex_array,
                physical_vertex_count: vbo.get_vertex_count() / row_width as usize,
                row_width,
                attributes,
            };
    
            vao.check_errors()?;
            vao.set_vertex_attributes(vbo, context);
    
            return Ok(vao);
        }
        
        Err(SWGLRuntimeError::new("VertexArrayObject::CreationError"))
    }

    pub fn bind(&self, context: &crate::AppContext) {
        context.bind_vertex_array(Some(&self.vao));
    }

    fn set_vertex_attributes(&self, vbo: &VertexBufferObject, context: &crate::AppContext) {
        self.bind(context);
        vbo.bind(context);

        let mut pos = 0u32;
        let len = self.attributes.len();
        for i in 0..len {
            let attr = &self.attributes[i];
            context.vertex_attrib_pointer_with_i32(
                i as u32,
                attr.get_element_count() as i32,
                crate::AppContext::FLOAT,
                false,
                (self.row_width * std::mem::size_of::<f32>() as u32) as i32,
                pos as i32
            );
            context.enable_vertex_attrib_array(i as u32);
            pos += attr.get_element_count() * std::mem::size_of::<f32>() as u32;
        }
    }

    fn check_errors(&self) -> SWGLResult<()> {
        if self.attributes.len() == 0 {
            return Err(SWGLRuntimeError::new("VertexArrayObject::NoAttributes"));
        }

        if self.get_attributes_byte_size() > self.row_width as usize * std::mem::size_of::<f32>() as usize {
            return Err(SWGLRuntimeError::new("VertexArrayObject::AttributesFitProblem"));
		}

		if self.attributes.len() > 16usize {
            return Err(SWGLRuntimeError::new("VertexArrayObject::AttributesLimit"));
        }
        
        Ok(())
    }

    fn get_attributes_byte_size(&self) -> usize {
        let mut stride_size = 0usize;
        for attr in &self.attributes {
            stride_size += attr.get_element_count() as usize *  std::mem::size_of::<f32>();
        }
        stride_size
    }

    /// This method returns object that can be used directly inside WebGL low level calls.
    pub fn get_raw_id(&self) -> &WebGlVertexArrayObject {
        &self.vao
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn draw_arrays(&self, context: &crate::AppContext, ptype: PrimitiveType, start: u32, n: u32) {
        self.bind(context);
        context.draw_arrays(ptype.get_gl_property_value(), start as i32, n as i32);
    }

    pub fn draw_arrays_safe(&self, context: &crate::AppContext, ptype: PrimitiveType, start: u32, n: u32) -> SWGLResult<()> {
        if start + n > self.physical_vertex_count as u32 {
            return Err(SWGLRuntimeError::new("VertexArrayObject::DrawArraysOverflow"));
        }
        self.draw_arrays(context, ptype, start, n);
        Ok(())
    }

    pub fn draw_all_arrays(&self, context: &crate::AppContext, ptype: PrimitiveType) {
        self.draw_arrays(context, ptype, 0, self.physical_vertex_count as u32);
    }

    // -----------------------------------------------------------------------------------------------------------

    pub fn draw_elements(&self, context: &crate::AppContext, ptype: PrimitiveType, start: u32, n: u32, ebo: &ElementBufferObject) {
        self.bind(context);
        ebo.bind(context);
        context.draw_elements_with_i32(ptype.get_gl_property_value(), n as i32, crate::AppContext::UNSIGNED_INT, start as i32);
    }

    pub fn draw_elements_safe(&self, context: &crate::AppContext, ptype: PrimitiveType, start: u32, n: u32, ebo: &ElementBufferObject) -> SWGLResult<()> {
        if start + n > ebo.get_size() as u32 {
            return Err(SWGLRuntimeError::new("VertexArrayObject::DrawElementsOverflow"));
        }
        self.draw_elements(context, ptype, start, n, ebo);
        Ok(())
    }

    pub fn draw_all_elements(&self, context: &crate::AppContext, ptype: PrimitiveType, ebo: &ElementBufferObject) {
        self.draw_elements(context, ptype, 0, ebo.get_size() as u32, ebo);
    }
}
