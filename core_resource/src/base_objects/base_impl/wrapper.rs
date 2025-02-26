use std::{cell::RefCell, ptr::NonNull, rc::Rc};

use super::{base_object::BaseObject, BaseObjectContainer};
use crate::{
    meta::base_object::{normal_meta::NormalBaseObjectMeta, synced_meta::SyncedBaseObjectMeta},
    SomeResult, VoidResult,
};

pub struct BaseObjectWrapper<T, InheritPtrs: Clone = ()> {
    pub(crate) value: RefCell<BaseObject<T, InheritPtrs>>,
}

impl<T, InheritPtrs: Clone> BaseObjectWrapper<T, InheritPtrs> {
    pub(crate) fn _new(
        ptr: NonNull<T>,
        base_ptr: altv_sdk::BaseObjectMutPtr,
        inherit_ptrs: InheritPtrs,
    ) -> BaseObjectContainer<T, InheritPtrs> {
        Rc::new(Self {
            value: RefCell::new(BaseObject {
                ptr: Some(ptr),
                base_ptr: Some(base_ptr),
                inherit_ptrs: Some(inherit_ptrs),
            }),
        })
    }

    pub(crate) fn ptr(&self) -> SomeResult<NonNull<T>> {
        self.value.try_borrow()?.ptr()
    }

    pub(crate) fn raw_ptr(&self) -> SomeResult<*mut T> {
        self.value.try_borrow()?.raw_ptr()
    }

    pub(crate) fn internal_destroy(&self) -> VoidResult {
        self.value.try_borrow_mut()?.internal_destroy()
    }
}

impl<T, InheritPtrs: Clone> SyncedBaseObjectMeta<T, InheritPtrs>
    for BaseObjectWrapper<T, InheritPtrs>
{
}

impl<T, InheritPtrs: Clone> NormalBaseObjectMeta<T, InheritPtrs>
    for BaseObjectWrapper<T, InheritPtrs>
{
}
