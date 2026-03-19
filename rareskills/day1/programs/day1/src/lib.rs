use anchor_lang::prelude::*;

declare_id!("AzDuzeGZXcmFgX32wufQjv27ZshZQvyvJS2wVnZxsMGB");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello from Andaru");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
