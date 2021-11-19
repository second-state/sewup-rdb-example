pub mod modules;

use modules::{todotask, ToDoTask, TODOTASK};
use sewup::primitives::EwasmAny;
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

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

#[ewasm_main(auto)]
fn main() -> anyhow::Result<sewup::primitives::EwasmAny> {
    use sewup_derive::ewasm_input_from;
    let contract = sewup::primitives::Contract::new()?;

    match contract.get_function_selector()? {
        ewasm_fn_sig!(todotask::get) => ewasm_input_from!(contract move todotask::get),
        ewasm_fn_sig!(create_with_address) => ewasm_input_from!(contract move create_with_address),
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
        expect_output.set_id(1);
        expect_output.records[0].owner = Some(
            sewup::types::Address::from_str("8663DBF0cC68AaF37fC8BA262F2df4c666a41993").unwrap(),
        );
        ewasm_auto_assert_eq!(
            create_with_address(create_input) by "8663DBF0cC68AaF37fC8BA262F2df4c666a41993",
            expect_output
        );
    }
}
