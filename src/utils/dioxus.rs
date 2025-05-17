use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use std::ops::Deref;

pub(crate) fn maybe_is_element_placeholder(element: Element) -> Option<bool> {
    let Ok(element) = &element else {
        return None;
    };

    let DynamicNode::Fragment(v_nodes) = &element.dynamic_nodes.deref()[0] else {
        return None;
    };

    let inner: &VNodeInner = v_nodes[0].deref();

    let result = matches!(
        inner.dynamic_nodes.deref().iter().next(),
        Some(DynamicNode::Placeholder(_))
    );

    Some(result)
}
