fn main() {
    // code will not compile as foo 
    // has not been declared to be 
    // mutable and as such only 
    // one assigment to foo can exist
    let foo = 5;
    foo = 6; // compile error
    println!("{}",foo);
    // to get avoid declaring a 
    // mutable variable we could also
    // just get a new pointer of the 
    // value by using the following piece 
    // of code
    let foo = 5;
    let foo = 7;
    println!("{}",foo);
    // to declear a mutable reference
    // it would be as follows
    let mut foo = 5;
    foo = 8;
    println!("{}",foo);
}


