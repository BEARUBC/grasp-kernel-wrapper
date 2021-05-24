use std::{
    future::Future,
    rc::Rc,
};

use crate::contacts::contacts::Contacts;

#[derive(Clone)]
pub enum Job<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    Spacer(u64),
    Lambda(Box<fn(Rc<Contacts<M>>) -> T>),
}

unsafe impl<T, M> Send for Job<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {}

unsafe impl<T, M> Sync for Job<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {}

impl<T, M> From<fn(Rc<Contacts<M>>) -> T> for Job<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn from(lambda: fn(Rc<Contacts<M>>) -> T) -> Self { Self::Lambda(Box::new(lambda)) }
}
