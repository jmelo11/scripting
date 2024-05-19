pub mod nodes;
pub mod parsers;
pub mod prelude;
pub mod utils;

use clap::{Arg, Command};
use prelude::*;
use std::fs::File;
use std::io::{self, Read};
// This is a placeholder function for your lexer, parser, and evaluator.
// Replace it with your actual implementation.
fn run_lefi_script(script: &str) -> Result<Vec<Value>> {
    // Tokenize the script (implement this with your actual lexer)
    let tokens = Lexer::new(script.to_string())
        .tokenize()
        .map_err(|e| ScriptingError::from(e))?;

    // Parse the tokens into an AST (implement with your parser)
    let nodes = Parser::new(tokens)
        .parse()
        .map_err(|e| ScriptingError::from(e))?;

    // Index expressions and initialize evaluator (adjust according to your actual logic)
    let indexer = ExpressionIndexer::new();
    indexer.visit(&nodes).unwrap();

    let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_variables_size());
    evaluator
        .const_visit(nodes)
        .map_err(|e| ScriptingError::from(e))?;

    // Return the evaluated variable values
    Ok(evaluator.variables().clone())
}

fn main() -> io::Result<()> {
    // Initialize CLI command argument parser
    let matches = Command::new("lefi-cli")
        .version("1.0")
        .author("Jose Melo")
        .about("Runs a .lefi file using the custom LEFI language interpreter")
        .arg(
            Arg::new("input")
                .help("Input .lefi file")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Retrieve the input file path using `get_one`
    let input_path: &String = matches.get_one("input").expect("Input file is required");

    // Check file extension
    if !input_path.ends_with(".lefi") {
        eprintln!("Error: The input file must have a .lefi extension.");
        std::process::exit(1);
    }

    // Read the contents of the .lefi file
    let mut file = File::open(input_path)?;
    let mut script = String::new();
    file.read_to_string(&mut script)?;

    // Tokenize, parse, and evaluate the script (Replace this section with your actual lexer, parser, and evaluator)
    match run_lefi_script(&script) {
        Ok(variables) => {
            for (index, value) in variables.iter().enumerate() {
                println!("Variable {}: {:?}", index, value);
            }
        }
        Err(e) => {
            eprintln!("Execution Error: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
