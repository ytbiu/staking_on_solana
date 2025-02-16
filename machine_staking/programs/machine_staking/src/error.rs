use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("invalid nft count, max is 20")]
    InvalidNFTCount,

    #[msg("can not end stake yet")]
    InvalidStakeEndTime,

    #[msg("machine is renting by others")]
    MachineRentingByOthers,

    #[msg("machine has ended staking")]
    MachineHasEndedStaking,

    #[msg("invalid rent fee")]
    InvalidRentFee,

    #[msg("can not end rent yet")]
    InvalidEndRentTime,
}
