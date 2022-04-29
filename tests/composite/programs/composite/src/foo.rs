use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Foo<'info> {
    #[account(mut)]
    pub dummy_a: Account<'info, DummyA>,
}

#[account]
pub struct DummyA {
    pub data: u64,
}
