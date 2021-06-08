use std::{borrow::Cow, future::Future};
use tokio::sync::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
    unbounded_channel,
};

use crate::{
    builder::Builder,
    component::{
        Component,
        ComponentResult,
        Identifier,
        error::ComponentError,
        request::Request,
    },
    contacts::{
        Contacts,
        builder::ContactsBuilder
    },
    routine::builder::RoutineBuilder,
    utils::get_new_id,
};

pub struct ComponentBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static + Future, {
    id: Identifier,
    name: N,
    sender: UnboundedSender<Request<M>>,
    recver: UnboundedReceiver<Request<M>>,
    routine_builder: RoutineBuilder<M, R>,
    contacts_builder: ContactsBuilder<M>,
    handler: fn(Contacts<M>, M) -> A,
}

impl<'a, M, R, A, N> ComponentBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static + Future,
N: Into<Cow<'a, str>>, {
    #[allow(unused)]
    pub fn new(
        name: N,
        routine_builder: RoutineBuilder<M, R>,
        handler: fn(Contacts<M>, M) -> A,
    ) -> ComponentResult<Self> {
        get_new_id()
            .map(|id| (id, unbounded_channel::<Request<M>>()))
            .map(|(id, (send, recv))| Self {
                id,
                name,
                sender: send,
                recver: recv,
                routine_builder,
                contacts_builder: ContactsBuilder::new(),
                handler,
            })
            .map_err(ComponentError::from)
    }

    #[allow(unused)]
    pub fn id(&self) -> Identifier { self.id }

    #[allow(unused)]
    pub fn sender(&self) -> UnboundedSender<Request<M>> { self.sender.clone() }

    #[allow(unused)]
    pub fn add_component(&mut self, component_builder: &Self) {
        self.contacts_builder
            .add_sender(
                component_builder.id(),
                component_builder.sender(),
            )
    }
}

impl<'a, M, R, A, N> Builder<Component<M, R, A>, ComponentError> for ComponentBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static + Future,
N: Into<Cow<'a, str>>, {
    fn build(self) -> ComponentResult<Component<M, R, A>> {
        Ok(Component::new(
            self.id,
            self.name,
            self.sender,
            self.recver,
            self.contacts_builder,
            self.routine_builder,
            self.handler,
        ))
    }
}