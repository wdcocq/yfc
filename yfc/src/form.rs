use std::{cell::Ref, rc::Rc};

use yew::UseStateHandle;

use crate::{
    form_state::{FormState, OwnedFormState, RefFormState, StateProvider},
    form_value::FormValue,
    model::ModelRelation,
    Model,
};

pub struct Form<T>
where
    T: StateProvider,
{
    form_state: Rc<dyn FormState<T>>,
    counter: UseStateHandle<u32>,
}

impl<T> Clone for Form<T>
where
    T: StateProvider,
{
    fn clone(&self) -> Self {
        Self {
            form_state: self.form_state.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl<T> PartialEq for Form<T>
where
    T: StateProvider,
{
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.form_state, &other.form_state) && self.counter == other.counter
    }
}

impl<T> Form<T>
where
    T: StateProvider,
{
    pub fn state(&self) -> Ref<<T as StateProvider>::State> {
        self.form_state.state()
    }

    pub fn state_mut<'a>(&'a self) -> <T as StateProvider>::StateMut<'a> {
        self.counter.set((*self.counter).wrapping_add(1));
        self.form_state.state_mut()
    }
}

impl<'a, T> Form<T>
where
    T: Model + 'static,
{
    pub fn new(model: T, counter: UseStateHandle<u32>) -> Self {
        Self {
            form_state: Rc::new(OwnedFormState::new(model)),
            counter,
        }
    }

    pub(crate) fn with_state(state: Rc<OwnedFormState<T>>, counter: UseStateHandle<u32>) -> Self {
        Self {
            form_state: state,
            counter,
        }
    }

    pub fn seed<C, R>(&self, relation: R) -> Form<C>
    where
        C: StateProvider + 'static,
        R: ModelRelation<T, C> + 'static,
    {
        Form {
            form_state: Rc::new(RefFormState::<T, C, R>::new(
                self.form_state.clone(),
                relation,
            )),
            counter: self.counter.clone(),
        }
    }
    pub fn model(&self) -> Ref<T> {
        self.form_state.model()
    }
}

impl<T> Form<T>
where
    T: FormValue,
{
    pub fn value<'a>(&'a self) -> Ref<'a, T> {
        self.form_state.model()
    }
}
