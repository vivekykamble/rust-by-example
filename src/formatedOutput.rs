// formated output
//
// format! : write formatted text to string
// print!  : same as format, print the text to consol
// println! : same as print but new line is appended
// eprint! : same as format but write to errorstream
// eprintpl! : same as eprint append new line.
//
//
fn main(){
    //{} is replaced with the argument is sent. Stringified
    println!("{} days ", 31);
   
    //positional arguments
    println!("Hi, {0} this {1}, {1} this {0}", "Vivek", "Subham");

    //named arguments
    println!("{subject} is very {verb}, he bark {object}", object="dog", subject="Max", verb="very well");

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can lef-align text with a specified width. This will output
    // "1     ". 5 white spaces.
    println!("{number:<width$}", number=1, width=6);

    let pi =3.141592;

    println!("pi is {:.2}",pi);
}
