#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

use bindings::docs::adder::add::add;
use bindings::docs::subtractor::subtract::subtract;
use bindings::docs::multiplier::multiply::multiply;
use bindings::docs::divider::divide::divide;


struct Component;


impl Guest for Component
{
    fn eval_expression(op: Op, x: f32, y: f32) -> Result<f32, String>
    {
        match op
        {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
            Op::Multiply => multiply(x, y),
            Op::Divide => divide(x, y),
        }
    }
}


bindings::export!(Component with_types_in bindings);
