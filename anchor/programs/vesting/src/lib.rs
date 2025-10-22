#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod vesting {
    use super::*;

}


#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    pub vesting_account:Account<'info,VestingAccount>,
    pub mint:InterfaceAccount<'info,Mint>,
    pub treasury_token_account:InterfaceAccount<'info,TokenAccount>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>


}


#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info>{
    pub signer:Signer<'info>,
    pub beneficiary:SystemAccount<'info>,
    pub vesting_account:Account<'info,VestingAccount>,
    pub employee_account:Account<'info,EmployeeAccount>,
    pub system_program:Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount{
    pub owner:Pubkey,
    pub mint:Pubkey,
    pub treasury_token_account:Pubkey,

}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount{
    pub beneficiary:Pubkey,
    pub start_time:i64,
    pub end_time:i64,
    pub cliff_time:i64,
    pub total_amount:i64,
    pub total_withdrawn:i64,
    pub vesting_account:Pubkey,
    pub bump:i64,
}
