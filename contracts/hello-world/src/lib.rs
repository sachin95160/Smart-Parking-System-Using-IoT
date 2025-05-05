#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone, Copy)]
pub enum SlotStatus {
    Available,
    Booked,
}

#[contracttype]
#[derive(Clone)]
pub struct ParkingSlot {
    pub slot_id: u64,
    pub status: SlotStatus,
}

#[contracttype]
pub enum SlotKey {
    Slot(u64),
}

const SLOT_COUNT: Symbol = symbol_short!("S_COUNT");

#[contract]
pub struct SmartParkingContract;

#[contractimpl]
impl SmartParkingContract {
    pub fn create_slot(env: Env) -> u64 {
        let mut count = env.storage().instance().get(&SLOT_COUNT).unwrap_or(0);
        count += 1;

        let new_slot = ParkingSlot {
            slot_id: count,
            status: SlotStatus::Available,
        };

        env.storage().instance().set(&SlotKey::Slot(count), &new_slot);
        env.storage().instance().set(&SLOT_COUNT, &count);
        log!(&env, "Created parking slot with ID: {}", count);
        count
    }

    pub fn book_slot(env: Env, slot_id: u64) {
        let key = SlotKey::Slot(slot_id);
        let mut slot = env.storage().instance().get(&key).expect("Slot not found");

        match slot.status {
            SlotStatus::Available => {
                slot.status = SlotStatus::Booked;
                env.storage().instance().set(&key, &slot);
                log!(&env, "Slot {} successfully booked", slot_id);
            },
            SlotStatus::Booked => {
                panic!("Slot already booked");
            },
        }
    }

    pub fn cancel_booking(env: Env, slot_id: u64) {
        let key = SlotKey::Slot(slot_id);
        let mut slot = env.storage().instance().get(&key).expect("Slot not found");

        match slot.status {
            SlotStatus::Booked => {
                slot.status = SlotStatus::Available;
                env.storage().instance().set(&key, &slot);
                log!(&env, "Booking for slot {} cancelled", slot_id);
            },
            SlotStatus::Available => {
                panic!("Slot is not booked");
            },
        }
    }

    pub fn view_slot_status(env: Env, slot_id: u64) -> SlotStatus {
        let key = SlotKey::Slot(slot_id);
        env.storage().instance().get(&key).expect("Slot not found").status
    }
}
