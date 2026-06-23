//! DOM operations for the Mini Program renderer

use crate::cfg::Config;
use dioxus_core::{AttributeValue, ElementId, Template};
use std::rc::Rc;

/// The main DOM writer for WeChat Mini Program
pub struct MiniProgramDom {
    config: Config,
    runtime: Rc<dioxus_core::Runtime>,
}

impl MiniProgramDom {
    /// Create a new MiniProgramDom instance
    pub fn new(config: Config, runtime: Rc<dioxus_core::Runtime>) -> Self {
        Self { config, runtime }
    }

    /// Apply edits to the DOM
    fn apply_edits(&self, _edits: &str) {
        // This will be implemented in the future to send edits to the mini program
    }
}

impl dioxus_core::WriteMutations for MiniProgramDom {
    fn create_placeholder(&mut self, _id: ElementId) {}

    fn append_children(&mut self, _id: ElementId, _children: usize) {}

    fn set_attribute(
        &mut self,
        _name: &'static str,
        _namespace: Option<&'static str>,
        _value: &AttributeValue,
        _id: ElementId,
    ) {
    }

    fn push_root(&mut self, _id: ElementId) {}

    fn insert_nodes_before(&mut self, _id: ElementId, _children: usize) {}

    fn remove_node(&mut self, _id: ElementId) {}

    fn replace_node_with(&mut self, _id: ElementId, _count: usize) {}

    fn set_node_text(&mut self, _id: &str, _text: ElementId) {}

    fn assign_node_id(&mut self, _: &'static [u8], _: ElementId) {}

    fn create_text_node(&mut self, _: &str, _: ElementId) {}

    fn load_template(&mut self, _: Template, _: usize, _: ElementId) {}

    fn replace_placeholder_with_nodes(&mut self, _: &'static [u8], _: usize) {}

    fn insert_nodes_after(&mut self, _id: ElementId, _children: usize) {}

    fn create_event_listener(&mut self, _: &'static str, _: ElementId) {}

    fn remove_event_listener(&mut self, _: &'static str, _: ElementId) {}
}
