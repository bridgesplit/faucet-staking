
#[derive(Accounts)]
pub struct UpdateMember<'info> {
    #[account(mut, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}
    
pub fn handler(ctx: Context<UpdateMember>, metadata: Option<Pubkey>) -> Result<()> {
    let member = &mut ctx.accounts.member;
    if let Some(m) = metadata {
        member.metadata = m;
    }
    Ok(())
}