// Add your service code

// services.rs
// necessary crates
use sails_rs::{
    prelude::*,
    gstd::msg,
};

// import the state
use crate::states::*;
use crate::services::service::state::*;

#[derive(Default)]
pub struct Service;

// Impl for seed-related function to init the state
impl Service {
    // Related function to init the service state (call only once)
    pub fn seed() {
        State::init_state();
    }
}

#[service]
impl Service {
    // Service constructor  
    pub fn new() -> Self {
        Self
    }

    // Function to handle service logic
    pub fn process_transaction(&mut self, input: TransactionInput) -> Result<Events, Errors> {
        // Add Validations
        if input.member_wallet.is_empty() {
            return Err(Errors::InvalidInput("member_wallet is empty".to_string()));
        }

        if input.amount <= 0.0 {
            return Err(Errors::InvalidInput("amount must be greater than zero".to_string()));
        }

        // Fetch on-chain data
        let balances = self.fetch_balances(&input.member_wallet)?;

        // Fetch market data
        let prices = self.fetch_market_data()?;

        // Determine best payment option
        let payment_option = self.analyze_best_currency(&balances, &prices, input.amount)?;

        // Return the recommended payment option and explanation
        Ok(Events::PaymentOptionSelected(payment_option))
    }

    pub fn fetch_balances(&self, member_wallet: &str) -> Result<Vec<TokenBalance>, Errors> {
        // Implement fetching balances from a service like Subscan
        Ok(vec![]) // Placeholder
    }

    pub fn fetch_market_data(&self) -> Result<Vec<TokenPrice>, Errors> {
        // Implement fetching market data
        Ok(vec![]) // Placeholder
    }

    pub fn analyze_best_currency(
        &self,
        balances: &[TokenBalance],
        prices: &[TokenPrice],
        amount: f64,
        model: String,
    ) -> Result<PaymentOption, Errors> {
        // Implement analysis logic using LLM or other models
        Ok(PaymentOption {
            currency: "TOKEN_X".to_string(),
            description: "Chosen for the reasons mentioned".to_string(),
        }) // Placeholder
    }

    // Queries for the state

    // Query to get all data requests
    pub fn query_data_requests(&self) -> Vec<DataRequest> {
        State::state_ref().data_requests.clone()
    }

    // Query to get a specific data request by wallet
    pub fn query_data_request_by_wallet(&self, wallet: ActorId) -> Option<DataRequest> {
        State::state_ref()
            .data_requests
            .iter()
            .find(|req| req.user_wallet == wallet)
            .cloned()
    }

    // Query for overall state
    pub fn query_state(&self) -> IoState {
        State::state_ref().to_owned().into()
    }

    // Service examples
    pub fn service_call_one(&mut self) -> Result<(), Errors> {
        // Perform some operation
        Ok(())
    }

    pub fn service_call_two(&mut self) -> Result<(), Errors> {
        // Perform some operation
        Ok(())
    }

    pub fn service_call_three(&mut self) -> Result<(), Errors> {
        // Perform some operation
        Ok(())
    }
}

// Enum to use as a response to the user
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    PaymentOptionSelected(PaymentOption),
}

// Placeholder structs for simplicity
#[derive(Clone)]
pub struct TransactionInput {
    pub member_wallet: String,
    pub amount: f64,
    pub currency: String,
    pub model: String,
}

#[derive(Clone)]
pub struct TokenBalance {
    pub currency: String,
    pub balance: f64,
}

#[derive(Clone)]
pub struct TokenPrice {
    pub currency: String,
    pub price: f64,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PaymentOption {
    pub currency: String,
    pub description: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
    InvalidInput(String),
    ApiError(String),
}