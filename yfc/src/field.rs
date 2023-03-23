use std::{fmt::Display, ops::Deref, rc::Rc};

use yew::html::IntoPropValue;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Field {
    value: String,
    valid: bool,
    dirty: bool,
    #[cfg(feature = "validator")]
    message: String,
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
            value: "".into(),
            valid: true,
            dirty: false,
            #[cfg(feature = "validator")]
            message: "".into(),
        }
    }
}

impl Field {
    pub fn new<S: ToString>(s: S) -> Self {
        Field {
            value: s.to_string(),
            ..Default::default()
        }
    }

    /** Returns the current value of the field as an [`Rc<str>`].  */
    pub fn value(&self) -> &str {
        &self.value
    }

    /** Sets the value of the field if the value is different from before and marks the fields as dirty if so. */
    pub fn set_value<S: ToString>(&mut self, value: S) {
        let value = value.to_string();
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

    pub fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    #[cfg(feature = "validator")]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[cfg(feature = "validator")]
    pub fn set_message<S: ToString>(&mut self, message: S) {
        self.message = message.to_string()
    }
}

impl Deref for Field {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntoPropValue<yew::AttrValue> for Field {
    fn into_prop_value(self) -> yew::AttrValue {
        self.value.into()
    }
}

impl IntoPropValue<String> for Field {
    fn into_prop_value(self) -> String {
        self.value
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
