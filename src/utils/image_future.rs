use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;
use wasm_bindgen::JsCast;

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: std::rc::Rc<std::cell::Cell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: std::rc::Rc::new(std::cell::Cell::new(false)),
        }
    }
}

impl std::future::Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let image = self.image.take().unwrap();
                let failed = self.load_failed.get();

                if failed {
                    std::task::Poll::Ready(Err(()))
                } else {
                    std::task::Poll::Ready(Ok(image))
                }
            }
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                std::task::Poll::Pending
            }
            _ => std::task::Poll::Ready(Err(())),
        }
    }
}
