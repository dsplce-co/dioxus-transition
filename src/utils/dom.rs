use gloo::utils::window;
use wasm_bindgen::{JsCast, closure::Closure};

#[macro_export]
macro_rules! animate {
    ($element:expr, $duration:expr) => {
        $element.style().set_property("transition", $duration)?;

        $element
            .style()
            .set_property("transition-timing-function", "ease-in-out")?;
    };
    ($element:expr => using $class:expr => for $duration:expr => forwards) => {
        $element.class_list().add_1($class)?;
        animate!($element, $duration);
    };
    ($element:expr => using $class:expr => for $duration:expr => backwards) => {
        $element.class_list().remove_1($class)?;
        animate!($element, $duration);
    };
}

#[macro_export]
macro_rules! element_by_id {
    ($id:expr) => {{
        use wasm_bindgen::JsCast;

        gloo::utils::document()
            .get_element_by_id($id)
            .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    }};
}

#[macro_export]
macro_rules! wait_for {
    ($ms:expr) => {
        async_std::task::sleep(instant::Duration::from_millis($ms as u64))
    };
}

pub(crate) async fn request_animation_frame() {
    use futures_channel::oneshot;

    let (tx, rx) = oneshot::channel::<()>();
    let mut tx = Some(tx);

    let callback = Closure::wrap(Box::new(move || {
        if let Some(tx) = tx.take() {
            let _ = tx.send(());
        }
    }) as Box<dyn FnMut()>);

    let _ = window()
        .request_animation_frame(callback.as_ref().unchecked_ref());

    callback.forget();

    rx.await.expect("Failed to receive the animation frame")
}
