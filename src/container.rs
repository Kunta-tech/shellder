// Copyright (c) 2024 Your Name
// Licensed under the MIT OR Apache-2.0 License

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

enum Entry {
    Instance(Arc<dyn Any + Send + Sync>),
    Factory(Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>),
}

pub struct Container {
    instances: RwLock<HashMap<TypeId, Entry>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Any + Send + Sync>(&self, instance: T) -> Result<(), String> {
        let mut instances = self.instances.write().unwrap();
        let type_id = TypeId::of::<T>();
        if instances.contains_key(&type_id) {
            return Err(format!("Type already registered: {}", std::any::type_name::<T>()));
        }
        instances.insert(type_id, Entry::Instance(Arc::new(instance)));
        Ok(())
    }

    pub fn register_lazy<T, F>(&self, factory: F) -> Result<(), String>
    where
        T: Any + Send + Sync,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let mut instances = self.instances.write().unwrap();
        let type_id = TypeId::of::<T>();
        if instances.contains_key(&type_id) {
            return Err(format!("Type already registered: {}", std::any::type_name::<T>()));
        }

        // Wrap factory to return Arc<dyn Any>
        let boxed_factory = Box::new(move || Arc::new(factory()) as Arc<dyn Any + Send + Sync>);
        instances.insert(type_id, Entry::Factory(boxed_factory));
        Ok(())
    }

    pub fn resolve<T: Any + Send + Sync>(&self) -> Result<Arc<T>, String> {
        let mut instances = self.instances.write().unwrap();
        let type_id = TypeId::of::<T>();
        let entry = instances.get_mut(&type_id).ok_or_else(|| {
            format!("Type not found: {}", std::any::type_name::<T>())
        })?;

        match entry {
            Entry::Instance(arc_any) => {
                // Downcast
                arc_any.clone()
                    .downcast::<T>()
                    .map_err(|_| format!("Failed to downcast: {}", std::any::type_name::<T>()))
            }
            Entry::Factory(factory) => {
                // Call factory
                let new_instance = factory();

                // Promote to Instance
                *entry = Entry::Instance(new_instance.clone());

                // Downcast
                new_instance
                    .downcast::<T>()
                    .map_err(|_| format!("Failed to downcast after factory: {}", std::any::type_name::<T>()))
            }
        }
    }
}
