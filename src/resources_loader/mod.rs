use web_sys::HtmlImageElement;
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use super::utils::image_future::ImageFuture;

use super::runtime_error::{SWGLResult, SWGLRuntimeError};
use super::utils::web_helpers::window;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

// -----------------------------------------------------------------------------------------------------------

async fn get_text_file(url: &str) -> SWGLResult<String> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(url, &opts)
        .ok()
        .ok_or(SWGLRuntimeError::new("ResourceLoader::CannotSendRequest"))?;

    let resp_value = JsFuture::from(window().fetch_with_request(&request))
        .await
        .ok()
        .ok_or(SWGLRuntimeError::new("ResourceLoader::CannotGetResponse"))?;

    let resp: Response = resp_value
        .dyn_into()
        .ok()
        .ok_or(SWGLRuntimeError::new("ResourceLoader::CannotGetResponse"))?;

    if resp.status() != 200 {
        return Err(SWGLRuntimeError::new("ResourceLoader::CannotGetFile"));
    }

    let content = JsFuture::from(resp.text().ok().ok_or(SWGLRuntimeError::new(
        "ResourceLoader::CannotGetResponseContent",
    ))?)
    .await
    .ok()
    .ok_or(SWGLRuntimeError::new(
        "ResourceLoader::CannotGetResponseContent",
    ))?
    .as_string()
    .ok_or(SWGLRuntimeError::new(
        "ResourceLoader::CannotGetResponseContent",
    ))?;

    Ok(content)
}

// -----------------------------------------------------------------------------------------------------------

async fn get_image_file(url: &str) -> SWGLResult<HtmlImageElement> {
    let image = ImageFuture::new(url).await
        .ok().ok_or(SWGLRuntimeError::new("ReosurceLoader::CannotGetImage"))?;
    Ok(image)
}

// -----------------------------------------------------------------------------------------------------------

pub type LoadedContentMap = HashMap<String, LoadedContent>;

#[derive(Debug, Clone)]
pub enum LoadedContent {
    Text(String),
    Image(HtmlImageElement),
}

// -----------------------------------------------------------------------------------------------------------

/// This function returns hashmap based on a given URL collection. 
pub async fn get_files(urls: &[&str]) -> SWGLResult<LoadedContentMap> {
    let mut result = HashMap::<String, LoadedContent>::new();

    for url in urls {
        let ext = Path::new(url)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or(SWGLRuntimeError::new("ResourceLoader::NoExtension"))?;

        if ext == "img" || ext == "jpg" || ext == "bmp" || ext == "png" {
            let file = get_image_file(url).await?;
            result.insert(String::from(*url), LoadedContent::Image(file));
        } else {
            let file = get_text_file(url).await?;
            result.insert(String::from(*url), LoadedContent::Text(file));
        }
    }

    Ok(result)
}

// -----------------------------------------------------------------------------------------------------------

/// This function converts LoadedContent::Text enum to String.
pub fn unwrap_text_content(veriant: &LoadedContent) -> Option<String> {
    if let LoadedContent::Text(data) = veriant {
        return Some(data.clone());
    }
    None
}

// -----------------------------------------------------------------------------------------------------------

/// This function converts LoadedContent::Image enum to HtmlImageElement.
pub fn unwrap_image_content(veriant: &LoadedContent) -> Option<HtmlImageElement> {
    if let LoadedContent::Image(data) = veriant {
        return Some(data.clone());
    }
    None
}
