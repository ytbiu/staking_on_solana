pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C2SqF49RUjhavh6g4bQBXQmChYc2GS9ogdsVU6vG6xiU");

#[program]
pub mod machine_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, reward_amount: u64) -> Result<()> {
        initialize::init(ctx, reward_amount)
    }
}
