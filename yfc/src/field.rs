use std::{fmt::Display, ops::Deref};

use yew::{html::IntoPropValue, AttrValue};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Field {
    value: AttrValue,
    valid: bool,
    dirty: bool,
    #[cfg(feature = "validate")]
    message: AttrValue,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Field(\"{}\", d: {}, v: {})",
            self.value, self.dirty, self.valid
        )
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            value: Default::default(),
            valid: true,
            dirty: false,
            #[cfg(feature = "validate")]
            message: Default::default(),
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

    /** Returns the current value of the field as an [`AttrValue`].  */
    pub fn value(&self) -> AttrValue {
        self.value.clone()
    }

    /** Sets the value of the field if the value is different from before and marks the fields as dirty if so. */
    pub fn set_value<S: Into<AttrValue>>(&mut self, value: S) {
        let value = value.into();
        if value != self.value {
            self.value = value;
            self.dirty = true;
        }
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    #[cfg(feature = "validate")]
    pub fn message(&self) -> AttrValue {
        self.message.clone()
    }

    #[cfg(feature = "validate")]
    pub fn set_message<S: Into<AttrValue>>(&mut self, message: S) {
        self.message = message.into()
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
