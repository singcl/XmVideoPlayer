use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

#[derive(Default)]
pub struct Database(pub(crate) Arc<Mutex<HashMap<String, String>>>);

pub struct Counter(pub(crate) AtomicUsize);

pub struct Client;

impl Client {
    pub fn send(&self) {}
}

#[derive(Default)]
pub struct Connection(pub(crate) Mutex<Option<Client>>);
