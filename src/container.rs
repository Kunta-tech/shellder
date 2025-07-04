use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Container {
    instances: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Any + Send + Sync>(&self, instance: T) {
        let mut instances = self.instances.write().unwrap();
        instances.insert(TypeId::of::<T>(), Arc::new(instance));
    }

    pub fn resolve<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let instances = self.instances.read().unwrap();
        instances
            .get(&TypeId::of::<T>())
            .and_then(|arc_any| arc_any.clone().downcast::<T>().ok())
    }
}
