// Add your state code
// State.rs
// necessary crates
use sails_rs::{
    prelude::*,
};

// static mut variable (contract's state)
pub static mut STATE: Option<State> = None;

// Create a struct for the state
#[derive(Clone, Default)]
pub struct State {
    pub data_requests: Vec<DataRequest>,
}

// Struct to represent a data request
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct DataRequest {
    pub user_wallet: ActorId,
    pub amount: f64,
    pub currency: String,
    pub model: String,
}

// Impl to set methods or related functions
impl State {
    // Method to create a new instance
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Related function to init the state
    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        };
    }

    // Related function to get the state as mut
    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Related function to get the state as ref
    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Add a new data request
    pub fn add_data_request(&mut self, request: DataRequest) {
        self.data_requests.push(request);
    }
}

// Create a struct that can be sent to the user who reads the state
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub data_requests: Vec<DataRequest>,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
}

// Implement conversion from State to IoState
impl From<State> for IoState {
    fn from(value: State) -> Self {
        Self {
            data_requests: value.data_requests,
        }
    }
}