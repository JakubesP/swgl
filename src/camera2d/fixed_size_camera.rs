use super::super::global_tools::vector2::Vector2;
use super::interface::CameraType;
use nalgebra_glm as glm;

// -----------------------------------------------------------------------------------------

/// For this type of camera one unit is one pixel. 
pub struct FixedViewCamera2D {
    pub canvas_size: Vector2<f32>,
    pub focus_position: Vector2<f32>,
    pub zoom: f32,
    pub rotation: f32,
}

// -----------------------------------------------------------------------------------------

impl FixedViewCamera2D {
    pub fn new(focus_position: &Vector2<f32>, canvas_size: &Vector2<f32>) -> Self {
        let mut camera = Self {
            focus_position: Vector2::new(focus_position.x, -focus_position.y),
            canvas_size: *canvas_size,
            zoom: 1.0,
            rotation: 0.0,
        };
        camera.update_canvas_size(canvas_size);
        camera
    }

    pub fn update_canvas_size_if_differ(&mut self, canvas_size: &Vector2<f32>) {
        if self.canvas_size.x != canvas_size.x || self.canvas_size.y != canvas_size.y {
            self.update_canvas_size(canvas_size);
        }
    }

    fn update_canvas_size(&mut self, canvas_size: &Vector2<f32>) {
        self.canvas_size = *canvas_size;
    }

    // -----------------------------------------------------------------------------------------

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        let size_x = self.canvas_size.x;
        let size_y = self.canvas_size.y;
        let left = self.focus_position.x - size_x as f32 / 2.0;
        let right = self.focus_position.x + size_x as f32 / 2.0;
        let top = self.focus_position.y - size_y as f32 / 2.0;
        let bottom = self.focus_position.y + size_y as f32 / 2.0;
        let projection = glm::ortho(left, right, top, bottom, -100.0, 100.0);
        return projection;
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        let mut model = glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        model = glm::scale(&model, &glm::vec3(self.zoom, self.zoom, 0.0));
        model = glm::rotate(&model, self.get_rotation_as_rad(), &glm::vec3(0.0, 0.0, 1.0));
        model
    }

    // -----------------------------------------------------------------------------------------


    pub fn get_left_top_corner_position(&self) -> Vector2<f32> {
        let mut pos = self.focus_position - Vector2::new(
            self.canvas_size.x / 2.0 / self.zoom,
            -self.canvas_size.y / 2.0 / self.zoom
        );
        pos.y = -pos.y;
        pos
    }

    // -----------------------------------------------------------------------------------------

    pub fn set_position(&mut self, point: Vector2<f32>) {
        self.focus_position = point;
    }

    pub fn move_camera(&mut self, point: Vector2<f32>) {
        self.focus_position.x += point.x;
        self.focus_position.y += -point.y;
    }

    pub fn get_position(&self) -> &Vector2<f32> {
        &self.focus_position
    } 

    // -----------------------------------------------------------------------------------------

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom; 
    }

    pub fn change_zoom(&mut self, diff: f32) {
       self.zoom = self.zoom / diff;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    // -----------------------------------------------------------------------------------------

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation; 
    }

    pub fn rotate(&mut self, diff: f32) {
        self.rotation += diff;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_rotation_as_rad(&self) -> f32 {
        self.rotation * 3.14 / 180.0
    }
}

// -----------------------------------------------------------------------------------------

impl CameraType for FixedViewCamera2D {

    fn matrix(&self) -> glm::Mat4 {
        self.get_view_matrix() * self.get_projection_matrix()
    }

    fn dedicated_model(&self) -> glm::Mat4 {
        glm::mat4(
            1.0,  0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0
        )
    }

    fn map_pixel_coords_to_game_coords(&self, coords: &Vector2<f32>) -> Vector2<f32> {
        Vector2::new(
            self.get_left_top_corner_position().x + coords.x / self.get_zoom(),
            self.get_left_top_corner_position().y + coords.y / self.get_zoom()
        )
    }
}
