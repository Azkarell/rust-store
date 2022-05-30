use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use uuid::Uuid;



#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct StoreHandle<T>(Uuid, PhantomData<T>);

#[cfg(feature = "untyped")]
pub mod untyped;



impl<T> StoreHandle<T> {
    pub fn new() -> Self {
        Self(Uuid::new_v4(), PhantomData)
    }
}



pub struct Store<T> {
    items: HashMap<Uuid, T>,
}


impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    pub fn insert(&mut self, item: T) -> StoreHandle<T> {
        let handle = StoreHandle::new();
        self.items.insert(handle.0, item);
        handle
    }
    pub fn get(&self, handle: &StoreHandle<T>) -> Option<&T> {
        self.items.get(&handle.0)
    }
    pub fn get_mut(&mut self, handle: &StoreHandle<T>) -> Option<&mut T> {
        self.items.get_mut(&handle.0)
    }
    pub fn remove(&mut self, handle: &StoreHandle<T>) {
        self.items.remove(&handle.0);
    }
}


impl<T> Index<StoreHandle<T>> for Store<T> {
    type Output = T;
    fn index(&self, handle: StoreHandle<T>) -> &Self::Output {
        &self.items[&handle.0]
    }
}

impl<T> IndexMut<StoreHandle<T>> for Store<T>{
    fn index_mut(&mut self, handle: StoreHandle<T>) -> &mut Self::Output {
        self.items.get_mut(&handle.0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_store() {
        let mut store = Store::new();
        let handle = store.insert(42);
        assert_eq!(store.get(&handle), Some(&42));
        assert_eq!(store.get_mut(&handle), Some(&mut 42));
        store.remove(&handle);
        assert_eq!(store.get(&handle), None);
    }

    #[test]
    fn test_index() {
        let mut store = Store::new();
        let handle = store.insert(42);
        assert_eq!(store[handle], 42);
    }

    #[test]
    fn test_index_mut(){
        let mut store = Store::new();
        let handle = store.insert(42);
        store[handle] = 43;
        assert_eq!(store[handle], 43);
    }
}