// experimental setting up mailboxes for components

use crate::state_functions::{
    get_state_with_topo_id, set_state_with_topo_id, update_state_with_topo_id, use_state,
};
use std::marker::PhantomData;
#[derive(Clone, Default)]
pub struct Mailbox<T> {
    pub messages: Vec<(topo::Id, T)>,
}

impl<T> Mailbox<T> {
    pub fn new() -> Mailbox<T> {
        Mailbox { messages: vec![] }
    }
}
#[derive(Clone)]
pub struct MailboxControl<T>
where
    T: Clone + 'static,
{
    pub id: topo::Id,
    _phantom_data: PhantomData<T>,
}

impl<T> MailboxControl<T>
where
    T: Clone + 'static,
{
    pub fn new(id: topo::Id) -> MailboxControl<T> {
        MailboxControl::<T> {
            id,
            _phantom_data: PhantomData,
        }
    }

    pub fn send<M: Clone + 'static>(&self, other_id: topo::Id, message: M) {
        update_state_with_topo_id::<Mailbox<M>, _>(other_id, |mb| {
            mb.messages.push((topo::Id::current(), message))
        });
    }

    pub fn pop(&self) -> Option<(topo::Id, T)> {
        if let Some(mut mailbox) = get_state_with_topo_id::<Mailbox<T>>(self.id) {
            if let Some(msg) = mailbox.messages.pop() {
                set_state_with_topo_id(mailbox, self.id);
                Some(msg)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn use_mailbox<T: Clone>() -> (Mailbox<T>, MailboxControl<T>) {
    let (mailbox, _mailbox_access) = use_state(Mailbox::<T>::new);

    let ctrl = MailboxControl::new(topo::Id::current());

    (mailbox, ctrl)
}
