#[allow(warnings)]
mod bindings;

use bindings::exports::docs::adder::add::Guest;


struct Component;


impl Guest for Component 
{
    fn add(a: f32, b: f32) -> f32
    {
       a + b
    }
}

bindings::export!(Component with_types_in bindings);
