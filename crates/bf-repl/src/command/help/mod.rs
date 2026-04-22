pub fn execute() {
    println!("help:");
    println!("  run <code>            Run brainfuck code");
    println!("  get <addr>            Get the value at the given address");
    println!("  set <addr> <val>      Set the value at the given address");
    println!("  add <addr> <val>      Add the value to the cell at the given address");
    println!("  position              Show the current data pointer position");
    println!("  view <left> <right>   View memory in the given range");
    println!("  help                  Show this help message");
    println!("  exit                  Exit the REPL");
}
