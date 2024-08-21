use std::{io::{stdin, stdout}, path::PathBuf};
use brainfuck_interpreter::{Interpreter, Transpiler};

fn interpret(path: &PathBuf) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|e| format!("Error opening the file in path {} - {e}",path.to_str().unwrap()))?;
    let mut i = Interpreter::new(&source);
    let mut stdin = stdin();
    let mut stdout = stdout();
    i.run(&mut stdin, &mut stdout)?;
    Ok(())
}

fn transpile(path: &PathBuf) -> Result<(), String> {
    let source = std::fs::read_to_string(path).map_err(|e| format!("Error opening the file in path {} - {e}",path.to_str().unwrap()))?;
    let mut t = Transpiler::new(&source);
    let mut stdout = stdout();
    t.transpile(&mut stdout)?;
    Ok(())
}


fn main() -> Result<(), String>{
    let args: Vec<_> = std::env::args().collect();

    match args.len() {
        2 => interpret(&PathBuf::from(&args[1])),
        3 => {
            
            for i in 1..=2 {
                if args[i] == "--transpile" {
                    return transpile(&PathBuf::from(&args[if i == 1 {2} else {1}]));
                }
            }
            println!("Usage: {} <SOURCE_FILE> [OPTIONS]\n\nOptions:\n\t--transpile: Transpiles to C",args[0]);
            return Ok(())
        },
        _ => {
            println!("Usage: {} <SOURCE_FILE> [OPTIONS]\n\nOptions:\n\t--transpile: Transpiles to C",args[0]);
            return Ok(())
        }
    }.map_err(|e| format!("[!] Error: {e}"))
}
