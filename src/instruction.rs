use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
	types::radium::Query
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolInstruction {
	Radium {data: Query}
}