use yew::prelude::*;

use crate::{form_state::ValueStateMut, Form};

#[derive(Properties, PartialEq, Clone)]
pub struct CheckBoxProps {
    pub form: Form<bool>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
}

#[function_component(CheckBox)]
pub fn check_box(
    CheckBoxProps {
        form,
        classes,
        ontoggle,
    }: &CheckBoxProps,
) -> Html {
    let value = *form.value();

    let ontoggle = {
        let form = form.clone();

        ontoggle.reform(move |_| {
            let value = !value;
            form.state_mut().set(value.to_string());
            value
        })
    };

    html! {
        <input
            // id={form.field_name()}
            class={classes.clone()}
            type="checkbox"
            value={value.to_string()}
            onchange={ontoggle}
            checked={value}
            class={classes.clone()}
         />
    }
}
