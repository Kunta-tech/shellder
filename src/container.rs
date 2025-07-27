// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{CliLogger, Logger, RegistrationError, ResolveError};


enum Entry {
    Instance(Arc<dyn Any + Send + Sync>),
    Factory(Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>),
}

pub struct Container {
    instances: RwLock<HashMap<TypeId, Entry>>,
    instance_names: RwLock<HashMap<TypeId, &'static str>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
            instance_names: RwLock::new(HashMap::new()),
        }
    }
    pub fn register<T: Any + Send + Sync>(&self, instance: T) -> Result<(), RegistrationError> {
        let mut instances = self.instances.write().unwrap();
        let mut inst_names = self.instance_names.write().unwrap();
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>();
        if instances.contains_key(&type_id) {
            CliLogger::warn(&format!("({}) is already registered", type_name));
            return Err(RegistrationError::AlreadyRegistered(std::any::type_name::<T>()));
        }
        instances.insert(type_id, Entry::Instance(Arc::new(instance)));
        inst_names.insert(type_id, type_name);
        Ok(())
    }
    pub fn register_lazy<T, F>(&self, factory: F) -> Result<(), RegistrationError>
    where
        T: Any + Send + Sync,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let mut instances = self.instances.write().unwrap();
        let mut inst_names = self.instance_names.write().unwrap();
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>();
        if instances.contains_key(&type_id) {
            CliLogger::warn(&format!("({}) is already registered", type_name));
            return Err(RegistrationError::AlreadyRegistered(std::any::type_name::<T>()));
        }

        // Wrap factory to return Arc<dyn Any>
        let boxed_factory = Box::new(move || Arc::new(factory()) as Arc<dyn Any + Send + Sync>);
        instances.insert(type_id, Entry::Factory(boxed_factory));
        inst_names.insert(type_id, type_name);
        Ok(())
    }

    pub fn resolve<T: Any + Send + Sync>(&self) -> Result<Arc<T>, ResolveError> {
        let mut instances = self.instances.write().unwrap();
        let type_id = TypeId::of::<T>();
        let entry = instances.get_mut(&type_id).ok_or_else(|| {
            CliLogger::warn(&format!("({}) Not Found", std::any::type_name::<T>()));
            ResolveError::NotFound(std::any::type_name::<T>())
        })?;

        match entry {
            Entry::Instance(arc_any) => {
                // Downcast
                arc_any.clone()
                    .downcast::<T>()
                    .map_err(|_| ResolveError::DowncastFailed(std::any::type_name::<T>()))
            }
            Entry::Factory(factory) => {
                // Call factory
                let new_instance = factory();

                // Promote to Instance
                *entry = Entry::Instance(new_instance.clone());

                // Downcast
                new_instance
                    .downcast::<T>()
                    .map_err(|_| ResolveError::DowncastFailed(std::any::type_name::<T>()))
            }
        }
    }
    pub fn remove<T: Any + Send + Sync>(&self) -> Result<(), ResolveError> {
        let mut instances = self.instances.write().unwrap();
        let type_id = TypeId::of::<T>();
        match instances.remove(&type_id) {
            Some(_) => {
                let mut names = self.instance_names.write().unwrap();
                names.remove(&type_id);
                return Ok(())
            },
            None => return Err(ResolveError::NotFound(std::any::type_name::<T>())),
        }
    }
    pub fn clear(&self){
        let mut instances = self.instances.write().unwrap();
        let mut names = self.instance_names.write().unwrap();
        instances.clear();
        names.clear();
    }

}
impl std::fmt::Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = self.instances.read().unwrap();
        let names = self.instance_names.read().unwrap();
        writeln!(f, "Container contents:")?;
        for (type_id, entry) in entries.iter() {
            println!(
                "- {:?}: {}",
                names[type_id],
                match entry {
                    Entry::Instance { .. } => "Instance",
                    Entry::Factory { .. } => "Factory",
                }
            );
        }
        Ok(())
    }
}

