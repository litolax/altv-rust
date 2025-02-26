use crate::{base_objects::virtual_entity_group, sdk, SomeResult};
use std::ptr::NonNull;

/// # **`VirtualEntityGroup implementation`**
impl virtual_entity_group::VirtualEntityGroup {
    pub fn new(max_entities_in_stream: u32) -> virtual_entity_group::VirtualEntityGroupContainer {
        let ptr = unsafe { sdk::ICore::CreateVirtualEntityGroup(max_entities_in_stream) };
        virtual_entity_group::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntityGroup::GetID(self.raw_ptr()?) })
    }

    pub fn max_entities_in_stream(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntityGroup::GetStreamingRangeLimit(self.raw_ptr()?) })
    }

    // cannot be destroyed
    // pub fn destroy(&self) -> VoidResult {
    //     virtual_entity_group::remove_from_pool!(self)?;
    //     self.internal_destroy()
    // }
}
