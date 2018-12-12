
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
    println!("value is {}", x)

    // println == print but with newline
    // eprintln == eprint but with newline
    // eprint == print but writes out to stderr instead of stdout
}