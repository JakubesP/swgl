use super::super::super::super::gl_wrapper::vertex_array_object;
use super::super::super::super::global_tools::vector2::Vector2;

use super::super::interface::VertexType;
use super::super::interface::DedicatedShader;
use super::super::interface::TextureCoords;

use super::super::super::shader_collections;

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Default)]
pub struct SingleTexVertex2D {
    pub position: Vector2<f32>,
    pub texture_coord: Vector2<f32>,
    pub z_index: f32,
    pub texture_factor: f32,
}

// -----------------------------------------------------------------------------------------------------------

impl SingleTexVertex2D {  
    pub fn new(position: Vector2<f32>, texture_coord: Vector2<f32>, z_index: f32, texture_factor: f32) -> Self {
        Self {
            position,
            texture_coord,
            z_index,
            texture_factor,
        }
    }

    // This constructor skips position & texture coords params. It can be used for RectangleRenderer. 
    pub fn new_general(z_index: f32, texture_factor: f32) -> Self {
        Self {
            position: Vector2::zero(),
            texture_coord: Vector2::zero(),
            z_index,
            texture_factor,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------

impl VertexType for SingleTexVertex2D {
    fn get_vao_signature() -> Vec<vertex_array_object::VertexBufferAttribute> {
        vec![
            vertex_array_object::VertexBufferAttribute::create(2), // position
            vertex_array_object::VertexBufferAttribute::create(2), // texture coords
            vertex_array_object::VertexBufferAttribute::create(1), // z-index
            vertex_array_object::VertexBufferAttribute::create(1), // texture factor
        ]
    }

    fn get_row_width() -> u32 {
        6
    }

    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.texture_coord.x,
            self.texture_coord.y,
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

impl DedicatedShader for SingleTexVertex2D {
    fn get_dedicated_shader() -> &'static str {
        shader_collections::SIMPLE_TEX_VERTEX2D_SHADER
    }
}

// -----------------------------------------------------------------------------------------------------------

impl TextureCoords for SingleTexVertex2D {
    fn get_tex_coords(&self) -> Vector2<f32> {
        self.texture_coord
    }

    fn set_tex_coords(&mut self, coords: &Vector2<f32>) {
        self.texture_coord = *coords; 
    }
}