pub mod modules;

use modules::{todotask, ToDoTask, TODOTASK};
use serde_derive::{Deserialize, Serialize};
use sewup::primitives::EwasmAny;
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[derive(Serialize, Deserialize)]
pub struct Input {
    id: usize,
}

#[ewasm_constructor]
fn constructor() {
    let mut db = sewup::rdb::Db::new().expect("there is no return for constructor currently");
    db.create_table::<ToDoTask>();
    db.commit()
        .expect("there is no return for constructor currently");
}

#[ewasm_fn]
fn create_with_address(mut p: todotask::Protocol) -> Result<EwasmAny, anyhow::Error> {
    let caller = sewup::utils::caller();
    p.records[0].owner = Some(caller);
    todotask::create(p)
}

#[ewasm_fn]
fn query_with_address() -> Result<EwasmAny, anyhow::Error> {
    let caller = sewup::utils::caller();
    let table = sewup::rdb::Db::load(None)?.table::<ToDoTask>()?;
    let tasks = table.filter_records(&|t: &ToDoTask| t.owner == caller)?;

    let protocol: todotask::Protocol = tasks.into();
    if protocol.records.len() > 1 {
        sewup::ewasm_dbg!(protocol.records[0].owner.clone());
    }
    Ok(sewup::primitives::EwasmAny::from(protocol))
}

#[ewasm_fn]
fn set_task_complete(input: Input) -> Result<EwasmAny, anyhow::Error> {
    let caller = sewup::utils::caller();

    let mut table = sewup::rdb::Db::load(None)?.table::<ToDoTask>()?;

    for (id, mut task) in table.filter_records(&|t: &ToDoTask| t.owner == caller)? {
        if id == input.id {
            task.completed = true;
            table.update_record(id, Some(task.into()))?;
            table.commit()?;
            return Ok(().into());
        }
    }

    Err(anyhow::anyhow!("ToDoTask Not Found"))
}

#[ewasm_main(auto)]
fn main() -> anyhow::Result<sewup::primitives::EwasmAny> {
    use sewup_derive::ewasm_input_from;
    let contract = sewup::primitives::Contract::new()?;

    match contract.get_function_selector()? {
        ewasm_fn_sig!(query_with_address) => query_with_address(),
        ewasm_fn_sig!(create_with_address) => ewasm_input_from!(contract move create_with_address),
        ewasm_fn_sig!(set_task_complete) => ewasm_input_from!(contract move set_task_complete),
        ewasm_fn_sig!(todotask::update) => ewasm_input_from!(contract move todotask::update),
        ewasm_fn_sig!(todotask::delete) => ewasm_input_from!(contract move todotask::delete),
        _ => return Err(anyhow::anyhow!("Unknow Error")),
    }
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup_derive::{ewasm_assert_eq, ewasm_assert_ok, ewasm_auto_assert_eq, ewasm_err_output};

    #[ewasm_test]
    fn test_handler() {
        let todo_task = ToDoTask::default();
        let create_input = todotask::protocol(todo_task);
        let mut expect_output = create_input.clone();
        let mut expect_empty_output = create_input.clone();

        expect_output.set_id(1);
        expect_output.records[0].owner = Some(
            sewup::types::Address::from_str("8663DBF0cC68AaF37fC8BA262F2df4c666a41993").unwrap(),
        );
        ewasm_auto_assert_eq!(
            create_with_address(create_input) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            expect_output
        );

        expect_empty_output.records = vec![];
        ewasm_auto_assert_eq!(query_with_address(), expect_empty_output);

        ewasm_auto_assert_eq!(
            query_with_address() by  "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            expect_output
        );

        ewasm_assert_ok!(set_task_complete(Input { id: 1 }) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993");

        expect_output.records[0].completed = Some(true);
        ewasm_auto_assert_eq!(
            query_with_address() by  "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            expect_output
        );
    }
}
