use yew::prelude::*;

use crate::{form::Form, form_state::OwnedFormState, model::Model};

#[hook]
pub fn use_form<T>(init_fn: impl FnOnce() -> T) -> Form<T>
where
    T: Model + 'static,
{
    use_form_with_deps(|_| init_fn(), ())
}

#[hook]
pub fn use_form_with_deps<T, D>(init_fn: impl FnOnce(&D) -> T, deps: D) -> Form<T>
where
    T: Model + 'static,
    D: PartialEq + 'static,
{
    let counter = use_state(|| 0);
    let form_state = use_memo(|d| OwnedFormState::new(init_fn(d)), deps);
    Form::with_state(form_state, counter)
}
