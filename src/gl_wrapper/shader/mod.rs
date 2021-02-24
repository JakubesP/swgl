use super::super::runtime_error::{SWGLResult, SWGLRuntimeError};
use web_sys::{WebGlProgram, WebGlShader, WebGlUniformLocation};

// -----------------------------------------------------------------------------------------------------------
// private:

fn compile_vertex_shader(context: &crate::AppContext, shader: &str) -> SWGLResult<WebGlShader> {
    if let Some(shader_id) = context.create_shader(crate::AppContext::VERTEX_SHADER) {
        context.shader_source(&shader_id, shader);
        context.compile_shader(&shader_id);
        return Ok(shader_id);
    }

    Err(SWGLRuntimeError::new("Program::VertexCreationError"))
}

fn compile_fragment_shader(context: &crate::AppContext, shader: &str) -> SWGLResult<WebGlShader> {
    if let Some(shader_id) = context.create_shader(crate::AppContext::FRAGMENT_SHADER) {
        context.shader_source(&shader_id, shader);
        context.compile_shader(&shader_id);
        return Ok(shader_id);
    }

    Err(SWGLRuntimeError::new("Program::FragmentCreationError"))
}

fn link_program(
    context: &crate::AppContext,
    vertex: &WebGlShader,
    fragment: &WebGlShader,
) -> SWGLResult<WebGlProgram> {
    if let Some(shader_program) = context.create_program() {
        context.attach_shader(&shader_program, &vertex);
        context.attach_shader(&shader_program, &fragment);
        context.link_program(&shader_program);
        return Ok(shader_program);
    }

    Err(SWGLRuntimeError::new("Program::LinkError"))
}

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ShaderSource {
    pub vertex: String,
    pub fragment: String,
}

// -----------------------------------------------------------------------------------------------------------
// private:

#[derive(Copy, Clone, Debug)]
enum ShaderType {
    Vertex,
    Fragment,
    Null,
}

// -----------------------------------------------------------------------------------------------------------

/// You can have both vertex and fragment shader in one file, this function can split it. Vertex Shader content must be after: "#shader vertex" line, and similar Fragment Shader need to be below "#shader fragment".
pub fn split_vfshader_to_shader_source(source: &str) -> ShaderSource {
    let mut stype = ShaderType::Null;

    let mut vertex_code = String::from("");
    let mut fragment_code = String::from("");

    for line in source.lines() {
        let line = format!("{}{}", line, "\n");

        if line.contains("#shader vertex") {
            stype = ShaderType::Vertex;
        } else if line.contains("#shader fragment") {
            stype = ShaderType::Fragment;
        } else {
            match stype {
                ShaderType::Vertex => vertex_code.push_str(&line),
                ShaderType::Fragment => fragment_code.push_str(&line),
                _ => (),
            };
        }
    }

    ShaderSource {
        vertex: vertex_code,
        fragment: fragment_code,
    }
}

// -----------------------------------------------------------------------------------------------------------
// private:

fn create_program(context: &crate::AppContext, source: &ShaderSource) -> SWGLResult<WebGlProgram> {
    let vertex = compile_vertex_shader(context, &source.vertex)?;

    let err_msg = context
        .get_shader_info_log(&vertex)
        .unwrap_or("No info".to_string());
    if err_msg.len() != 0 {
        return Err(SWGLRuntimeError::new(&format!(
            "Program::VertexCompilationError: {}",
            err_msg
        )));
    }

    // ----------------------------------------------------------

    let fragment = compile_fragment_shader(context, &source.fragment)?;

    let err_msg = context
        .get_shader_info_log(&fragment)
        .unwrap_or("No info".to_string());
    if err_msg.len() != 0 {
        return Err(SWGLRuntimeError::new(&format!(
            "Program::FragmentCompilationError: {}",
            err_msg
        )));
    }

    // ----------------------------------------------------------

    let program = link_program(context, &vertex, &fragment)?;

    let err_msg = context
        .get_program_info_log(&program)
        .unwrap_or("No info".to_string());
    if err_msg.len() != 0 {
        return Err(SWGLRuntimeError::new(&format!(
            "Program::LinkError: {}",
            err_msg
        )));
    }

    Ok(program)
}

