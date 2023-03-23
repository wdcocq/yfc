use crate::{form_state::StateProvider, prelude::FormValue};

pub trait Model
where
    Self: StateProvider,
{
}

pub trait ModelRelation<P, C>
where
    P: Model,
    C: StateProvider,
{
    fn relation_model<'a>(&self, parent: &'a P) -> &'a C;
    fn relation_model_mut<'a>(&self, parent: &'a mut P) -> &'a mut C;
    fn relation_state<'a>(&self, parent: &'a P::State) -> &'a C::State;
    fn relation_state_mut<'a>(&self, parent: &'a mut P::State) -> &'a mut C::State;
}

impl<T> Model for T where T: FormValue {}
