mod registers;

use registers::Registers;
pub const MEMORY_LOCATION: i32 = 65536 ;

#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    //Memory
    let memory : [u16; MEMORY_LOCATION as usize] = [0; MEMORY_LOCATION];

    println!("The first registers is {:?}", Registers::Rr0);

}

#[cfg(test)]
mod tests{
    use crate::Registers;
    use super::*;

    #[test]
    fn anything(){

    }



}

