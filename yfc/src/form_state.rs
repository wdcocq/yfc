use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

use yew::AttrValue;

use crate::{
    field::Field,
    form_value::{FormValue, FormValueState},
    model::{Model, ModelRelation},
};

pub(crate) trait FormState<T>
where
    T: StateProvider,
{
    fn model(&self) -> Ref<T>;
    fn state(&self) -> Ref<<T as StateProvider>::State>;
    fn state_mut<'a>(&'a self) -> <T as StateProvider>::StateMut<'a>;
}

pub(crate) struct OwnedFormState<T>
where
    T: Model,
{
    pub(crate) inner: RefCell<(T, <T as StateProvider>::State)>,
}

impl<T> OwnedFormState<T>
where
    T: Model,
{
    pub fn new(model: T) -> Self {
        let state = model.create_state();
        Self {
            inner: RefCell::new((model, state)),
        }
    }
}

impl<T> FormState<T> for OwnedFormState<T>
where
    T: Model,
{
    fn model(&self) -> Ref<T> {
        Ref::map(self.inner.borrow(), |i| &i.0)
    }

    fn state(&self) -> Ref<<T as StateProvider>::State> {
        Ref::map(self.inner.borrow(), |i| &i.1)
    }

    fn state_mut<'a>(&'a self) -> <T as StateProvider>::StateMut<'a> {
        let (model, state) = RefMut::map_split(self.inner.borrow_mut(), |s| (&mut s.0, &mut s.1));
        T::create_state_mut(model, state)
    }
}

pub(crate) struct RefFormState<P, C, R>
where
    P: Model,
    C: StateProvider,
    R: ModelRelation<P, C>,
{
    parent_state: Rc<dyn FormState<P>>,
    relation: R,
    phantom: PhantomData<C>,
}

impl<P, C, R> RefFormState<P, C, R>
where
    P: Model,
    C: StateProvider,
    R: ModelRelation<P, C>,
{
    pub fn new(parent_state: Rc<dyn FormState<P>>, relation: R) -> Self {
        Self {
            parent_state,
            relation,
            phantom: PhantomData,
        }
    }
}

pub struct ListMut<'a, T>
where
    T: StateProvider,
{
    values: RefMut<'a, Vec<T>>,
    states: RefMut<'a, Vec<<T as StateProvider>::State>>,
}

impl<'a, T> ListMut<'a, T>
where
    T: StateProvider,
{
    pub fn push(&mut self, element: T) {
        self.states.push(element.create_state());
        self.values.push(element);
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.states.insert(index, element.create_state());
        self.values.insert(index, element);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.states.remove(index);
        self.values.remove(index)
    }
}

impl<'a, T> StateMut<'a, Vec<T>> for ListMut<'a, T>
where
    T: StateProvider,
{
    fn split(self) -> (RefMut<'a, Vec<T>>, RefMut<'a, Vec<T::State>>) {
        (self.values, self.states)
    }
}

impl<P, C, R> FormState<C> for RefFormState<P, C, R>
where
    P: Model,
    C: StateProvider,
    R: ModelRelation<P, C>,
{
    fn model(&self) -> Ref<C> {
        Ref::map(self.parent_state.model(), |m| {
            self.relation.relation_model(m)
        })
    }

    fn state(&self) -> Ref<<C as StateProvider>::State> {
        Ref::map(self.parent_state.state(), |s| {
            self.relation.relation_state(s)
        })
    }

    fn state_mut<'a>(&'a self) -> <C as StateProvider>::StateMut<'a> {
        StateMut::map(self.parent_state.state_mut(), &self.relation)
    }
}

pub trait StateProvider: Sized {
    type State: PartialEq;
    type StateMut<'a>: StateMut<'a, Self>
    where
        Self: 'a;

    fn create_state(&self) -> Self::State;
    fn create_state_mut<'a>(
        model: RefMut<'a, Self>,
        state: RefMut<'a, Self::State>,
    ) -> Self::StateMut<'a>;
}

pub trait StateMut<'a, T>
where
    T: StateProvider,
{
    fn split(self) -> (RefMut<'a, T>, RefMut<'a, T::State>);
    fn map<C, R>(self, relation: &R) -> <C as StateProvider>::StateMut<'a>
    where
        Self: Sized,
        T: Model + 'a,
        C: StateProvider + 'a,
        R: ModelRelation<T, C>,
    {
        let (model, state) = self.split();
        let model = RefMut::map(model, |m| relation.relation_model_mut(m));
        let state = RefMut::map(state, |s| relation.relation_state_mut(s));
        C::create_state_mut(model, state)
    }
}

pub trait ValueStateMut<'a, T>
where
    T: StateProvider<State = Field>,
{
    fn set<S: Into<AttrValue>>(&mut self, value: S);
}

macro_rules! impl_state_provider {
    ($($t:ty),*) => {
        $(
            impl StateProvider for $t {
                type State = Field;
                type StateMut<'a> = FormValueState<'a, $t>;

                fn create_state(&self) -> Self::State {
                    Field::new(self.value())
                }

                fn create_state_mut<'a>(
                    model: RefMut<'a, Self>,
                    state: RefMut<'a, Self::State>,
                ) -> Self::StateMut<'a> {
                    FormValueState {
                        value: model,
                        field: state,
                    }
                }
            }
        )*
    };
}

impl_state_provider!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, String
);

impl<T> StateProvider for Vec<T>
where
    T: StateProvider,
{
    type State = Vec<T::State>;
    type StateMut<'a> = ListMut<'a, T> where T: 'a;

    fn create_state(&self) -> Self::State {
        self.iter().map(StateProvider::create_state).collect()
    }

    fn create_state_mut<'a>(
        model: RefMut<'a, Self>,
        state: RefMut<'a, Self::State>,
    ) -> Self::StateMut<'a> {
        ListMut {
            values: model,
            states: state,
        }
    }
}

pub struct OptionStateMut<'a, T>
where
    T: StateProvider + Default,
{
    model: RefMut<'a, Option<T>>,
    state: RefMut<'a, T::State>,
}

impl<'a, T> StateMut<'a, Option<T>> for OptionStateMut<'a, T>
where
    T: StateProvider + Default,
    <T as StateProvider>::State: Default,
{
    fn split(
        self,
    ) -> (
        RefMut<'a, Option<T>>,
        RefMut<'a, <Option<T> as StateProvider>::State>,
    ) {
        (self.model, self.state)
    }
}

impl<'a, T> ValueStateMut<'a, Option<T>> for OptionStateMut<'a, T>
where
    T: FormValue + Default,
{
    fn set<S: Into<AttrValue>>(&mut self, value: S) {
        let value = value.into();
        match value.is_empty() {
            true => {
                *self.model = None;
                self.state.set_value("");
            }
            false => {
                *self.model = Some(T::from_value(&value));
                self.state.set_value(value);
            }
        }
    }
}

impl<T> StateProvider for Option<T>
where
    T: StateProvider + Default,
    <T as StateProvider>::State: Default,
{
    type State = T::State;
    type StateMut<'a> = OptionStateMut<'a, T>
    where
        Self: 'a;

    fn create_state(&self) -> Self::State {
        match self {
            Some(value) => value.create_state(),
            None => Default::default(),
        }
    }

    fn create_state_mut<'a>(
        model: RefMut<'a, Self>,
        state: RefMut<'a, Self::State>,
    ) -> Self::StateMut<'a> {
        OptionStateMut { model, state }
    }
}
