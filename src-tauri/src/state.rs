use std::{
  collections::HashMap,
  sync::{
      atomic::{AtomicUsize, Ordering},
      Arc, Mutex,
  },
};

#[derive(Default)]
pub struct Database(pub(crate) Arc<Mutex<HashMap<String, String>>>);