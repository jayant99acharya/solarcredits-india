#![no_main]
extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

#[storage]
#[entrypoint]
pub struct SolarCredits {
    total_supply: StorageU256,
}

#[public]
impl SolarCredits {
    pub fn init(&mut self) {
        self.total_supply.set(U256::from(0));
    }

    pub fn mint(&mut self, amount: U256) -> U256 {
        let current = self.total_supply.get();
        let new_total = current + amount;
        self.total_supply.set(new_total);
        new_total
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn burn(&mut self, amount: U256) -> U256 {
        let current = self.total_supply.get();
        if current >= amount {
            let new_total = current - amount;
            self.total_supply.set(new_total);
            new_total
        } else {
            current
        }
    }
}



