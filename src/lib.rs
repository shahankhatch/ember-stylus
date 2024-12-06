
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define some persistent storage using the Solidity ABI.
// Define the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct VolatilityContract {
        uint256[100] prices;
        uint256 volatility;
        uint256 index;
    }
}

#[public]
impl VolatilityContract {
    pub fn init(&mut self) {
        // self.volatility.set(U256::from(0));
        // for i in 0..100 {
        //     self.prices.get_mut(i).unwrap().set(U256::from(0));
        // }
        self.index.set(U256::from(0));
    }

    pub fn index(&self) -> U256 {
        self.index.get()
    }

    pub fn last_volatility(&self) -> U256 {
        let index = self.index.get();
        // return 0 volatility if we have less than 100 swaps
        if index < U256::from(100) {
            return U256::from(0);
        }
        self.volatility.get()
    }

    pub fn get_volatility(&self) -> U256 {
        self.volatility.get()
    }

    pub fn set_volatility(&mut self, new_volatility: U256) {
        self.volatility.set(new_volatility);
    }

    pub fn add_swap(&mut self, swap: U256) {
        // implement a circular buffer using last_index and swaps
        let index = self.index.get() % U256::from(100);
        self.prices.get_mut(index).unwrap().set(swap);
        self.index.set(self.index.get() + U256::from(1));
    }

    pub fn simulate_swap(&mut self, swap: U256) -> U256 {
        let mut sum: U256 = U256::from(0);
        for i in 1..99 {
            sum += self.prices.get(i).unwrap();
        }
        sum += swap;
        let mean = sum / U256::from(100);
        let mut sum_of_squares: U256 = U256::from(0);
        for i in 1..99 {
            sum_of_squares += (self.prices.get(i).unwrap() - mean).pow(U256::from(2));
        }
        sum_of_squares += (swap - mean).pow(U256::from(2));
        let variance = sum_of_squares / U256::from(100);
        let volatility = self.sqrt(variance);
        return volatility
    }

    pub fn calculate_volatility(&mut self) -> U256 {
        if self.index.get() < U256::from(100) {
            return U256::from(100);
        }
        let mut sum: U256 = U256::from(0);
        for i in 1..100 {
            sum += self.prices.get(i).unwrap();
        }
        let mean = sum / U256::from(100);
        let mut sum_of_squares: U256 = U256::from(0);
        for i in 1..100 {
            sum_of_squares += (self.prices.get(i).unwrap() - mean).pow(U256::from(2));
        }
        let variance = sum_of_squares / U256::from(100);
        let volatility = self.sqrt(variance);
        return volatility
    }

    // Even though we have access to the Rust ecosystem, sqrt is in "Solidity"
    // otherwise we need to align data types, e.g., fixed-point math
    fn sqrt(&self, value: U256) -> U256 {
        let mut z = value;
        let mut x = value / U256::from(2) + U256::from(1);
        while x < z {
            z = x;
            x = (value / x + x) / U256::from(2);
        }
        z
    }
}


