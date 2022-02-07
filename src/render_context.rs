#![warn(unsafe_op_in_unsafe_fn)]
use std::sync::Arc;

use erupt::{
    utils::loading::EntryLoaderError,
    vk::{ApplicationInfoBuilder, InstanceCreateInfo, InstanceCreateInfoBuilder, API_VERSION_1_2},
    EntryLoader, InstanceLoader,
};

#[derive(Debug)]
struct Entry {
    deref: EntryLoader,
}

impl Entry {
    fn new() -> Result<Self, EntryLoaderError> {
        Ok(Self {
            deref: EntryLoader::new()?,
        })
    }
}

//kept just in case
impl Drop for Entry {
    fn drop(&mut self) {}
}

#[derive(Debug)]
struct Instance {
    deref: InstanceLoader,
    _parent: Arc<Entry>,
}

impl std::ops::Deref for Instance {
    type Target = InstanceLoader;

    fn deref(&self) -> &Self::Target {
        &self.deref
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        //SAFETY: we are dropping so we never use the instance after this
        unsafe { self.deref.destroy_instance(None) };
    }
}
impl Instance {
    fn new(
        instance_create_info: &InstanceCreateInfo,
        parent: &Arc<Entry>,
    ) -> Result<Self, erupt::LoaderError> {
        //okay because so long as we have a reference to Instance we have an Arc to the parent
        let i = unsafe { InstanceLoader::new(parent, instance_create_info) }?;
        Ok(Self {
            deref: i,
            _parent: parent.clone(),
        })
    }
}

impl std::ops::Deref for Entry {
    type Target = EntryLoader;

    fn deref(&self) -> &Self::Target {
        &self.deref
    }
}

#[derive(Debug)]
pub struct RenderContext {
    _entry: Arc<Entry>,
    _instance: Arc<Instance>,
}

impl RenderContext {
    pub fn new() -> Self {
        let entry = Arc::new(Entry::new().unwrap());
        let app_info = ApplicationInfoBuilder::new().api_version(API_VERSION_1_2);
        let instance_info = InstanceCreateInfoBuilder::new().application_info(&app_info);

        let instance = Arc::new(Instance::new(&instance_info, &entry).unwrap());
        RenderContext {
            _entry: entry,
            _instance: instance,
        }
    }
}
