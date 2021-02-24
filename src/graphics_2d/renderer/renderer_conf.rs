
/// This type may be helpful when you need to use your own shader for a given renderer. 
#[derive(Debug, Clone)]
pub struct RendererConf {
    pub projection_matrix_uniform_name: String,
    pub model_matrix_uniform_name: String,
    pub texture_uniform_prefix: String,
}

impl RendererConf {
    pub fn default() -> Self {
        Self {
            projection_matrix_uniform_name: String::from("projection"),
            model_matrix_uniform_name: String::from("model"),
            texture_uniform_prefix: String::from("tex_"),
        }
    }
}
