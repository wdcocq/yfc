use std::{cell::RefMut, ops::Deref};

use yew::AttrValue;

use crate::{
    field::Field,
    form_state::{OptionStateMut, StateMut, StateProvider, ValueStateMut},
};

pub trait FormValue
where
    Self: PartialEq,
    for<'a> Self:
        StateProvider<State = Field, StateMut<'a> = <Self as FormValue>::StateMut<'a>> + 'a,
{
    type StateMut<'a>: ValueStateMut<'a, Self>
    where
        Self: 'a;

    fn value(&self) -> String;
    fn from_value(value: &str) -> Self;
}

pub struct FormValueState<'a, T>
where
    T: StateProvider,
{
    pub(crate) value: RefMut<'a, T>,
    pub(crate) field: RefMut<'a, T::State>,
}

impl<'a, T> Deref for FormValueState<'a, T>
where
    T: StateProvider<State = Field>,
{
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        self.field.deref()
    }
}

impl<'a, T> StateMut<'a, T> for FormValueState<'a, T>
where
    T: StateProvider<State = Field>,
{
    fn split(self) -> (RefMut<'a, T>, RefMut<'a, T::State>) {
        (self.value, self.field)
    }
}

impl<'a, T> ValueStateMut<'a, T> for FormValueState<'a, T>
where
    T: FormValue,
{
    fn set<S: Into<AttrValue>>(&mut self, value: S) {
        let value = value.into();
        *self.value = FormValue::from_value(&value);
        self.field.set_value(value);
    }
}

macro_rules! impl_form_value {
    ($($t:ty),*) => {
        $(
            impl FormValue for $t {
                type StateMut<'a> = FormValueState<'a, $t>;

                fn value(&self) -> String {
                    self.to_string()
                }

                fn from_value(value: &str) -> Self {
                    value.parse().unwrap()
                }
            }
        )*
    };
}

impl_form_value!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, String
);

impl<T> FormValue for Option<T>
where
    T: FormValue + Default + 'static,
    <T as StateProvider>::State: Default,
    for<'a> <T as StateProvider>::StateMut<'a>: ValueStateMut<'a, T>,
{
    type StateMut<'a> = OptionStateMut<'a, T>;
    fn value(&self) -> String {
        self.as_ref().map(FormValue::value).unwrap_or_default()
    }

    fn from_value(value: &str) -> Self {
        if value.is_empty() {
            None
        } else {
            Some(FormValue::from_value(value))
        }
    }
}
