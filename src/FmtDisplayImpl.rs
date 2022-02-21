

use std::fmt;


struct Structure(i32);

impl fmt::Display for Structure {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}


struct MinMax(i32, i32);

impl fmt::Display for MinMax{

    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"Value min = {}, max ={}", self.1, self.0)
    }
}


struct Complex{

    real: f64,
    img: f64
}



impl fmt::Display for Complex{

    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{

    write!(f, "{} + {}i", self.real, self.img)
    }
}
fn main(){
    
    println!("Structure Printed with fmt::Display implemented,{ }", Structure(34));

        println!("Structure Printed with fmt::Display implemented,{ }", MinMax(12,23));

    println!("{}",Complex{real:3.3, img :7.1});

}
