use crate::{
    events::{self, Event, PublicEventType},
    timers, vehicle,
};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

// these statics should not be used directly in altv_module,
// but via ResourceImpl instance
static TIMER_MANAGER_INSTANCE: OnceCell<Mutex<timers::TimerManager>> = OnceCell::new();
static TIMER_SCHEDULE_STATE_INSTANCE: OnceCell<Mutex<timers::ScheduleState>> = OnceCell::new();
static EVENT_MANAGER_INSTANCE: OnceCell<Mutex<events::EventManager>> = OnceCell::new();

macro_rules! init_static {
    ($static_var: path, $manager: ty) => {{
        $static_var.set(Mutex::new(<$manager>::new())).unwrap();
        $static_var.get().unwrap()
    }};
}

// intended for altv_module
#[derive(Debug)]
pub struct ResourceImpl {
    pub full_main_path: String,
    timers: &'static Mutex<timers::TimerManager>,
    timer_schedule_state: &'static Mutex<timers::ScheduleState>,
    events: &'static Mutex<events::EventManager>,
    vehicles: &'static Mutex<vehicle::VehicleManager>,
    vehicle_creation: &'static Mutex<vehicle::PendingVehicleCreation>,
    vehicle_deletion: &'static Mutex<vehicle::PendingVehicleDeletion>,
}

impl ResourceImpl {
    pub fn init(full_main_path: String) -> ResourceImpl {
        let instance = ResourceImpl {
            full_main_path,
            timers: init_static!(TIMER_MANAGER_INSTANCE, timers::TimerManager),
            timer_schedule_state: init_static!(
                TIMER_SCHEDULE_STATE_INSTANCE,
                timers::ScheduleState
            ),
            events: init_static!(EVENT_MANAGER_INSTANCE, events::EventManager),
            vehicles: init_static!(vehicle::VEHICLE_MANAGER_INSTANCE, vehicle::VehicleManager),
            vehicle_creation: init_static!(
                vehicle::VEHICLE_CREATION_INSTANCE,
                vehicle::PendingVehicleCreation
            ),
            vehicle_deletion: init_static!(
                vehicle::VEHICLE_DELETION_INSTANCE,
                vehicle::PendingVehicleDeletion
            ),
        };

        instance
    }

    pub fn __on_sdk_event(
        &self,
        event_type: altv_sdk::EventType,
        event: *const altv_sdk::ffi::CEvent,
    ) {
        self.events
            .try_lock()
            .unwrap()
            .on_sdk_event(event_type, event);
    }

    pub fn __on_tick(&self) {
        self.timers
            .try_lock()
            .unwrap()
            .process_timers(self.timer_schedule_state.try_lock().unwrap());
    }

    pub fn __on_vehicle_create(&self, vehicle: *mut altv_sdk::ffi::IVehicle) {
        if self.vehicle_creation.try_lock().is_err() {
            crate::log_warn!("__on_vehicle_create pending creation, skip");
            return;
        }
        self.vehicles.try_lock().unwrap().on_vehicle_create(vehicle);
    }

    pub fn __on_vehicle_destroy(&self, vehicle: *mut altv_sdk::ffi::IVehicle) {
        if self.vehicle_deletion.try_lock().is_err() {
            crate::log_warn!("__on_vehicle_destroy pending deletion, skip");
            return;
        }
        self.vehicles
            .try_lock()
            .unwrap()
            .on_vehicle_destroy(vehicle);
    }
}

pub fn timers_create(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    let state = TIMER_SCHEDULE_STATE_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap();

    timers::create(state, callback, millis, once);
}

pub fn create_timer(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    let state = TIMER_SCHEDULE_STATE_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap();

    timers::create(state, callback, millis, once);
}

pub fn add_event_handler(
    public_type: PublicEventType,
    sdk_type: altv_sdk::EventType,
    event: Event,
) {
    EVENT_MANAGER_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .add_handler(public_type, sdk_type, event);
}