// -----------------------------------------------------------------------------------------------------------

pub struct Program {
    program: WebGlProgram,
}

impl Program {
    pub fn new(context: &crate::AppContext, source: &ShaderSource) -> SWGLResult<Self> {
        let program = create_program(context, source)?;
        Ok(Program { program })
    }

    /// This method returns object that can be used directly inside WebGL low level calls.
    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn use_program(&self, context: &crate::AppContext) {
        context.use_program(Some(&self.program));
    }

    pub fn unuse_program(&self, context: &crate::AppContext) {
        context.use_program(None);
    }

    fn get_uniform_location(&self, context: &crate::AppContext, name: &str) -> SWGLResult<WebGlUniformLocation> {
        if let Some(location) = context.get_uniform_location(&self.program, name) {
            return Ok(location);
        }
        Err(SWGLRuntimeError::new(&format!(
            "Program::CannotGetUniformLocation: {}",
            name
        )))
    }

    // bind uniform params

    // setnf

    pub fn set1f(&self, context: &crate::AppContext, name: &str, v1: f32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform1f(Some(&self.get_uniform_location(context, name)?), v1);
        Ok(())
    }

    pub fn set2f(&self, context: &crate::AppContext, name: &str, v1: f32, v2: f32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform2f(Some(&self.get_uniform_location(context, name)?), v1, v2);
        Ok(())
    }

    pub fn set3f(&self, context: &crate::AppContext, name: &str, v1: f32, v2: f32, v3: f32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform3f(Some(&self.get_uniform_location(context, name)?), v1, v2, v3);
        Ok(())
    }

    pub fn set4f(
        &self,
        context: &crate::AppContext,
        name: &str,
        v1: f32,
        v2: f32,
        v3: f32,
        v4: f32,
    ) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform4f(
            Some(&self.get_uniform_location(context, name)?),
            v1,
            v2,
            v3,
            v4,
        );
        Ok(())
    }

    // setni

    pub fn set1i(&self, context: &crate::AppContext, name: &str, v1: i32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform1i(Some(&self.get_uniform_location(context, name)?), v1);
        Ok(())
    }

    pub fn set2i(&self, context: &crate::AppContext, name: &str, v1: i32, v2: i32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform2i(Some(&self.get_uniform_location(context, name)?), v1, v2);
        Ok(())
    }

    pub fn set3i(&self, context: &crate::AppContext, name: &str, v1: i32, v2: i32, v3: i32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform3i(Some(&self.get_uniform_location(context, name)?), v1, v2, v3);
        Ok(())
    }

    pub fn set4i(
        &self,
        context: &crate::AppContext,
        name: &str,
        v1: i32,
        v2: i32,
        v3: i32,
        v4: i32,
    ) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform4i(
            Some(&self.get_uniform_location(context, name)?),
            v1,
            v2,
            v3,
            v4,
        );
        Ok(())
    }

    // setnu

    pub fn set1u(&self, context: &crate::AppContext, name: &str, v1: u32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform1ui(Some(&self.get_uniform_location(context, name)?), v1);
        Ok(())
    }

    pub fn set2u(&self, context: &crate::AppContext, name: &str, v1: u32, v2: u32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform2ui(Some(&self.get_uniform_location(context, name)?), v1, v2);
        Ok(())
    }

    pub fn set3u(&self, context: &crate::AppContext, name: &str, v1: u32, v2: u32, v3: u32) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform3ui(Some(&self.get_uniform_location(context, name)?), v1, v2, v3);
        Ok(())
    }

    pub fn set4u(
        &self,
        context: &crate::AppContext,
        name: &str,
        v1: u32,
        v2: u32,
        v3: u32,
        v4: u32,
    ) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform4ui(
            Some(&self.get_uniform_location(context, name)?),
            v1,
            v2,
            v3,
            v4,
        );
        Ok(())
    }

    // setmat

    pub fn set_mat_4x4f(&self, context: &crate::AppContext, name: &str, v: &[f32]) -> SWGLResult<()> {
        self.use_program(context);
        context.uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
            Some(&self.get_uniform_location(context, name)?),
            false,
            v,
            0,
            v.len() as u32,
        );
        Ok(())
    }
}
