use std::{marker::PhantomData, sync::MutexGuard};

pub type PhantomUnsync = PhantomData<std::cell::Cell<()>>;
pub type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
