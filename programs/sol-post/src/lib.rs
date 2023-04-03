use anchor_lang::prelude::*;

use solana_program::{
    system_program
};

declare_id!("A8LZ1pRNQe7cpLCUEdtE9WG1yARR1vmeyiLD4S4FsZq8");

#[program]
pub mod sol_post {
    use super::*;

    pub fn create_post(ctx: Context<CreatePost>, text: String) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;

        // Initialize the post account
        post_account.author = *ctx.accounts.author.key;
        post_account.text = text;

        // Get the generated bump value
        let (_, bump) = Pubkey::find_program_address(
            &[
                b"post_account",
                ctx.accounts.author.key.as_ref(),
            ],
            ctx.program_id,
        );

        post_account.bump = bump;
        post_account.is_initialized = true;

        Ok(())
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;

        // Ensure the author of the post is the same as the one trying to delete it
        require!(
            post_account.author == *ctx.accounts.author.key,
            ValidationErrorCode::Unauthorized
        );

        // Ensure the post account is initialized
        require!(
            post_account.is_initialized == true,
            ValidationErrorCode::InvalidAccountState
        );

        // Clear the post data
        post_account.text = String::new();
        post_account.bump = 0;
        post_account.author = Pubkey::default();
        post_account.is_initialized = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
    init_if_needed,
    payer=author,
    space=1000,
    seeds=[b"post_account", author.key.as_ref()],
    bump
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(address=system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
    mut,
    seeds=[b"post_account", author.key.as_ref()],
    bump
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(address=system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PostAccount {
    pub author: Pubkey,
    pub text: String,
    pub bump: u8,
    pub is_initialized: bool
}

#[error_code]
pub enum ValidationErrorCode {
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Invalid Account State.")]
    InvalidAccountState
}
