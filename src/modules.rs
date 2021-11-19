use serde_derive::{Deserialize, Serialize};
use sewup::types::Address;
use sewup::types::Raw;
use sewup_derive::{SizedString, Table};

#[derive(Table, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToDoTask {
    pub completed: bool,
    pub content: SizedString!(50),
    pub owner: Address,
}
