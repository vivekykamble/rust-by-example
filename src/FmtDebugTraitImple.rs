
struct UnPrintable(i32);

#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct DeepPrint(Printable);

fn main(){
println!("{:?}", Printable(2));

println!("{:?}", DeepPrint(Printable(4)));

//Pretty Printing with #
//

println!("{:#?}", DeepPrint(Printable(10)));
}
