pub mod modules;

use modules::{todotask, ToDoTask, TODOTASK};
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {
    let mut db = sewup::rdb::Db::new().expect("there is no return for constructor currently");
    db.create_table::<ToDoTask>();
    db.commit()
        .expect("there is no return for constructor currently");
}

#[ewasm_main(auto)]
fn main() -> anyhow::Result<sewup::primitives::EwasmAny> {
    use sewup_derive::ewasm_input_from;
    let contract = sewup::primitives::Contract::new()?;

    match contract.get_function_selector()? {
        ewasm_fn_sig!(todotask::get) => ewasm_input_from!(contract move todotask::get),
        ewasm_fn_sig!(todotask::create) => ewasm_input_from!(contract move todotask::create),
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
        assert!(true);
    }
}
