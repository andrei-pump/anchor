use crate::MyEvent;
use anchor_lang::prelude::*;

pub mod hehe {
    use anchor_lang::Accounts;

    #[derive(Accounts)]
    pub struct Initialize2 {}
}

pub fn init_event() -> Result<()> {
    emit!(MyEvent {
        data: 5,
        label: "hello".to_string(),
    });
    Ok(())
}
