pub mod modules;

use modules::{todotask, ToDoTask, TODOTASK};
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {}

#[ewasm_fn]
fn handler() -> anyhow::Result<sewup::primitives::EwasmAny> {
    Ok(().into())
}

#[ewasm_main(auto)]
fn main() -> anyhow::Result<sewup::primitives::EwasmAny> {
    use sewup_derive::ewasm_input_from;
    let contract = sewup::primitives::Contract::new()?;

    match contract.get_function_selector()? {
        ewasm_fn_sig!(handler) => handler(),
        _ => return Err(anyhow::anyhow!("Unknow Error")),
    }
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup_derive::{ewasm_assert_eq, ewasm_assert_ok, ewasm_auto_assert_eq, ewasm_err_output};

    #[ewasm_test]
    fn test_handler() {
        ewasm_auto_assert_eq!(handler(), ());
    }
}
