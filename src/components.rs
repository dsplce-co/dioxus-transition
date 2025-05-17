use dioxus::prelude::*;
use wasm_bindgen::JsValue;

use crate::utils;

/// This is a dioxus component similar to Vue's `<Transition>` component.
///
/// It allows you to animate elements when they are added or removed from the DOM.
#[component]
pub fn Transition(
    /// The tree of elements to conduct the animation on.
    children: Element,

    /// The id of the element within the tree to animate.
    id: ReadOnlySignal<String>,

    /// The kind of animation to conduct, you can either use one of the built-in transitions
    /// or create your own.
    ///
    /// Built-in transitions are enabled and injected by default, you can opt out by setting the
    /// `default-features = false` in your `Cargo.toml`.
    kind: ReadOnlySignal<String>,

    /// The duration of the animation in milliseconds.
    duration: u32,

    /// Whether to ignore the fade-in part of the animation on the first run. Useful if the animated
    /// element is present by default.
    #[props(default)]
    ignore_first: bool,
) -> Element {
    #[cfg(feature = "builtins")]
    utils::int::inject_default_stylesheet();

    let mut backup_element: Signal<Option<Element>> = use_signal(|| None);
    let mut slot_element: Signal<Option<Element>> = use_signal(|| None);
    let mut first_run = use_signal(|| true);

    let root_element = use_callback(move |_: ()| {
        let id = id();
        element_by_id!(&id)
    });

    let backup_slot_element = use_callback(move |_: ()| {
        let slot_element = slot_element();
        backup_element.set(slot_element);
    });

    let wipe_backup_element = use_callback(move |_: ()| {
        backup_element.set(None);
    });

    let record_the_run = use_callback(move |_: ()| {
        let is_it_first_run = first_run();

        if !is_it_first_run {
            return;
        }

        first_run.set(false);
    });

    let hidden_class = use_memo(move || format!("{kind}-transition-hidden"));
    let run_class = use_memo(move || format!("{kind}-transition-activating"));
    let duration_ms = use_memo(move || format!("{duration}ms"));

    let mut activate = use_future(move || async move {
        if let Some(element) = root_element(()) {
            let should_be_hidden = !first_run() || !ignore_first;

            if should_be_hidden {
                let hidden_class = hidden_class();
                element.class_list().add_1(&hidden_class)?;
            }
        }

        utils::dom::request_animation_frame().await;

        let Some(element) = root_element(()) else {
            return Ok(());
        };

        let run_class = run_class();
        let duration_ms = duration_ms();

        animate!(element => using &run_class => for &duration_ms => forwards);
        wait_for!(duration).await;

        backup_slot_element(());
        record_the_run(());

        Ok::<(), JsValue>(())
    });

    let mut deactivate = use_future(move || async move {
        utils::dom::request_animation_frame().await;

        let Some(element) = root_element(()) else {
            return Ok(());
        };

        let run_class = run_class();
        let duration_ms = duration_ms();

        animate!(element => using &run_class => for &duration_ms => backwards);
        wait_for!(duration).await;

        wipe_backup_element(());

        Ok::<(), JsValue>(())
    });

    let when_children_change = use_reactive((&children,), move |(children,)| {
        let maybe_is_slot_placeholder = {
            let children = children.clone();
            utils::dioxus::maybe_is_element_placeholder(children)
        };

        let Some(is_slot_placeholder) = maybe_is_slot_placeholder else {
            return;
        };

        if is_slot_placeholder {
            deactivate.restart();
        } else {
            let indeed_children = Some(children.clone());

            slot_element.set(indeed_children);
            activate.restart();
        }
    });

    use_effect(when_children_change);

    rsx! {
        Fragment {
            { backup_element().unwrap_or(children) }
        }
    }
}
