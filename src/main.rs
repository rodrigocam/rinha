mod ast;
mod eval;

use std::{env, error, fs};

use eval::Interpreter;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let ast = fs::read_to_string(&args[1])?;

    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast);

    Ok(())
}
