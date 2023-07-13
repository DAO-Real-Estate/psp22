#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod psp22 {

    use ink_prelude::string::String;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    enum PSP22Error {
    /// Custom error type for cases in which an implementation adds its own restrictions.
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if a safe transfer check fails (e.g. if the receiving contract does not accept tokens).
    SafeTransferCheckFailed(String),
    }

    #[ink(storage)]
    pub struct Psp22 {
        /// Total token supply
        total_token_supply: Balance,
        /// The super user is the holder of all the tokens
        super_user: AccountId, 
    }

    impl Psp22 {
        /// Initializes the token supply
        #[ink(constructor)]
        pub fn new(init_total_token_supply: Balance) -> Self {
            let caller = Self::env().caller();
            Self { total_token_supply: init_total_token_supply, super_user: caller}
        }

        /// Get the total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_token_supply
        }
    }

}
