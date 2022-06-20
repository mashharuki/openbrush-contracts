// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::psp35::BalancesManager;
pub use crate::{
    psp35::*,
    traits::psp35::extensions::enumerable::*,
};
pub use derive::PSP35EnumerableStorage;
use openbrush::{
    declare_storage_trait,
    storage::{
        Mapping,
        MultiMapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
    },
};

pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("openbrush::PSP35EnumerableData");

#[derive(Default, Debug)]
#[openbrush::storage(STORAGE_KEY)]
pub struct EnumerableBalances {
    pub enumerable: MultiMapping<Option<AccountId>, Id, EnumerableKey>,
    pub balances: Mapping<(AccountId, Id), Balance, BalancesKey>,
    pub total_supply: Mapping<Id, Balance>,
    pub _reserved: Option<()>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub struct BalancesKey;

impl<'a> TypeGuard<'a> for BalancesKey {
    type Type = &'a (&'a AccountId, &'a Id);
}

declare_storage_trait!(PSP35EnumerableBalancesStorage);

impl BalancesManager for EnumerableBalances {
    fn balance_of(&self, owner: &AccountId, id: &Id) -> Balance {
        self.balances.get(&(owner, id)).unwrap_or(0)
    }

    fn increase_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, mint: bool) -> Result<(), PSP35Error> {
        let initial_balance = self.balance_of(owner, id);
        self.balances
            .insert(&(owner, id), &(initial_balance.checked_add(amount.clone()).unwrap()));

        if initial_balance == 0 {
            self.enumerable.insert(&Some(owner), id);
        }

        if mint {
            let token_supply = self.total_supply.get(id).unwrap_or(0);

            self.total_supply
                .insert(id, &(token_supply.checked_add(amount.clone()).unwrap()));

            if token_supply == 0 {
                self.enumerable.insert(&None, id);
            }
        }
        Ok(())
    }

    fn decrease_balance(&mut self, owner: &AccountId, id: &Id, amount: &Balance, burn: bool) -> Result<(), PSP35Error> {
        let initial_balance = self.balance_of(owner, id);
        self.balances.insert(
            &(owner, id),
            &(initial_balance
                .checked_sub(amount.clone())
                .ok_or(PSP35Error::InsufficientBalance)?),
        );

        if initial_balance == *amount {
            self.enumerable.remove_value(&Some(owner), id);
        }

        if burn {
            let token_supply = self.total_supply.get(id).unwrap_or(0);
            self.total_supply.insert(
                id,
                &(token_supply
                    .checked_sub(amount.clone())
                    .ok_or(PSP35Error::InsufficientBalance)?),
            );

            if token_supply == *amount {
                self.enumerable.remove_value(&None, id);
            }
        }
        Ok(())
    }
}

impl<T> PSP35EnumerableBalancesStorage for T
where
    T: PSP35Storage<Data = PSP35Data<EnumerableBalances>>,
{
    type Data = EnumerableBalances;

    fn get(&self) -> &Self::Data {
        &self.get().balances
    }

    fn get_mut(&mut self) -> &mut Self::Data {
        &mut self.get_mut().balances
    }
}

impl<T: PSP35EnumerableBalancesStorage<Data = EnumerableBalances> + PSP35> PSP35Enumerable for T {
    default fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP35Error> {
        self.get()
            .enumerable
            .get_value(&Some(&owner), &index)
            .ok_or(PSP35Error::TokenNotExists)
    }

    default fn token_by_index(&self, index: u128) -> Result<Id, PSP35Error> {
        self.get()
            .enumerable
            .get_value(&None, &index)
            .ok_or(PSP35Error::TokenNotExists)
    }
}
