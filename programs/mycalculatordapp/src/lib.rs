use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;

    // pun fn means public function.
    // Context<T> is a required parameter to call any program in Solana. It represents the accounts array.
    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = & mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

// We need to define a Create struct. We want to pass 3 accounts.
// First: the calculator account that is used by the function itself
// Second: user account
// Third: system_program.
// In this structure we give the command to actually create the
// calculator account. This will cost SOL paid by the user account.
// We need to specify also the space parameter which tells how much
// space is required by this account. system_program is just system
// specification for the Solana blockchain.
// Use of "derive" macro to incorporate 3 accounts.
// init macro is used to create a new account owned by the current program.
// Need to also specifiy a payer for an init and the space.
// mut means the account is mutable, allowing Solana to update the data in the account.
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Definition of the calculator account, using Anchor macro to convert a struct into an account.
#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}