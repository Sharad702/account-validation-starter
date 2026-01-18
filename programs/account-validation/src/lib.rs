use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_validation {
    use super::*;

    /// Create a user profile with proper validation
    /// TODO: Implement with full account validation
    /// - Verify PDA derivation
    /// - Ensure rent-exemption
    /// - Set proper account discriminator
    pub fn create_profile(
        _ctx: Context<CreateProfile>,
        _username: String,
    ) -> Result<()> {
        // TODO: Implement this function
        // 1. Validate username length
        // 2. Initialize profile data
        // 3. Anchor handles rent-exemption
        Ok(())
    }

    /// Update user profile with ownership check
    /// TODO: Implement with authority validation
    /// - Only profile owner can update
    /// - Validate account hasn't been substituted
    pub fn update_profile(
        _ctx: Context<UpdateProfile>,
        _username: String,
    ) -> Result<()> {
        // TODO: Implement this function
        // 1. Verify caller is owner
        // 2. Update profile data
        Ok(())
    }

    /// Close profile and return rent
    /// TODO: Implement safe account closure
    /// - Return lamports to owner
    /// - Zero out account data
    /// - Handle race conditions
    pub fn close_profile(_ctx: Context<CloseProfile>) -> Result<()> {
        // TODO: Implement this function
        // Anchor's `close` constraint handles most of this
        Ok(())
    }

    /// Transfer profile ownership
    /// TODO: Implement with proper validation
    /// - Current owner must sign
    /// - New owner must be valid pubkey
    pub fn transfer_ownership(
        _ctx: Context<TransferOwnership>,
        _new_owner: Pubkey,
    ) -> Result<()> {
        // TODO: Implement this function
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CreateProfile<'info> {
    // TODO: Add proper account constraints
    // Hints:
    // - Use #[account(init, pda, space = ...)] for profile
    // - Calculate space correctly for rent-exemption
    // - Validate username in seeds
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // TODO: Add profile account with PDA
    // #[account(
    //     init,
    //     payer = authority,
    //     space = 8 + Profile::INIT_SPACE,
    //     seeds = [b"profile", authority.key().as_ref()],
    //     bump
    // )]
    // pub profile: Account<'info, Profile>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    // TODO: Add validation constraints
    // Hints:
    // - Verify authority matches profile.owner
    // - Use has_one constraint
    
    pub authority: Signer<'info>,
    
    // TODO: Add profile account with owner check
    // #[account(
    //     mut,
    //     has_one = authority @ ValidationError::Unauthorized,
    //     seeds = [b"profile", authority.key().as_ref()],
    //     bump = profile.bump
    // )]
    // pub profile: Account<'info, Profile>,
}

#[derive(Accounts)]
pub struct CloseProfile<'info> {
    // TODO: Add close constraints
    // Hints:
    // - Use close = authority to return lamports
    // - Verify ownership before closing
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    // TODO: Add profile with close constraint
    // #[account(
    //     mut,
    //     close = authority,
    //     has_one = authority,
    //     seeds = [b"profile", authority.key().as_ref()],
    //     bump = profile.bump
    // )]
    // pub profile: Account<'info, Profile>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    // TODO: Add transfer constraints
    // Only current owner can transfer
    
    pub authority: Signer<'info>,
    
    // TODO: Add profile account
}

// TODO: Define the Profile account structure
// Use InitSpace derive for automatic space calculation
// #[account]
// #[derive(InitSpace)]
// pub struct Profile {
//     pub authority: Pubkey,
//     #[max_len(32)]
//     pub username: String,
//     pub bump: u8,
//     pub created_at: i64,
// }

#[error_code]
pub enum ValidationError {
    #[msg("Unauthorized: You don't own this profile")]
    Unauthorized,
    #[msg("Username too long (max 32 characters)")]
    UsernameTooLong,
    #[msg("Username cannot be empty")]
    UsernameEmpty,
    #[msg("Invalid account owner")]
    InvalidOwner,
}
