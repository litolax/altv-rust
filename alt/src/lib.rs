use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use altv_sdk::ffi;
use cxx::let_cxx_string;

pub use ffi::set_alt_core as __set_alt_core;
pub use ffi::ICore as __alt_ICore;
use once_cell::sync::OnceCell;
use resource_api::TestData;
use resource_api::TestDataContainer;
pub use resource_main_macro::resource_main_func as res_main;

pub use resource_api;

pub type ModelHash = u32;

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
pub fn hash(str: &str) -> ModelHash {
    let bytes = str.as_bytes();
    let mut num: std::num::Wrapping<u32> = std::num::Wrapping(0u32);

    for n in bytes {
        num += std::num::Wrapping(*n as u32);
        num += num << 10;
        num ^= num >> 6;
    }

    num += num << 3;
    num ^= num >> 11;

    (num + (num << 15)).0
}

pub fn log(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        ffi::log_colored(&cxx_str);
    }
}

pub struct Vehicle {
    ptr: *mut ffi::IVehicle,
    id: u16,
}

impl Vehicle {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn destroy(&self) {
        // TODO: baseobject validation shit
        unsafe { ffi::vehicle_destroy(self.ptr) }
    }
}

pub fn create_vehicle(
    model: ModelHash,
    x: f32,
    y: f32,
    z: f32,
    rx: f32,
    ry: f32,
    rz: f32,
) -> Vehicle {
    let ptr = unsafe { ffi::create_vehicle(model, x, y, z, rx, ry, rz) };
    let id = unsafe { ffi::vehicle_get_id(ptr) };

    Vehicle { id, ptr }
}

pub type ResourceToggleTickHandler = fn(resource: &MainResource, enabled: bool);

pub struct MainResource {
    pub path: PathBuf,
}

static RESOURCE_API: OnceCell<Arc<Mutex<resource_api::ResourceApi>>> = OnceCell::new();

impl MainResource {
    pub fn new(path: PathBuf, resource_api: Arc<Mutex<resource_api::ResourceApi>>) -> Self {
        let instance = MainResource { path };
        println!("MainResource::new call ptr: {:?}", &instance as *const _);

        RESOURCE_API.set(resource_api);

        instance
    }

    pub fn on_tick(&mut self) {
        println!("MainResource on_tick path: {}", self.path.display());
    }
}

pub fn set_interval(callback: fn(), millis: u64, test_data: TestDataContainer) {
    RESOURCE_API
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .create_timer(callback, millis, test_data);
}
