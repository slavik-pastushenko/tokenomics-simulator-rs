//! # User module
//!
//! This module provides functionality to create and manage users in the tokenomics simulator.
//! Users are entities that interact with the tokenomics system by buying, selling, and holding tokens.

use rand::Rng;
use rust_decimal::{prelude::*, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// ID for the user.
    pub id: Uuid,

    /// Balance of the user.
    pub balance: Decimal,

    /// Market behaviour of the user.
    pub behaviour: UserBehaviour,
}

/// Market behaviour of the user.
#[derive(Debug, Deserialize, Serialize)]
pub enum UserBehaviour {
    /// Speculator: Users who buy and sell tokens frequently to make a profit.
    Speculator,

    /// Holder: Users who buy tokens and hold them for a long time.
    Holder,

    /// Trader: Users who trade tokens frequently but do not hold them for long.
    Trader,
}

impl User {
    /// Create a new user.
    ///
    /// # Arguments
    ///
    /// * `id` - ID for the user.
    /// * `balance` - Balance of the user.
    ///
    /// # Returns
    ///
    /// New user.
    pub fn new(id: Uuid, balance: Decimal) -> Self {
        User {
            id,
            balance,
            behaviour: UserBehaviour::Trader,
        }
    }

    /// Generate a list of users with random balances.
    ///
    /// # Arguments
    ///
    /// * `total_users` - Total number of users to generate.
    /// * `supply` - Initial supply of the token.
    /// * `price` - Initial price of the token.
    /// * `decimals` - Number of decimal places for the token.
    ///
    /// # Returns
    ///
    /// List of users with random balances.
    pub fn generate(total_users: u64, supply: Decimal, price: Decimal, decimals: u32) -> Vec<User> {
        let mut rng = rand::rng();
        let mut users = vec![];

        let mut total_balance = Decimal::default();
        for _ in 0..total_users {
            let balance = Decimal::from_f64(
                rng.random_range(
                    0.0..(supply / Decimal::new(total_users as i64, 0))
                        .to_f64()
                        .unwrap(),
                ),
            )
            .unwrap()
            .round_dp(decimals);
            total_balance += balance;

            users.push(User {
                id: Uuid::new_v4(),
                balance,
                behaviour: UserBehaviour::Trader,
            });
        }

        // Normalize balances to ensure the total does not exceed initial supply
        let normalization_factor = supply / total_balance;
        for user in &mut users {
            user.balance *= normalization_factor;
            user.balance = user.balance.round_dp(decimals);
        }

        // Adjust balances based on the initial price
        for user in &mut users {
            user.balance *= price;
            user.balance = user.balance.round_dp(decimals);
        }

        // Distribute any remaining balance to ensure total balance matches initial supply
        let mut remaining_balance = supply - users.iter().map(|u| u.balance).sum::<Decimal>();
        for user in &mut users {
            if remaining_balance.is_zero() {
                break;
            }

            let add_balance = Decimal::min(remaining_balance, Decimal::new(1, 4));
            user.balance += add_balance;
            remaining_balance -= add_balance;
        }

        users
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let id = Uuid::new_v4();
        let balance = Decimal::new(100, 0);

        let user = User::new(id, balance);

        assert_eq!(user.id, id);
        assert_eq!(user.balance, balance);
    }

    #[test]
    fn test_user_generate() {
        let total_users = 10;
        let decimals = 4;
        let initial_supply = Decimal::new(1000, 0);
        let initial_price = Decimal::new(1, 0);

        let users = User::generate(total_users, initial_supply, initial_price, decimals);

        assert_eq!(users.len(), total_users as usize);

        let total_balance = users
            .iter()
            .map(|user| user.balance.round_dp(decimals))
            .sum::<Decimal>();

        assert_eq!(total_balance, initial_supply);
    }
}
