use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateOrganizationInstruction {
    pub id: i64,
    pub fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateOrganizationInstruction {
    pub fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawFromOrganizationInstruction {
    pub amount: u64,
}
