use std::fmt::Debug;
use std::marker::PhantomData;

use crate::header::EmailAddress;

#[derive(Debug)]
pub struct Message {
    pub from: EmailAddress,
    pub to: EmailAddress,
}

#[derive(Debug, Default)]
pub struct Yes;
#[derive(Debug, Default)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[derive(Debug, Clone, Default)]
pub struct MessageBuilder<FROM, TO>
where
    FROM: ToAssign,
    TO: ToAssign,
{
    from_set: PhantomData<FROM>,
    to_set: PhantomData<TO>,
    from: EmailAddress,
    to: EmailAddress,
}

impl<TO> MessageBuilder<No, TO>
where
    TO: ToAssign,
{
    pub fn from(self, from: EmailAddress) -> MessageBuilder<Yes, TO> {
        MessageBuilder {
            from_set: PhantomData {},
            to_set: PhantomData {},
            from,
            to: self.to,
        }
    }
}

impl<FROM> MessageBuilder<FROM, No>
where
    FROM: ToAssign,
{
    pub fn to(self, to: EmailAddress) -> MessageBuilder<FROM, Yes> {
        MessageBuilder {
            from_set: PhantomData {},
            to_set: PhantomData {},
            from: self.from,
            to,
        }
    }
}

impl MessageBuilder<Yes, Yes> {
    pub fn build(&self) -> Message {
        Message {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

pub fn message_builder() -> MessageBuilder<No, No> {
    MessageBuilder::default()
}
