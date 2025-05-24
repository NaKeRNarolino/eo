use std::sync::{Arc, RwLock};

type CallbackType<'a, T> = Arc<dyn Fn(&T) + 'a>;

pub struct Event<'a, T> {
    subscribers: RwLock<Vec<CallbackType<'a, T>>>
}

impl<'a, T> Event<'a, T> {
    pub fn new(subscribers: RwLock<Vec<CallbackType<'a, T>>>) -> Self {
        Self { subscribers }
    }
    
    pub fn subscribe(&self, callback: impl Fn(&T) + 'a) {
        self.subscribers.write().unwrap().push(Arc::new(callback));
    }
    
    pub fn notify(&self, value: T) {
        self.subscribers.read().unwrap().iter().for_each(|subscriber| {
            subscriber(&value);
        })
    }
}