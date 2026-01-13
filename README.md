> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# dioxus-transition

> 🎬 Seamless transitions in [Dioxus](https://dioxuslabs.com/), inspired by Vue’s `<Transition>` — declarative, built-in, composable.

This crate provides a fully reactive `<Transition>` component for **Dioxus**, allowing you to animate elements when they enter or leave the DOM.

---

## 🖤 Features

✅ `Transition` component for conditional DOM animations<br/>
✅ Vue-style logic with signal-reactive updates<br/>
✅ Built-in transitions: `fade`, `blur` — easy to extend<br/>
✅ Opt-out of built-ins via `default-features = false`<br/>
✅ Built for flexibility: own styles, fine-grained control<br/>

---

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
dioxus-transition = { version = "0.3" }
```
or:

```toml
[dependencies]
dioxus-transition = { version = "0.3", default-features = false }
```

to disable the default stylesheet (opt out of default transition kinds).

The latest version of this crate requires Dioxus version 0.7. Check the [compatibility table](#compatibility-with-dioxus-versions) for other supported Dioxus versions.

### Fullstack applications

You need to enable the crate's SSR feature on the server for fullstack apps:

```toml
[features]
server = ["dioxus/server", "dioxus-transition/ssr"]
```

This will tell `dioxus-transition` not to perform DOM-related operations at the stage of server side rendering.

---

## ⚡ Example

```rust
use dioxus::prelude::*;
use dioxus_transition::prelude::*;

fn main() {
  dioxus::web::launch(App);
}

#[component]
fn App() -> Element {
    let mut visible = use_signal(bool::default);

    rsx! {
        div {
            button {
                onclick: move |_| visible.set(!visible()),
                "Toggle"
            }

            Transition {
                id: "square",
                kind: "fade", // try "blur", or define your own
                duration: 300,

                if visible() {
                    div {
                        id: "square",
                        display: "block",
                        width: "200px",
                        height: "200px",
                        background: "red",
                    }
                }
            }
        }
    }
}
```

---

## 🧠 How It Works

The `<Transition>` component:

* Tracks whether its children are present or a placeholder (`<!--placeholder-->`)
* On entrance: injects `*-transition-hidden`, then animates in with `*-transition-activating`
* On exit: runs reverse animation and cleans up
* You control:

    * `id`: DOM node to animate
    * `kind`: animation class prefix (e.g. `fade`)
    * `duration`: in ms
    * `ignore_first`: skip entrance animation on first mount (default: false)

---

## 🎨 Built-In Styles (enabled by default)

```css
/* fade */
.fade-transition-hidden {
  opacity: 0;
}

.fade-transition-activating {
  opacity: 1;
}

/* blur */
.blur-transition-hidden {
  backdrop-filter: brightness(1) blur(0);
}

.blur-transition-activating {
  backdrop-filter: brightness(0.375) blur(2px);
}
```

Don’t like them? Set `default-features = false` and roll your own 🧘

---

## 🧩 Custom Transitions

Use any CSS class name as `kind`. All that matters is you provide these two classes:

* `.<kind>-transition-hidden` — hidden state
* `.<kind>-transition-activating` — visible state

---

## Compatibility with Dioxus versions

| Dioxus version | `dioxus-transition` version |
|:---------------|:----------------------------|
| `0.7`          | `0.3`                       |
| `0.6`          | `0.2`                       |

---

## 📁 Repo & Contributions

📦 Crate: [crates.io/crates/dioxus-transition](https://crates.io/crates/dioxus-transition)<br/>
🛠️ Repo: [github.com/dsplce-co/dioxus-transition](https://github.com/dsplce-co/dioxus-transition)<br/>

Contributions, issues, ideas? Hit us up — let's make transitions in Dioxus delightful 🖤

---

## 📄 License

MIT or Apache-2.0, at your option.
