#[cfg(feature = "builtins")]
pub(crate) use builtins::inject_default_stylesheet;

#[cfg(feature = "builtins")]
mod builtins {
    use gloo::utils::document;

    static STYLESHEET: &str = concat!(
        // language=css
        r#"
        .fade-transition-hidden {
            opacity: 0;
        }
        .fade-transition-activating {
            opacity: 1;
        }
        "#,
        // language=css
        r#"
        .blur-transition-hidden {
            backdrop-filter: brightness(1) blur(0);
        }

        .blur-transition-activating {
            backdrop-filter: brightness(0.375) blur(2px);
        }
        "#
    );

    pub(crate) fn inject_default_stylesheet() {
        if document()
            .get_element_by_id(env!("CARGO_PKG_NAME"))
            .is_some()
        {
            return;
        }

        dioxus_document::document().create_head_element(
            "style",
            &[
                ("id", String::from(env!("CARGO_PKG_NAME"))),
                ("type", "text/css".to_string()),
            ],
            Some(STYLESHEET.to_string()),
        );
    }
}
