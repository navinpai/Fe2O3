
fn main(){
    // println is a macro which is why we need ! after methodname

    println!("hello");

    // This fails at compilation if you reference {2} which is not provided
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    let x = 5;
    println!("value is {}", x);

    // :b converts to binary 
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // println == print but with newline
    // eprintln == eprint but with newline
    // eprint == print but writes out to stderr instead of stdout
    
    let pi = 3.141592;
    println!("Value of pi {:0.3}", pi);

    // This generates a warning saying Deadcode unused. To use uncomment line below
    // #[allow(dead_code)]
    // struct Structure(i32);
    
    // #[allow(unused_variables)] or convert var to _y
    // let y = 7;
}