#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod psp22 {
    use ink::storage::Mapping;
    use ink_prelude::string::String;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22Error {
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

    #[ink(event)]
    pub struct Transferred {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
        data: String,
    }

    #[ink(storage)]
    pub struct Psp22 {
        /// The super user is the holder of all the tokens
        pub super_user: AccountId,
        /// Total token supply
        pub total_supply: Balance,
        pub balances: Mapping<AccountId, Balance>,
        pub allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    impl Psp22 {
        /// Initializes the token supply
        #[ink(constructor)]
        pub fn new(init_supply: Balance, super_user: AccountId) -> Self {
            Self { total_supply: init_supply, super_user: super_user, balances: Default::default(), allowances: Default::default() }
        }

        /// Returns the total token supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`
        /// Returns `0` if the account is non-existent
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        //  "Returns the amount which `spender` is still allowed to withdraw from `owner`.",
        //  "Returns `0` if no allowance has been set."
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }

        //  "Transfers `value` amount of tokens from the caller's account to account `to`",
        //  "with additional `data` in unspecified format.",
        //  "",
        //  "On success a `Transfer` event is emitted.",
        //  "",
        //  "# Errors",
        //  "",
        //  "Reverts with error `InsufficientBalance` if there are not enough tokens on",
        //  "the caller's account Balance.",
        //  "",
        //  "Reverts with error `ZeroSenderAddress` if sender's address is zero.",
        //  "",
        //  "Reverts with error `ZeroRecipientAddress` if recipient's address is zero."
        //  "Reverts with error `SafeTransferCheckFailed` if the recipient is a contract and
        //  rejected the transfer."
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance, data: String) -> Result<(), PSP22Error> {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            if sender_balance <= value {
                return Err(PSP22Error::InsufficientBalance)
            }

            if sender == AccountId::from([0u8; 32]) {
                return Err(PSP22Error::ZeroSenderAddress)
            }

            if to == AccountId::from([0u8; 32]) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }

            if self.env().is_contract(&to) {
                return Err(PSP22Error::SafeTransferCheckFailed(format!("AccountId {:?} is contract", &to)))
            }

            let recipient_balance = self.balance_of(to);

            self.balances.insert(sender, &(sender_balance - value));
            self.balances.insert(to, &(recipient_balance + value));

            self.env().emit_event(
                Transferred {
                    from: Some(sender),
                    to: Some(to),
                    value,
                    data,
                }
            );

            Ok(())
        }
    }
}
