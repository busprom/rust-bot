use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
	types::radium::{Query, Sell}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolInstruction {
	Radium {data: Query},
	RadiumTest {data: Query},
	Sell {data: Sell}
}