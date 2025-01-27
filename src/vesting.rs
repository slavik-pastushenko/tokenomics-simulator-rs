use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VestingSchedule {
    /// Percentage of tokens allocated (total for this schedule).
    pub allocation_percentage: Decimal,

    /// List of cliffs with their percentages and durations or timestamps.
    /// For start percentage allocation just set duration to 0.
    pub cliffs: Vec<VestingCliff>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VestingCliff {
    /// Percentage of tokens unlocked at the end of this cliff.
    pub allocation_percentage: Decimal,

    /// Duration of the cliff in seconds.
    pub duration: u64,
}

impl VestingSchedule {
    /// Calculate unlocked tokens based on the vesting cliffs and allocation percentage.
    ///
    /// # Arguments
    ///
    /// * `total_tokens` - The current number of tokens.
    /// * `elapsed_time` - The seconds elapsed since vesting started.
    ///
    /// # Returns
    ///
    /// The number of unlocked tokens.
    pub fn calculate_unlocked_tokens(&self, total_tokens: Decimal, elapsed_time: u64) -> Decimal {
        let mut unlocked_tokens = Decimal::from(0);
        let mut cumulative_duration = 0;
        let allocated_tokens = self.allocation_percentage * total_tokens;

        for cliff in &self.cliffs {
            cumulative_duration += cliff.duration;

            if elapsed_time >= cumulative_duration {
                // Calculate the tokens unlocked by this cliff
                let tokens_for_cliff = allocated_tokens * cliff.allocation_percentage;
                unlocked_tokens += tokens_for_cliff;
            } else {
                // Stop, elapsed time did not reach the next cliffs
                break;
            }
        }

        unlocked_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_vesting_schedule() -> VestingSchedule {
        VestingSchedule {
            cliffs: vec![
                VestingCliff {
                    duration: 3600,
                    allocation_percentage: Decimal::new(25, 2),
                },
                VestingCliff {
                    duration: 3600,
                    allocation_percentage: Decimal::new(25, 2),
                },
                VestingCliff {
                    duration: 3600,
                    allocation_percentage: Decimal::new(25, 2),
                },
                VestingCliff {
                    duration: 3600,
                    allocation_percentage: Decimal::new(25, 2),
                },
            ],
            allocation_percentage: Decimal::new(1, 0),
        }
    }

    #[test]
    fn test_after_first_cliff() {
        let vesting_schedule = create_vesting_schedule();
        let total_tokens = Decimal::from(1000);
        let unlocked_tokens = vesting_schedule.calculate_unlocked_tokens(total_tokens, 3600);
        assert_eq!(unlocked_tokens, Decimal::from(250));
    }

    #[test]
    fn test_after_second_cliff() {
        let vesting_schedule = create_vesting_schedule();
        let total_tokens = Decimal::from(1000);
        let unlocked_tokens = vesting_schedule.calculate_unlocked_tokens(total_tokens, 3600 * 2);
        assert_eq!(unlocked_tokens, Decimal::from(500));
    }

    #[test]
    fn test_after_third_cliff() {
        let vesting_schedule = create_vesting_schedule();
        let total_tokens = Decimal::from(1000);
        let unlocked_tokens = vesting_schedule.calculate_unlocked_tokens(total_tokens, 3600 * 3);
        assert_eq!(unlocked_tokens, Decimal::from(750));
    }

    #[test]
    fn test_after_fourth_cliff() {
        let vesting_schedule = create_vesting_schedule();
        let total_tokens = Decimal::from(1000);
        let unlocked_tokens = vesting_schedule.calculate_unlocked_tokens(total_tokens, 3600 * 4);
        assert_eq!(unlocked_tokens, Decimal::from(1000));
    }

    #[test]
    fn test_halfway_between_cliffs() {
        let vesting_schedule = create_vesting_schedule();
        let total_tokens = Decimal::from(1000);
        let unlocked_tokens =
            vesting_schedule.calculate_unlocked_tokens(total_tokens, (3600.0 * 3.5) as u64);
        assert_eq!(unlocked_tokens, Decimal::from(750));
    }
}
