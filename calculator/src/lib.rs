#[allow(warnings)]
mod bindings;

use bindings::exports::component::calculator::Calculator;

struct Component;

impl Calculator for Component {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(a: f64, b:f64) -> f64 {
        a - b
    }

    fn multiply(a: f64, b: f64) -> {
        a * b
    }

    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }
}

bindings::export!(Component with_types_in bindings);
