use yew::prelude::*;

use crate::{form_state::ValueStateMut, Form};

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    pub form: Form<bool>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
}

#[function_component(Checkbox)]
pub fn checkbox(
    CheckboxProps {
        form,
        classes,
        ontoggle,
    }: &CheckboxProps,
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

    #[cfg(feature = "ybc")]
    {
        html!(
            <ybc::Checkbox name="" checked={value} update={ontoggle}/>
        )
    }

    #[cfg(not(feature = "ybc"))]
    {
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
}
