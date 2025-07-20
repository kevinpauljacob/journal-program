use anchor_lang::prelude::*;

declare_id!("CjFByKUUhHm4N4U7oekeh8cKRsHZBS8CXRM6MVr7p69G");

#[program]
pub mod journal {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateJournalEntry>, title: String, content: String) -> Result<()> {
        require!(title.len() <= 50, ErrorCode::TitleLengthExceeded);
        require!(content.len() <= 500, ErrorCode::ContentLengthExceeded);

        let journal_count = &mut ctx.accounts.journal_count;
        journal_count.count = journal_count.count.checked_add(1).ok_or(ErrorCode::InvalidJournalEntryId)?;

        journal_count.owner = ctx.accounts.owner.key();
        let journal_entry = &mut ctx.accounts.journal;
        msg!("Added new journal entry titled: {}", title);

        journal_entry.id = journal_count.count;
        journal_entry.owner = ctx.accounts.owner.key();
    
        journal_entry.title = title;
        journal_entry.content = content;
        journal_entry.created_at = Clock::get()?.unix_timestamp;
        journal_entry.updated_at = journal_entry.created_at;

        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateJournalEntry>, id: u64, title: String, content: String) -> Result<()> {
        require!(title.len() <= 50, ErrorCode::TitleLengthExceeded);
        require!(content.len() <= 500, ErrorCode::ContentLengthExceeded);
        
        let journal_entry = &mut ctx.accounts.journal;

        require!(journal_entry.id == id, ErrorCode::InvalidJournalEntryId);

        msg!("Updated journal entry titled: {}", title);
        
        journal_entry.title = title;
        journal_entry.content = content;
        journal_entry.updated_at = Clock::get()?.unix_timestamp;
      
        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteJournalEntry>, title: String) -> Result<()> {
        msg!("Deleted journal entry titled: {}", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreateJournalEntry<'info> {
    #[account(
        mut,
        seeds = [b"journal_count", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub journal_count: Account<'info, JournalCount>,
    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 32 + 8 + 8 + 4 + title.len() + 4 + content.len(),
        seeds = [b"journal", journal_count.count.to_le_bytes().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub journal: Account<'info, JournalEntry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(id: u64, title: String, content: String)]
pub struct UpdateJournalEntry<'info> {
    #[account(
        mut,
        has_one = owner,
        seeds = [b"journal", id.to_le_bytes().as_ref(), owner.key().as_ref()],
        bump,
        realloc = 8 + 8 + 32 + 8 + 8 + 4 + title.len() + 4 + content.len(),
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal: Account<'info, JournalEntry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
#[instruction(id: u64)]
pub struct DeleteJournalEntry<'info> {
    #[account(
        mut,
        seeds = [b"journal", id.to_le_bytes().as_ref(), owner.key().as_ref()],
        bump,
        close = owner,
        has_one = owner,
    )]
    pub journal: Account<'info, JournalEntry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account] 
pub struct JournalEntry {
    pub id: u64,
    pub owner: Pubkey,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub content: String,
}

#[account]
#[derive(InitSpace)]
pub struct JournalCount {
    pub count: u64,
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid journal entry ID")]
    InvalidJournalEntryId,
    #[msg("Title length exceeded the maximum limit of 50 characters")]
    TitleLengthExceeded,
    #[msg("Content length exceeded the maximum limit of 500 characters")]
    ContentLengthExceeded,
}