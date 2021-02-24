use super::super::gl_wrapper::texture::{texture_2d, texture_config};
use super::super::global_tools::vector2::Vector2;

// --------------------------------------------------------------------------------------------------
 
pub struct TextureAtlas {
    pub grid: Vec<texture_config::TextureCoords>,
    pub grid_x: u32,
    pub grid_y: u32,
    pub texture_pixel_width: u32,
    pub texture_pixel_height: u32,
    pub field_pixel_width: u32,
    pub field_pixel_height: u32,
}

// --------------------------------------------------------------------------------------------------

impl TextureAtlas {
    pub fn new(texture: &texture_2d::Texture2D, grid_x: u32, grid_y: u32) -> Self {

        let (texture_pixel_width, texture_pixel_height) = texture.get_size();

        let texture_field_pixel_width = texture_pixel_width / grid_x as f32;
        let texture_field_pixel_heigth = texture_pixel_height / grid_y as f32;

        let mut atlas = vec![];

        for y in 0..grid_y {
            for x in 0..grid_x {
                let current_x = texture_field_pixel_width * x as f32;
                let current_y = texture_field_pixel_heigth * y as f32;
                let end_current_x = current_x + texture_field_pixel_width;
                let end_current_y = current_y + texture_field_pixel_heigth;

                let mut cds = texture_config::TextureCoords {
                    left_top: Vector2::new(
                        current_x / texture_pixel_width,
                        current_y / texture_pixel_height,
                    ),
                    left_bottom: Vector2::new(
                        current_x / texture_pixel_width,
                        end_current_y / texture_pixel_height,
                    ),
                    right_top: Vector2::new(
                        end_current_x / texture_pixel_width,
                        end_current_y / texture_pixel_height,
                    ),
                    right_bottom: Vector2::new(
                        end_current_x / texture_pixel_width,
                        current_y / texture_pixel_height,
                    ),
                };

                cds.reverse();
                atlas.push(cds);
            }
        }

        TextureAtlas {
            grid: atlas,
            grid_x,
            grid_y,
            texture_pixel_width: texture_pixel_width as u32,
            texture_pixel_height: texture_pixel_height as u32,
            field_pixel_width: texture_field_pixel_width as u32,
            field_pixel_height: texture_field_pixel_heigth as u32,
        }
    }
}
