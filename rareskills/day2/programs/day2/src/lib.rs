use anchor_lang::prelude::*;

declare_id!("Fi8Z46H2FTf9VHEdeaX7bNZKwpf3u2G3dDQNW3YqypHs");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
