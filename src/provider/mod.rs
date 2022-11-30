use std::any::{TypeId};
use std::ops::Deref;
use std::sync::Arc;
use downcast_rs::{DowncastSync, impl_downcast};

impl_downcast!(sync Component);
pub trait Component: DowncastSync {}

pub trait Provider: Clone {

    type ProviderImpl: Clone;

    fn identity(self) -> Self::ProviderImpl;
    fn store<T: Component>(&mut self, key: TypeId, value: Arc<T>);
    fn retrieve(&self, key: &TypeId) -> Option<&Arc<dyn Component>>;

    fn manage<T: Component>(&mut self, item: T) -> Self::ProviderImpl  {
         self.store(item.type_id(), Arc::new(item));
         self.deref().clone().identity()
    }

    fn get<U: Component>(&self) -> Arc<U> {
        let key = TypeId::of::<U>();
        if let Some(it) = self.retrieve(&key) {
            if let Ok(service) = it.clone().downcast_arc::<U>()  {
                return service.clone()
            }
        }
        panic!("Service of type {} not found. Did you forget a manage() call", std::any::type_name::<U>())
    }
}
