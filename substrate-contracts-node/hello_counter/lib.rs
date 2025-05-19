#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hello_counter {
    #[ink(storage)]
    pub struct HelloCounter {
        value: u32,
    }

    impl HelloCounter {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.value = self.value.wrapping_add(1);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_increment_works() {
            let mut contract = HelloCounter::new(0);
            contract.increment();
            assert_eq!(contract.get(), 1);
        }

        #[ink::test]
        fn test_initial_value() {
            let contract = HelloCounter::new(5);
            assert_eq!(contract.get(), 5);
        }

        #[ink::test]
        fn test_wrapping_overflow() {
            let mut contract = HelloCounter::new(u32::MAX);
            contract.increment();
            assert_eq!(contract.get(), 0); // wrapping_add causes overflow back to 0
        }
    }
}
