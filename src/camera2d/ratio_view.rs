use super::super::global_tools::vector2::Vector2;
use super::interface::CameraType;
use nalgebra_glm as glm;

// -----------------------------------------------------------------------------------------

/// For this type of camera you can specify your own responsive unit size (for width & height size will be the same).
pub struct RatioView {
    pub canvas_size: Vector2<f32>,
    pub scene_relative_size: f32,
}

// -----------------------------------------------------------------------------------------

impl RatioView {
    pub fn new(canvas_size: Vector2<f32>, scene_relative_size: f32) -> Self {
        Self {
            canvas_size,
            scene_relative_size,
        }
    }

    pub fn update_canvas_size(&mut self, canvas_size: Vector2<f32>) {
        self.canvas_size = canvas_size;
    }

    pub fn get_relative_size(&self) -> f32 {
        self.scene_relative_size
    }
}

// -----------------------------------------------------------------------------------------

impl CameraType for RatioView {
    fn matrix(&self) -> glm::Mat4 {
        let size: f32 = self.scene_relative_size;

        let aspect: f32 = if self.canvas_size.x > self.canvas_size.y {
            self.canvas_size.x as f32 / self.canvas_size.y as f32
        } else {
            self.canvas_size.y as f32 / self.canvas_size.x as f32
        };

        let projection = if self.canvas_size.x > self.canvas_size.y {
            glm::ortho(
                -aspect * size / 2.0,
                aspect * size / 2.0,
                -size / 2.0,
                size / 2.0,
                -1.0,
                1.0,
            )
        } else {
            glm::ortho(
                -size / 2.0,
                size / 2.0,
                -aspect * size / 2.0,
                aspect * size / 2.0,
                -1.0,
                1.0,
            )
        };
        return projection;
    }

    fn dedicated_model(&self) -> glm::Mat4 {
        let rel2div = self.scene_relative_size / 2.0;
        glm::mat4(
            1.0,  0.0, 0.0, -rel2div,
            0.0, -1.0, 0.0,  rel2div,
            0.0,  0.0, 1.0,  0.0,
            0.0,  0.0, 0.0,  1.0,
        )
    }

    fn map_pixel_coords_to_game_coords(&self, coords: &Vector2<f32>) -> Vector2<f32> {
        let w_peer_h = self.canvas_size.x as f32 / self.canvas_size.y as f32;
        let h_peer_w = self.canvas_size.y as f32 / self.canvas_size.x as f32;
        let rel2div = self.scene_relative_size / 2.0;

        if self.canvas_size.x > self.canvas_size.y {
            Vector2::new(
                (coords.x / self.canvas_size.x * self.scene_relative_size - rel2div) * w_peer_h
                    + rel2div,
                coords.y / self.canvas_size.y * self.scene_relative_size,
            )
        } else {
            Vector2::new(
                coords.x / self.canvas_size.x * self.scene_relative_size,
                (coords.y / self.canvas_size.y * self.scene_relative_size - rel2div) * h_peer_w
                    + rel2div,
            )
        }
    }
}
