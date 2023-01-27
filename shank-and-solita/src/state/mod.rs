use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize 
    },
    shank::ShankAccount,
    solana_program::pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankAccount)]
pub struct Car {
    pub year: u16,
    pub make: String,
    pub model: String,
}

impl Car {
    pub const SEED_PREFIX: &'static str = "car";
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum RentalOrderStatus {
    Created,
    PickedUp,
    Returned,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankAccount)]
pub struct RentalOrder {
    pub car: Pubkey,
    pub name: String,
    pub pick_up_date: String,
    pub return_date: String,
    pub price: u64,
    pub status: RentalOrderStatus,
}

impl RentalOrder {
    pub const SEED_PREFIX: &'static str = "rental_order";
}