mod bindings;


use clap::Parser;
use std::fmt;


use bindings::docs::calculator::{calculate, calculate::Op};


fn parse_operator(op: &str) -> anyhow::Result<Op> 
{
    match op 
    {
        "add" => Ok(Op::Add),
        "subtract" => Ok(Op::Subtract),
        "multiply" => Ok(Op::Multiply),
        "divide" => Ok(Op::Divide),
        "divide-jco" => Ok(Op::DivideJco),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}


impl fmt::Display for Op
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Op::Add => write!(f, "+"),
            Op::Subtract => write!(f, "-"),
            Op::Multiply => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::DivideJco => write!(f, "/"),
        }
    }
}


/// A CLI for executing mathematical expressions
/// using WebAssembly
#[derive(Parser)]
#[clap(name = "calculator", version = env!("CARGO_PKG_VERSION"))]
struct Command
{
    /// The first operand
    x: f32,
    /// The second operand
    y: f32,
    /// Expression operator
    #[clap(value_parser = parse_operator)]
    op: Op,
}


impl Command
{
    fn run(self)
    {
        match calculate::eval_expression(self.op, self.x, self.y)
        {
            Ok(res) => println!("{} {} {} = {res}", self.x, self.op, self.y),
            Err(e) => println!("{}", e),
        }
    }
}

fn main()
{
    Command::parse().run()
}
