use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signer_authorization_secure {
    use super::*;

    pub fn log_message(
        ctx: Context<LogMessage>,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,
}

fn main() {}
