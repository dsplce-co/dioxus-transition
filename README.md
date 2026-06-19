> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# dioxus-transition

[![Dioxus](https://img.shields.io/badge/Dioxus-000000?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com/)
[![crates.io downloads](https://img.shields.io/crates/d/dioxus-transition?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/dioxus-transition)
[![crates.io size](https://img.shields.io/crates/size/dioxus-transition?style=for-the-badge)](https://crates.io/crates/dioxus-transition)
[![License](https://img.shields.io/crates/l/dioxus-transition.svg?style=for-the-badge)](https://crates.io/crates/dioxus-transition)
[![crates.io](https://img.shields.io/crates/v/dioxus-transition?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/dioxus-transition)

🎬 Seamless enter/leave transitions for [Dioxus](https://dioxuslabs.com/), inspired by Vue's `<Transition>` — declarative, built-in, composable.

`dioxus-transition` gives you a fully reactive `<Transition>` component: wrap the thing you're conditionally rendering and it animates *in* when it enters the DOM and *out* when it leaves, instead of just blinking into (and out of) existence.

## 🖤 Features

- **Drop-in `<Transition>`** — Wrap a conditionally-rendered element and it eases in and out — no more elements popping into existence like nothing happened
- **Vue's mental model, Dioxus' reactivity** — If you've used Vue's `<Transition>` you already know the shape of this; under the hood it's signals all the way down
- **Batteries included** — `fade` and `blur` ship in the box; don't like them? `default-features = false` and roll your own 🧘
- **A transition is just two CSS classes** — Pick any `kind`, provide a `-hidden` and an `-activating` class, and you're off — no DSL to learn
- **SSR-aware** — Flip on the `ssr` feature for fullstack apps and it won't touch the DOM during server-side rendering

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
  - [cargo](#cargo)
  - [Fullstack apps](#fullstack-apps)
- [🧪 Usage](#-usage)
  - [Quick example](#quick-example)
  - [How it works](#how-it-works)
  - [Props](#props)
  - [Built-in styles](#built-in-styles)
  - [Custom transitions](#custom-transitions)
- [🛠️ Compatibility](#%EF%B8%8F-compatibility)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

### cargo

Add it with `cargo add`:

```bash
cargo add dioxus-transition
```

or drop it into your `Cargo.toml` by hand:

```toml
[dependencies]
dioxus-transition = "0.3"
```

The built-in `fade` / `blur` stylesheet is injected by default. If you'd rather bring your own styles and skip the built-ins entirely, opt out:

```toml
[dependencies]
dioxus-transition = { version = "0.3", default-features = false }
```

The latest version targets **Dioxus 0.7** — see the [compatibility table](#%EF%B8%8F-compatibility) for the version mapping.

### Fullstack apps

For fullstack (SSR) apps, enable the crate's `ssr` feature on the server so it knows not to perform DOM operations while server-side rendering:

```toml
[features]
server = ["dioxus/server", "dioxus-transition/ssr"]
```

⸻

## 🧪 Usage

### Quick example

Toggle a signal, wrap the conditional element in `<Transition>`, and let it animate:

```rust
use dioxus::prelude::*;
use dioxus_transition::prelude::*;

fn main() {
    dioxus::launch(App);
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

The one thing worth calling out: the `id` you give `<Transition>` has to match the `id` of the element you're actually animating (`"square"` in both places above) — that's the node it reaches for in the DOM.

### How it works

The `<Transition>` component watches whether its children are really there or have collapsed to a placeholder (`<!--placeholder-->`), and drives the animation off that:

- **On entrance** it injects `*-transition-hidden`, then on the next animation frame swaps in `*-transition-activating` to animate in
- **On exit** it runs the same animation in reverse and cleans up once the duration's elapsed

Two classes do all the work — `<kind>-transition-hidden` for the resting/hidden state and `<kind>-transition-activating` for the visible one — so a "transition" is really just a pair of CSS classes you control.

### Props

| Prop | Type | Required | What it does |
|:-----|:-----|:---------|:-------------|
| `id` | `String` | Yes | The `id` of the element within the tree to animate (must match the rendered node) |
| `kind` | `String` | Yes | The animation class prefix — a built-in (`fade`, `blur`) or your own |
| `duration` | `u32` | Yes | Animation duration, in milliseconds |
| `ignore_first` | `bool` | No | Skip the entrance animation on first mount (default `false`). Handy when the element is present by default and you don't want it animating in on load |

### Built-in styles

Enabled by default (`builtins` feature) and injected for you:

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

Don't like them? Set `default-features = false` and roll your own 🧘

### Custom transitions

A `kind` is just a CSS class prefix, so use whatever name you like — all that matters is you provide these two classes:

- `.<kind>-transition-hidden` — the hidden state
- `.<kind>-transition-activating` — the visible state

That's the whole contract. Define `slide`, `zoom`, `whatever` and pass it as `kind`.

⸻

## 🛠️ Compatibility

| Dioxus version | `dioxus-transition` version |
|:---------------|:----------------------------|
| `0.7`          | `0.3`                       |
| `0.6`          | `0.2`                       |

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/dioxus-transition](https://github.com/dsplce-co/dioxus-transition)<br>
📦 **Crate**: [https://crates.io/crates/dioxus-transition](https://crates.io/crates/dioxus-transition)

Contributions, issues, ideas? Hit us up — let's make transitions in Dioxus delightful 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.
