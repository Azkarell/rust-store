use std::marker::PhantomData;
use type_uuid::{Bytes, TypeUuid};
use uuid::Uuid;
use crate::StoreHandle;
use thiserror::*;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("invalid cast {from} to {to}", from = Uuid::from_slice(.from).unwrap(), to = Uuid::from_slice(.to).unwrap())]
    InvalidCast { from: Bytes, to: Bytes },
}


pub struct StoreHandleUntyped {
    id: Uuid,
    type_id: Bytes,
}

impl StoreHandleUntyped {
    pub fn new<T: TypeUuid>() -> Self {
        StoreHandleUntyped {
            id: Uuid::new_v4(),
            type_id: T::UUID,
        }
    }

    pub fn try_typed<T: TypeUuid>(&self) -> Option<StoreHandle<T>> {
        if T::UUID != self.type_id {
            return None
        }
        Some(StoreHandle(self.id, PhantomData))
    }
}

impl<T: TypeUuid> From<StoreHandle<T>> for StoreHandleUntyped {
    fn from(h: StoreHandle<T>) -> Self {
        Self {
            type_id: T::UUID,
            id: h.0,
        }
    }
}

impl<T: TypeUuid> TryFrom<StoreHandleUntyped> for StoreHandle<T> {
    type Error = anyhow::Error;

    fn try_from(value: StoreHandleUntyped) -> Result<Self, Self::Error> {
        if value.type_id != T::UUID {
            return Err(StoreError::InvalidCast {
                from: value.type_id,
                to: T::UUID,
            }.into());
        }
        Ok(StoreHandle(value.id, PhantomData))
    }
}


impl<T: TypeUuid> StoreHandle<T> {
    pub fn to_untyped(&self) -> StoreHandleUntyped {
        StoreHandleUntyped {
            type_id: T::UUID,
            id: self.0,
        }
    }
}

#[cfg(test)]
mod test {
    use type_uuid::TypeUuid;
    use crate::Store;

    #[derive(TypeUuid, Debug, PartialEq, Eq, Clone)]
    #[uuid = "a0a0a0a0-a0a0-a0a0-a0a0-a0a0a0a0a0a0"]
    struct UntypedTest;

    #[test]
    fn untyped_should_work(){


        let mut store = Store::new();
        let val = UntypedTest{};
        let handle = store.insert(val.clone());
        let untyped = handle.to_untyped();

        let retrieved = store.get(&untyped.try_typed().unwrap());

        assert_eq!(handle.0, untyped.id);
        assert_eq!(untyped.type_id, UntypedTest::UUID);
        assert_eq!(retrieved, Some(&val));
    }
}