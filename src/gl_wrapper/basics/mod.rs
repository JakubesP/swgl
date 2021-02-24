
pub fn clear_canvas(context: &crate::AppContext) {
    context.clear(crate::AppContext::COLOR_BUFFER_BIT | crate::AppContext::DEPTH_BUFFER_BIT);
}
