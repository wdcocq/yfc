use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{
    html::{ImplicitClone, IntoPropValue},
    prelude::*,
};

use crate::{form::Form, form_state::ValueStateMut, form_value::FormValue};

#[derive(Clone, Copy, PartialEq)]
pub enum Wrap {
    Soft,
    Hard,
}

impl ImplicitClone for Wrap {}

impl IntoPropValue<Option<AttrValue>> for &Wrap {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(match self {
            Wrap::Soft => "soft".into(),
            Wrap::Hard => "hard".into(),
        })
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps<T: FormValue> {
    pub form: Form<T>,
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or(20)]
    pub cols: u32,
    #[prop_or(5)]
    pub rows: u32,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Wrap::Soft)]
    pub wrap: Wrap,
    #[prop_or_default]
    pub spellcheck: bool,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub autocorrect: bool,
}

#[function_component(TextArea)]
pub fn text_area<T: FormValue>(
    TextAreaProps {
        form,
        oninput,
        classes,
        classes_invalid,
        classes_valid,
        cols,
        rows,
        placeholder,
        disabled,
        wrap,
        spellcheck,
        autocomplete,
        autocorrect,
        ..
    }: &TextAreaProps<T>,
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
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                form.state_mut().set(input.value());
            }
            e
        })
    };

    html! {
        <textarea
            // id={form.field_name()}
            // name={form.field_name()}
            class={classes}
            cols={cols.to_string()}
            rows={rows.to_string()}
            {placeholder}
            {wrap}
            spellcheck={spellcheck.to_string()}
            autocomplete={autocomplete.to_string()}
            autocorrect={autocorrect.to_string()}
            {oninput}
            disabled={*disabled}
        />
    }
}
