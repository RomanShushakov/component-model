#[allow(warnings)]
mod bindings;

use bindings::exports::docs::divider::divide::Guest;


struct Component;


impl Guest for Component 
{
    fn divide(a: f32, b: f32) -> Result<f32, String>
    {
        if b == 0.0 
        {
            return Err("Denominator is equal to zero!".to_string());
        }
       Ok(a - b)
    }
}

bindings::export!(Component with_types_in bindings);
