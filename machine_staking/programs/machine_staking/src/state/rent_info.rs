use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct RentInfo {
    pub renter: Pubkey,
    #[max_len(50)]
    pub machine_id: String,
    pub rent_start_time: i64,
    pub rent_end_time: i64,
}
