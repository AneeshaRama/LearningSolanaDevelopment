use anchor_lang::prelude::*;

declare_id!("9CQ6LCyfAKcvQ8F3L8iTjYSi5dG2TLZ9zWgCK5X5VBUV");

#[program]
pub mod counter_app {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.value = 1;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.value += 1;
        msg!("The current value: {}", counter_account.value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=64)]
    pub counter_account: Account<'info, CounterAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub value: u32,
}
