use std::rc::Rc;

use web_sys::HtmlSelectElement;
#[cfg(feature = "ybc")]
use ybc;
use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VChild};

use crate::{form::Form, form_state::ValueStateMut, form_value::FormValue};

#[derive(Clone, PartialEq)]
pub enum Options {
    Controlled(VChild<SelectOption>),
    Uncontrolled(Html),
}

impl From<VChild<SelectOption>> for Options {
    fn from(child: VChild<SelectOption>) -> Self {
        Options::Controlled(child)
    }
}

impl From<Html> for Options {
    fn from(child: Html) -> Self {
        Options::Uncontrolled(child)
    }
}

impl Into<Html> for Options {
    fn into(self) -> Html {
        match self {
            Options::Controlled(child) => child.into(),
            Options::Uncontrolled(child) => child.into(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps<T: FormValue> {
    pub form: Form<T>,
    pub children: ChildrenRenderer<Options>,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Select)]
pub fn select<T: FormValue>(
    SelectProps {
        form,
        autocomplete,
        disabled,
        multiple,
        classes,
        classes_valid,
        classes_invalid,
        children,
        onchange,
    }: &SelectProps<T>,
) -> Html {
    let selected = form.state().value().to_owned();
    let classes = classes!(
        classes.clone(),
        form.state().dirty().then(|| match form.state().valid() {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );

    let onchange = {
        let form = form.clone();

        onchange.reform(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                form.state_mut().set(input.value());
            }

            e
        })
    };

    #[cfg(feature = "ybc")]
    html! {
       <ybc::Select
            // name={form_field.field_name()}
            name="select"
            {classes}
            disabled={*disabled}
            update={onchange}>
            { for children.iter().map(|option| {
                match option {
                    Options::Controlled(mut option) => {
                        let mut props = Rc::make_mut(&mut option.props);
                        props.selected = props.value == *selected;
                        option.into()
                    },
                    Options::Uncontrolled(option) => {
                        option
                    }
                }
            })}
       </ybc::Select>
    }

    #[cfg(not(feature = "ybc"))]
    html! {
        <select
            // id={form_field.field_name()}
            // name={form_field.field_name()}
            autocomplete={if *autocomplete {"on"} else {"off"}}
            disabled={*disabled}
            multiple={*multiple}
            class={classes}
            {onchange}
        >
            { for children.iter().map(|option| {
                match option {
                    Options::Controlled(mut option) => {
                        let mut props = Rc::make_mut(&mut option.props);
                        props.selected = *props.value == *selected;
                        option.into()
                    },
                    Options::Uncontrolled(option) => {
                        option
                    }
                }
            })}
        </select>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectOptionProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Option<Children>,
    #[prop_or_default]
    selected: bool,
}

#[function_component(SelectOption)]
pub fn select_item(
    SelectOptionProps {
        value,
        children,
        selected,
    }: &SelectOptionProps,
) -> Html {
    html! {
        <option selected={*selected} {value}>
            if let Some(children) = children {
                {children.clone()}
            } else {
                {value}
            }
        </option>
    }
}
