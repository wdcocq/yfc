use std::{fmt::Display, ops::Deref};

use yew::{html::IntoPropValue, AttrValue};

#[derive(Debug, PartialEq)]
pub struct Field {
    value: AttrValue,
    valid: bool,
    dirty: bool,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            value: Default::default(),
            valid: true,
            dirty: false,
        }
    }
}

impl Field {
    pub fn new<S: Into<AttrValue>>(s: S) -> Self {
        Field {
            value: s.into(),
            ..Default::default()
        }
    }

    pub fn value(&self) -> AttrValue {
        self.value.clone()
    }

    pub fn set_value<S: Into<AttrValue>>(&mut self, value: S) {
        self.value = value.into();
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
}

impl Deref for Field {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntoPropValue<AttrValue> for Field {
    fn into_prop_value(self) -> AttrValue {
        self.value.clone().into()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
