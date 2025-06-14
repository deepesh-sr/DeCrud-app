use anchor_lang::prelude::*;
// use crate::SourceFile;
// use crate::fallback::SourceFile;

declare_id!("JCbdvjeep12RTG6B1N8XmQ8dxuoGk4PJAK5ioqUjBU2C");

#[program]
pub mod crud_app {
    use super::*;

    pub fn initialize(
        ctx: Context<GetJournalEntry>,
        title: String,
        description: String,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.set_inner(JournalState {
            owner: ctx.accounts.owner.key(),
            title,
            description,
        });
        Ok(())
    }

    pub fn update(ctx: Context<UpdateJournalEntry>, title: String, description: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.set_inner(JournalState {
            owner: ctx.accounts.owner.key(),
            title,
            description,
        });
        Ok(())
    }

    pub fn delete(ctx: Context<DeleteJournalEntry>,_title: String)->Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct GetJournalEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [title.as_bytes() , owner.key().as_ref()],
        bump,
        space = 8 + JournalState::INIT_SPACE,
    )]
    pub journal_entry: Account<'info, JournalState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateJournalEntry<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        mut,
        seeds = [title.as_bytes(),owner.key().as_ref()],
        bump,
        realloc = 8 + JournalState::INIT_SPACE,
        realloc::zero= true,
        realloc::payer = owner
    )]
    pub journal_entry : Account<'info,JournalState>,
    pub system_program : Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteJournalEntry<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        mut,
        seeds = [title.as_bytes() , owner.key().as_ref()],
        bump,
        close = owner
    )]
    pub journal_entry :Account<'info,JournalState>,
    pub system_program : Program<'info,System>
}

// It is the account sate , just say a skeleteon which will have multiple bodies over it.
#[account]
#[derive(InitSpace)]
pub struct JournalState {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub description: String,
}
