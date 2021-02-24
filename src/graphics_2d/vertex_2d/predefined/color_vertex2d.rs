use super::super::super::super::gl_wrapper::vertex_array_object;
use super::super::super::color::Color;
use super::super::super::super::global_tools::vector2::Vector2;

use super::super::interface::VertexType;
use super::super::interface::DedicatedShader;
use super::super::super::shader_collections;

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Default)]
pub struct ColorVertex2D {
    pub position: Vector2<f32>,
    pub color: Color,
    pub z_index: f32,
}

// -----------------------------------------------------------------------------------------------------------

impl ColorVertex2D {  
    pub fn new(position: Vector2<f32>, color: Color, z_index: f32) -> Self {
        Self {
            position,
            color,
            z_index, 
        }
    }

    // This constructor skips position & texture coords params. It can be used for RectangleRenderer. 
    pub fn new_general(color: Color, z_index: f32) -> Self {
        Self {
            position: Vector2::zero(),
            color,
            z_index, 
        }
    }
}

// -----------------------------------------------------------------------------------------------------------

impl VertexType for ColorVertex2D {
    fn get_vao_signature() -> Vec<vertex_array_object::VertexBufferAttribute> {
        vec![
            vertex_array_object::VertexBufferAttribute::create(2), // position
            vertex_array_object::VertexBufferAttribute::create(4), // color
            vertex_array_object::VertexBufferAttribute::create(1), // z-index
        ]
    }

    fn get_row_width() -> u32 {
        7
    }

    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha,
            self.z_index,
        ]
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn set_position(&mut self, position: &Vector2<f32>) {
        self.position = *position; 
    }
}

// -----------------------------------------------------------------------------------------------------------

impl DedicatedShader for ColorVertex2D {
    fn get_dedicated_shader() -> &'static str {
        shader_collections::COLOR_VERTEX2D_SHADER
    }
}