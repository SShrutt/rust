#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct OwnershipRecord {
    pub owner: Address,
    pub timestamp: u64,
    pub remarks: String,
}

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub property_id: u64,
    pub current_owner: Address,
    pub history: Vec<OwnershipRecord>,
}

#[contracttype]
pub enum PropertyKey {
    Property(u64),
    Count,
}

#[contract]
pub struct PropertyHistoryLedger;

#[contractimpl]
impl PropertyHistoryLedger {
    // Register a new property
    pub fn register_property(env: Env, owner: Address, remarks: String) -> u64 {
        let mut count = env.storage().instance().get(&PropertyKey::Count).unwrap_or(0);
        count += 1;

        let mut history = Vec::new(&env);
        let record = OwnershipRecord {
            owner: owner.clone(),
            timestamp: env.ledger().timestamp(),
            remarks,
        };
        history.push_back(record);

        let property = Property {
            property_id: count,
            current_owner: owner,
            history,
        };

        env.storage().instance().set(&PropertyKey::Property(count), &property);
        env.storage().instance().set(&PropertyKey::Count, &count);

        count
    }

    // Transfer ownership
    pub fn transfer_property(env: Env, property_id: u64, new_owner: Address, remarks: String) {
        let mut property: Property = env
            .storage()
            .instance()
            .get(&PropertyKey::Property(property_id))
            .expect("Property not found");

        let new_record = OwnershipRecord {
            owner: new_owner.clone(),
            timestamp: env.ledger().timestamp(),
            remarks,
        };
        property.history.push_back(new_record);
        property.current_owner = new_owner;

        env.storage().instance().set(&PropertyKey::Property(property_id), &property);
    }

    // Get property details
    pub fn get_property(env: Env, property_id: u64) -> Property {
        env.storage()
            .instance()
            .get(&PropertyKey::Property(property_id))
            .expect("Property not found")
    }
}
