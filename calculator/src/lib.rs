#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

use bindings::docs::adder::add::add;
use bindings::docs::subtractor::subtract::subtract;


struct Component;


impl Guest for Component
{
    fn eval_expression(op: Op, x: f32, y: f32) -> f32
    {
        match op
        {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}


bindings::export!(Component with_types_in bindings);
