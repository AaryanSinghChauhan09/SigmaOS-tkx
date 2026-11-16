// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-tkx: tkx component model.
//!
//! Real implementation: tkx_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TkxItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TkxError {
    EmptyTkxItemName,
    DuplicateTkxItem(String),
    EmptyEventSubject,
}

impl fmt::Display for TkxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTkxItemName => write!(f, "tkx_item name must not be empty"),
            Self::DuplicateTkxItem(id) => write!(f, "duplicate tkx_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TkxError {}

pub struct Tkx {
    pub name: String,
    tkx_items: BTreeMap<String, TkxItem>,
    events: BTreeMap<String, Event>,
}

impl Tkx {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tkx_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_tkx_item(&mut self, item: TkxItem) -> Result<(), TkxError> {
        if item.name.trim().is_empty() {
            return Err(TkxError::EmptyTkxItemName);
        }
        let id = item.id.clone();
        if self.tkx_items.contains_key(&id) {
            return Err(TkxError::DuplicateTkxItem(id.clone()));
        }
        self.tkx_items.insert(id, item);
        Ok(())
    }

    pub fn tkx_item_count(&self) -> usize {
        self.tkx_items.len()
    }

    pub fn get_tkx_item(&self, id: &str) -> Option<&TkxItem> {
        self.tkx_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TkxError> {
        if event.subject.trim().is_empty() {
            return Err(TkxError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tkx_item(id: &str, name: &str) -> TkxItem {
        TkxItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Tkx::new("Tkx");
        assert_eq!(m.name, "Tkx");
        assert_eq!(m.tkx_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_tkx_item() {
        let mut m = Tkx::new("Tkx");
        m.add_tkx_item(sample_tkx_item("a1", "Alpha")).unwrap();
        assert_eq!(m.tkx_item_count(), 1);
        assert!(m.get_tkx_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Tkx::new("Tkx");
        assert!(m.add_tkx_item(sample_tkx_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Tkx::new("Tkx");
        m.add_tkx_item(sample_tkx_item("x", "One")).unwrap();
        assert!(m.add_tkx_item(sample_tkx_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Tkx::new("Tkx");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
