use anchor_lang::prelude::*;

declare_id!("HwdgXMKDrmMfqtKR1aWJqwsEExTHe4fJJB8XDP3SAyRU");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwaysError);
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("Always errors")]
    AlwaysError,
}
