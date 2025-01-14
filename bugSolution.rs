fn main() {
    let mut x = 5;
    { //This is one way to solve, using a block scope
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15;
}

//Alternative solution
fn main() {
    let mut x = 5;
    let y = x; //Clone the value
    let mut z = x; //Clone the value
    z = 15;
    x = y + z;
}