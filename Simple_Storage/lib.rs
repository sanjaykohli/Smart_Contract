use ink;

#[ink::contract]
mod simple_storage {
    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct SimpleStorage {
        /// A simple value stored in the contract.
        value: u32,
    }

    impl SimpleStorage {
        /// Constructor that initializes the `value` to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `value` to `0`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0 }
        }

        /// A message that returns the current value.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        /// A message that lets you increment the value.
        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;
        }

        /// A message that lets you set a new value.
        #[ink(message)]
        pub fn set(&mut self, new_value: u32) {
            self.value = new_value;
        }
    }

    /// Unit tests in Rust are normally defined within such a `tests` module
    /// and test the public interfaces of the contract.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let contract = SimpleStorage::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn increment_works() {
            let mut contract = SimpleStorage::new(5);
            contract.increment();
            assert_eq!(contract.get(), 6);
        }

        #[ink::test]
        fn set_works() {
            let mut contract = SimpleStorage::new(5);
            contract.set(10);
            assert_eq!(contract.get(), 10);
        }
    }
}