// TODO: Allow users to define templates or profiles for different types of users,
// and then generate multiple users based on these templates.

// TODO: Provide an option for users to manually create a few specific users
// and then generate additional users programmatically to reach the desired number.

use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// ID for the user.
    pub id: Uuid,

    /// Balance of the user.
    pub balance: f64,
}

impl User {
    pub fn new(id: Uuid, balance: f64) -> Self {
        User { id, balance }
    }

    pub fn generate(total_users: u64, supply: u64) -> Vec<User> {
        let mut rng = rand::thread_rng();
        let mut users = vec![];

        // Generate random balances
        let mut total_balance: f64 = 0.0;
        for _ in 0..total_users {
            let balance = rng.gen_range(0.0..(supply as f64) / total_users as f64);
            total_balance += balance;

            users.push(User {
                id: Uuid::new_v4(),
                balance,
            });
        }

        // Normalize balances to ensure the total does not exceed initial_supply
        // TODO: verify that the total balance is not less and not greater than the initial supply
        let normalization_factor = (supply as f64) / total_balance;

        for user in &mut users {
            user.balance *= normalization_factor;
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
        let balance = 100.0;

        let user = User::new(id, balance);

        assert_eq!(user.id, id);
        assert_eq!(user.balance, balance);
    }

    // #[test]
    // fn test_user_generate() {
    //     let total_users = 10;
    //     let initial_supply = 1000;

    //     let users = User::generate(total_users, initial_supply);

    //     assert_eq!(users.len(), total_users as usize);

    //     let total_balance: f64 = users.iter().map(|user| user.balance).sum();
    //     assert_eq!(total_balance, initial_supply as f64);
    // }
}
