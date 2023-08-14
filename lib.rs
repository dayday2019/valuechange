#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod valuechange {
    use ink_prelude::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Valuechange {
        /// Stores a single `bool` value on the storage.
        value: bool,
        score: u64,
    }


    #[ink(event)]
    pub struct ScoreReturn {
        pub score: u64,
    }


    impl Valuechange {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, score: 0 }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn add_score(&mut self) {
            self.score = self.score+1;
            ink_env::debug_println!("更新后的 score: {} 区块高度: {}",self.score,self.env().block_number());
            // 通过事件返回值。对于涉及到修改合约存储项的操作，需要使用事件返回值。
            self.env().emit_event(ScoreReturn {
                score: self.score,
            });
        }

        #[ink(message)]
        pub fn get_score(&self) -> u64 {
            ink_env::debug_println!("当前 score: {} 区块高度: {}",self.score,self.env().block_number());
            self.score
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let valuechange = Valuechange::default();
            assert_eq!(valuechange.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut valuechange = Valuechange::new(false);
            assert_eq!(valuechange.get(), false);
            valuechange.flip();
            assert_eq!(valuechange.get(), true);
        }
    }
}
