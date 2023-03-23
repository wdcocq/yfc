use strum::IntoStaticStr;
use web_sys::{HtmlInputElement, InputEvent};
#[cfg(feature = "ybc")]
use ybc;
use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

use crate::{form::Form, form_state::ValueStateMut, form_value::FormValue};

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum InputType {
    Text,
    Password,
    Email,
    Tel,
    Url,
    Date,
}

impl ImplicitClone for InputType {}

impl IntoPropValue<Option<AttrValue>> for InputType {
    fn into_prop_value(self) -> Option<AttrValue> {
        <AttrValue as From<&'static str>>::from(self.into()).into()
    }
}

#[cfg(feature = "ybc")]
impl InputType {
    fn ybc_type(&self) -> Option<ybc::InputType> {
        match self {
            InputType::Text => Some(ybc::InputType::Text),
            InputType::Password => Some(ybc::InputType::Password),
            InputType::Email => Some(ybc::InputType::Email),
            InputType::Tel => Some(ybc::InputType::Tel),
            _ => None,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T: FormValue> {
    pub form: Form<T>,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub classes: Classes,
    /// Classes that are applied when the field is dirt and invalid
    #[prop_or_default]
    pub classes_invalid: Classes,
    /// Classes that are applied when the field is dirty and valid
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(Input)]
pub fn input<T: FormValue + 'static>(
    InputProps {
        autocomplete,
        input_type,
        form,
        placeholder,
        disabled,
        classes,
        classes_invalid,
        classes_valid,
        oninput,
    }: &InputProps<T>,
) -> Html {
    let classes = classes!(
        classes.clone(),
        form.state().dirty().then(|| match form.state().valid() {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );

    let oninput = {
        let form = form.clone();

        oninput.reform(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                form.state_mut().set(input.value());
            }
            e
        })
    };

    // If a valid bulma/ybc input class return early with ybc element
    #[cfg(feature = "ybc")]
    if let Some(input_type) = input_type.ybc_type() {
        return html! {
            <ybc::Input
                name="input"
                {classes}
                r#type={input_type}
                autocomplete={*autocomplete}
                {placeholder}
                value={form.state().value().to_owned()}
                update={oninput}
                disabled={*disabled}
            />
        };
    }

    // If not one of the valid bulma/ybc input classes, still add the 'input' class so it's styled properly.
    #[cfg(feature = "ybc")]
    let classes = classes!(classes, "input");

    let autocomplete = if *autocomplete { "on" } else { "off" };

    html! {
        <input
            // id={form.field_name()}
            class={classes}
            type={*input_type}
            {autocomplete}
            {placeholder}
            value={form.state().value().to_owned()}
            {oninput}
            disabled={*disabled}
        />
    }
}
