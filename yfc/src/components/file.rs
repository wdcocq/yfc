use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

use crate::{form::Form, form_state::ValueStateMut};

pub enum FileMessage {
    OnInput(InputEvent),
}

#[derive(Properties, PartialEq, Clone)]
pub struct FilePropeties {
    pub form: Form<String>,
    pub field_name: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub capture: AttrValue,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| "is-invalid".into())]
    pub classes_invalid: Classes,
    #[prop_or_else(|| "is-valid".into())]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(File)]
pub fn file(
    FilePropeties {
        form,
        field_name,
        disabled,
        multiple,
        accept,
        capture,
        classes,
        classes_valid,
        classes_invalid,
        oninput,
        ..
    }: &FilePropeties,
) -> Html {
    let classes = classes!(
        classes.clone(),
        form.state().dirty().then(|| match form.state().valid() {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );
    let oninput = oninput.reform({
        let form = form.clone();

        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                form.state_mut().set(input.value());
            }

            e
        }
    });

    html! {
        <input
            id={field_name}
            type="file"
            name={field_name}
            {accept}
            disabled={*disabled}
            multiple={*multiple}
            class={classes}
            {oninput}
            {capture}
        />
    }
}
