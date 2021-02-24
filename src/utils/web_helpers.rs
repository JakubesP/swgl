use super::super::runtime_error::{SWGLResult, SWGLRuntimeError};
use wasm_bindgen::JsCast;
use web_sys::*;

// -----------------------------------------------------------------------------------------

/// This function returns global window object. 
pub fn window() -> web_sys::Window {
    web_sys::window().expect("SWGL Unexpected: Cannot get web_sys::Window")
}

// -----------------------------------------------------------------------------------------

/// This function creates and returns canvas object and GL renderer object based on a given selector. 
pub fn app_handler(canvas_selector: &str) -> SWGLResult<(web_sys::HtmlCanvasElement, crate::AppContext)> {
    let window = window();

    // Get Canvas

    let document = window.document().ok_or(SWGLRuntimeError::new(
        "Cannot get document from std_web::Window",
    ))?;

    let get_canvs_err_msg = format!(
        "Cannot get canvas of id: {} from std_web::Window",
        canvas_selector
    );

    let canvas = document
        .query_selector(canvas_selector)
        .ok()
        .ok_or(SWGLRuntimeError::new(&get_canvs_err_msg))?
        .ok_or(SWGLRuntimeError::new(&get_canvs_err_msg))?;

    // Get GL context

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()
        .ok_or(SWGLRuntimeError::new(
        "Cannot convert canvas to HtmlCanvasElement",
    ))?;

    let get_context_err_msg = "Cannot get webgl2 rendering context";
    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .ok()
        .ok_or(SWGLRuntimeError::new(get_context_err_msg))?
        .ok_or(SWGLRuntimeError::new(get_context_err_msg))?
        .dyn_into()
        .ok()
        .ok_or(SWGLRuntimeError::new(get_context_err_msg))?;

    gl.enable(crate::AppContext::BLEND);
    gl.blend_func(crate::AppContext::SRC_ALPHA, crate::AppContext::ONE_MINUS_SRC_ALPHA);

    Ok((canvas, gl))
}