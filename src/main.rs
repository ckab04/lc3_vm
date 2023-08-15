use crate::components::registers::Registers;
mod components;

fn main() {
    println!("Hello there !!!");

    let val = Registers::R_R7;
    //let i = Registers::from(val);
    let s: i32 = val.into();
    //println!("Value obtained {:?}", i);
    println!("Value of s = {}", s);

}



