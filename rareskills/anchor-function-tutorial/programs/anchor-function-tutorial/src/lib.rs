use anchor_lang::prelude::*;

declare_id!("7dXGwevo9p9gwGty7xKJ89N8EQePwwyoUwQHvXDAB1B");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn function_a(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(ctx: Context<Empty>, firstArg: u64) -> Result<()> {
        Ok(())
    }

    // pub fn non_empty_account_example(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
    //     // msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    // pub fn boaty_mc_boatface(ctx: Context<Empty>) -> Result<()> {
    //     // msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    // pub fn add(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
    //     let sum = a + b;
    //     msg!("Sum is {}", sum);
    //     Ok(())
    // }

    // pub fn sub(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
    //     let difference = a - b;
    //     msg!("Difference is {}", difference);
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
