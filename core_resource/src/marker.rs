use crate::{
    base_objects::{marker, player},
    helpers,
    rgba::RGBA,
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
};

use autocxx::prelude::*;
use std::ptr::NonNull;

/// # **`Marker implementation`**
impl marker::Marker {
    pub fn new(
        marker_type: altv_sdk::MarkerType,
        pos: impl Into<Vector3>,
        color: impl Into<RGBA>,
    ) -> marker::MarkerContainer {
        let pos = pos.into();
        let color = color.into();

        let ptr = unsafe {
            sdk::ICore::CreateMarker(
                std::ptr::null_mut(),
                marker_type as u32,
                pos.x(),
                pos.y(),
                pos.z(),
                color.r(),
                color.g(),
                color.b(),
                color.a(),
                std::ptr::null_mut(),
            )
        };

        marker::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn destroy(&self) -> VoidResult {
        marker::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IMarker::GetID(self.raw_ptr()?) })
    }

    pub fn global(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IMarker::IsGlobal(self.raw_ptr()?) })
    }

    pub fn target(&self) -> SomeResult<Option<player::PlayerContainer>> {
        helpers::get_any_option_base_object!(sdk::IMarker::GetTarget(self.raw_ptr()?), player)
    }

    pub fn color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IMarker::GetColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color = color.into();

        unsafe {
            sdk::IMarker::SetColor(self.raw_ptr()?, color.r(), color.g(), color.b(), color.a())
        }
        Ok(())
    }

    pub fn visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IMarker::IsVisible(self.raw_ptr()?) })
    }

    pub fn set_visible(&self, visible: bool) -> VoidResult {
        unsafe { sdk::IMarker::SetVisible(self.raw_ptr()?, visible) }
        Ok(())
    }

    pub fn marker_type(&self) -> SomeResult<altv_sdk::MarkerType> {
        let raw = unsafe { sdk::IMarker::GetMarkerType(self.raw_ptr()?) };
        Ok(altv_sdk::MarkerType::try_from(raw).unwrap())
    }

    pub fn set_marker_type(&self, marker_type: altv_sdk::MarkerType) -> VoidResult {
        unsafe { sdk::IMarker::SetMarkerType(self.raw_ptr()?, marker_type as u32) };
        Ok(())
    }

    pub fn scale(&self) -> SomeResult<Vector3> {
        Ok(helpers::read_cpp_vector3(unsafe {
            sdk::IMarker::GetScale(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    pub fn set_scale(&self, scale: impl Into<Vector3>) -> VoidResult {
        let scale = scale.into();

        unsafe { sdk::IMarker::SetScale(self.raw_ptr()?, scale.x(), scale.y(), scale.z()) };
        Ok(())
    }

    pub fn rot(&self) -> SomeResult<Vector3> {
        Ok(helpers::read_cpp_vector3(unsafe {
            sdk::IMarker::GetRotation(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    pub fn set_rot(&self, rot: impl Into<Vector3>) -> VoidResult {
        let rot = rot.into();
        unsafe { sdk::IMarker::SetRotation(self.raw_ptr()?, rot.x(), rot.y(), rot.z()) };

        Ok(())
    }

    pub fn dir(&self) -> SomeResult<Vector3> {
        Ok(helpers::read_cpp_vector3(unsafe {
            sdk::IMarker::GetDirection(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    pub fn set_dir(&self, dir: impl Into<Vector3>) -> VoidResult {
        let dir = dir.into();
        unsafe { sdk::IMarker::SetDirection(self.raw_ptr()?, dir.x(), dir.y(), dir.z()) };

        Ok(())
    }

    pub fn face_camera(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IMarker::IsFaceCamera(self.raw_ptr()?) })
    }

    pub fn set_face_camera(&self, face_camera: bool) -> VoidResult {
        unsafe { sdk::IMarker::SetFaceCamera(self.raw_ptr()?, face_camera) }
        Ok(())
    }

    pub fn streaming_distance(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IMarker::GetStreamingDistance(self.raw_ptr()?) })
    }
}
