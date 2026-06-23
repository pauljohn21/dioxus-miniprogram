//! Mutation types for the Mini Program renderer
//!
//! These types represent the operations that can be performed on the DOM.

use dioxus_core::AttributeValue;
use serde::{Deserialize, Serialize};

/// A collection of mutations to apply to the mini program DOM
#[derive(Debug)]
pub struct MiniProgramMutations {
    edits: Vec<MiniProgramEdit>,
}

impl MiniProgramMutations {
    /// Create a new empty mutations collection
    pub fn new() -> Self {
        Self { edits: Vec::new() }
    }

    /// Add an edit to the collection
    pub fn push(&mut self, edit: MiniProgramEdit) {
        self.edits.push(edit);
    }

    /// Take all edits from the collection
    pub fn take_edits(&mut self) -> Vec<MiniProgramEdit> {
        std::mem::take(&mut self.edits)
    }

    /// Check if there are any pending edits
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Get the number of edits
    pub fn len(&self) -> usize {
        self.edits.len()
    }
}

/// A single edit operation to apply to the mini program DOM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MiniProgramEdit {
    /// Insert a new element before another element
    InsertBefore {
        parent_id: u64,
        new_id: u64,
        before_id: Option<u64>,
        tag: String,
        namespace: Option<String>,
    },
    /// Append a new child element
    AppendChild {
        parent_id: u64,
        new_id: u64,
        tag: String,
        namespace: Option<String>,
    },
    /// Remove an element
    Remove { parent_id: u64, child_id: u64 },
    /// Replace an element with a new one
    ReplaceWith {
        old_id: u64,
        new_id: u64,
        tag: String,
        namespace: Option<String>,
    },
    /// Set an attribute on an element
    SetAttribute {
        element_id: u64,
        name: String,
        value: AttributeValue,
        namespace: Option<String>,
    },
    /// Remove an attribute from an element
    RemoveAttribute {
        element_id: u64,
        name: String,
        namespace: Option<String>,
    },
    /// Set the text content of an element
    SetText { element_id: u64, text: String },
    /// Set a marker on an element (for suspense, etc.)
    SetMarker { element_id: u64, marker: String },
}
