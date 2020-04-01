use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use mice::{ExpressionResult, builder::{RollBuilder, Roll}};
use rand::{SeedableRng, rngs::StdRng};

/// JavaScript binding for `mice::roll`.
// thread_rng isn't supported on WASM
#[wasm_bindgen]
pub fn roll(input: &str) -> Result<ExpressionResult, JsValue> {
    let rb: RollBuilder = RollBuilder::new().parse(input).map_err(|e| format!("{}", e))?;
    let rb: RollBuilder = rb.with_rng(Box::new(StdRng::seed_from_u64(random().to_bits())));
    let mut rb: Roll = rb.into_roll().map_err(|e| format!("{}", e))?;
    rb.roll().map_err(|e| format!("{}", e).into())
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
