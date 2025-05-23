use std::sync::{Arc, RwLock};

/// Notifiers are a tool to listen to variable updates.
pub struct Notifier<T>
where
    T: PartialEq,
    T: Clone {
    inner_value: Arc<RwLock<T>>,
    listeners: RwLock<Vec<Arc<dyn Fn(&T)>>>,
    explicit_listeners: RwLock<Vec<Arc<dyn Fn(&T)>>>,
}

impl<T> Notifier<T>
where
    T: PartialEq,
    T: Clone {
    /// Create a new Notifier
    pub fn new(initial_value: T) -> Self {
        Self { inner_value: Arc::new(RwLock::new(initial_value)), listeners: RwLock::new(Vec::new()), explicit_listeners: RwLock::new(Vec::new()) }
    }

    /// Add a new listener. These listeners listen *only* to set operations where the value was actually different from the old one. If you want to explicitly listen on all set operations, see [listen_explicit](Notifier::listen_explicit]).
    pub fn listen(&self, listener: impl Fn(&T) + 'static) {
        self.listeners.write().unwrap().push(Arc::new(listener));
    }

    /// Add a new listener. These listeners listen to *every* set operation. If you want to listen to operations when the value is actually being changed, see [listen](Notifier::listen).
    pub fn listen_explicit(&self, listener: impl Fn(&T) + 'static) {
        self.explicit_listeners.write().unwrap().push(Arc::new(listener));
    }

    /// Set's the value inside the Notifier. Notifies all the listeners.
    pub fn set(&self, value: T) {
        let all = value != *self.inner_value.read().unwrap();
        *self.inner_value.write().unwrap() = value;
        self.notify(all);
    }

    // Get the value from the Notifier
    pub fn get(&self) -> T {
        let v = self.inner_value.read().unwrap();
        let x = &*v;

        x.clone()
    }

    fn notify(&self, all: bool) {
        if all {
            self.listeners.read().unwrap().iter().for_each(|listener| {
                listener(&self.get())
            })
        }
        self.explicit_listeners.read().unwrap().iter().for_each(|listener| {
            listener(&self.get())
        })
    }

    /// Allows doing multiple operations on the value as a single set operation.
    pub fn modify(&self, callback: impl Fn(&T) -> T + 'static) {
        let new = callback(&self.get());

        self.set(new);
    }
}

pub struct ReactiveValue<'a, T> {
    computer: Arc<dyn Fn() -> T + 'a>
}

impl<'a, T> ReactiveValue<'a, T> {
    pub fn new(computer: Arc<dyn Fn() -> T + 'a>) -> Self {
        Self { computer }
    }
    
    pub fn get(&self) -> T {
        (self.computer)()
    }
}