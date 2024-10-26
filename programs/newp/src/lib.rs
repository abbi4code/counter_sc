//imports essentials components and macros from anchor framew
use anchor_lang::prelude::*;

//this macro set the programID ie the unique address of this solana program on the blockchain
//only this specific program will controll our pdas
declare_id!("Fg6PaFpoGXkYsidMpWxqSWPJDpYF7tHrfGKXzWnV4oCH");

//The #[account] attribute is applied to a Rust struct that represents the data structure of an account.

// #[derive(Accounts)]
// This is a macro applied to a Rust struct that represents the context of the Solana program instruction.

//<-------creating a counter smart-contract------->

pub mod counter_project {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space = 8 + 8)]

    pub counter: Account<'info,Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}


