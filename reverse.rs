
// imports
use std::io::{self, Read};


// program entry for reverse(1)
fn main() {
    // top-level error handler
    match reverse() {
        Err(value) => panic!(value),
        _ => {}
    }
}


fn reverse() -> io::Result<()> {
    // create a String buffer
    let mut input = String::new();

    // get stdin and make a synchronous handle to it
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // read all of stdin into a String buffer
    try!(handle.read_to_string(&mut input));

    // transform the buffer into lines and then reverse those
    for line in input.lines().rev() {
        println!("{}", line);
    }

    Ok(()) // exit the process
}
