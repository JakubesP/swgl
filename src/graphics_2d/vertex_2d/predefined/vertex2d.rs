use super::super::super::super::gl_wrapper::vertex_array_object;
use super::super::super::color::Color;
use super::super::super::super::global_tools::vector2::Vector2;

use super::super::interface::VertexType;
use super::super::interface::DedicatedShader;
use super::super::interface::TextureCoords;

use super::super::super::shader_collections;

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
    pub color: Color,
    pub texture_coord: Vector2<f32>,
    pub array_index: f32,
    pub z_index: f32,
    pub texture_factor: f32,
}

// -----------------------------------------------------------------------------------------------------------

impl Vertex2D {  
    pub fn new(
        position: Vector2<f32>,
        color: Color,
        texture_coord: Vector2<f32>,
        array_index: u32,
        z_index: f32,
        texture_factor: f32,
    ) -> Self {
        Self {
            position,
            color,
            texture_coord,
            array_index: array_index as f32,
            z_index,
            texture_factor,
        }
    }

    // This constructor skips position & texture coords params. It can be used for RectangleRenderer. 
    pub fn new_general(
        color: Color,
        array_index: u32,
        z_index: f32,
        texture_factor: f32,
    ) -> Self {
        Self {
            position: Vector2::zero(),
            color,
            texture_coord: Vector2::zero(), 
            array_index: array_index as f32,
            z_index,
            texture_factor,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------

impl VertexType for Vertex2D {
    fn get_vao_signature() -> Vec<vertex_array_object::VertexBufferAttribute> {
        vec![
            vertex_array_object::VertexBufferAttribute::create(2), // position
            vertex_array_object::VertexBufferAttribute::create(4), // color
            vertex_array_object::VertexBufferAttribute::create(2), // texture coords
            vertex_array_object::VertexBufferAttribute::create(1), // array index
            vertex_array_object::VertexBufferAttribute::create(1), // z-index
            vertex_array_object::VertexBufferAttribute::create(1), // texture factor
        ]
    }

    fn get_row_width() -> u32 {
        11
    }

    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.color.red,
            self.color.green,
            self.color.blue,
            self.color.alpha,
            self.texture_coord.x,
            self.texture_coord.y,
            self.array_index,
            self.z_index,
            self.texture_factor,
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

impl DedicatedShader for Vertex2D {
    fn get_dedicated_shader() -> &'static str {
        shader_collections::FULL_VERTEX2D_SHADER
    }
}

// -----------------------------------------------------------------------------------------------------------

impl TextureCoords for Vertex2D {
    fn get_tex_coords(&self) -> Vector2<f32> {
        self.texture_coord
    }

    fn set_tex_coords(&mut self, coords: &Vector2<f32>) {
        self.texture_coord = *coords; 
    }
}