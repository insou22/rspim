mod context;
mod data_label_gen;
mod gen;
mod lexer;
mod vm;

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate enum_display_derive;

use lexer::Token;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", &args[0]);
        return Ok(());
    }

    let file_name = &args[1];
    println!("Tokenising file: {}", file_name);

    let file_contents = std::fs::read_to_string(file_name).expect("Could not read file {}");
    let tokens = lexer::tokenise(&file_contents)?;

    println!("Tokens: {:?}", tokens);
    println!("\n\n");

    let program = gen::generate(tokens);

    println!("Program: {:#x?}", program);

    Ok(())
}
