fn main() {
    let s1 = String::from("len");

    // s1 not dropped after len is computed, passed reference to s1, reference does not own it
    // Value it points to will not be dropped after reference is no longer used
    let len = calculate_length(&s1);

    println!("{len}");

    let mut s2 = String::from("hi");
    let l = mod_and_calculate_length(&mut s2);

    let mut s3 = String::new();

    {
        // Goes out of scope so a new reference can be made without any issue
        let r2 = &mut s3;
    }

    let r1 = &mut s3;
    // This code is invalid, cannot borrow a value as a mutable more than once at a time
    //let r2 = &mut s3;
    //
    //println!("{}, {}", r1, r2);

    let mut s5 = String::new();
    let r4 = &s5;
    // Causes compiler error, cannot borrow something as a mutable that is also borrowed as immutable
    //let r5 = &mut s5;
    //println!("{}, {}", r4, r5);

    println!("{r1}");
    println!("{l}");

    let s6 = String::from("hi");
    let r6 = &s6;
    let r7 = &s6;
    // Scopes don't overlap, code is valid

    println!("{r6}, {r7}");
    // r6 is not used after this point
    dangle();
}

// &String indicates parameter is a reference to a string
fn calculate_length(s: &String) -> usize {
    // Attempt to modify value being borrowed fails
    //s.push_str("o"):
    s.len()
}

fn mod_and_calculate_length(s: &mut String) -> usize {
    s.push_str("o");
    s.len()
}

//fn dangle() -> &String {
//    let s = String::from("hello");

    // Cannot return this as it references data owned by the current function
    // s would be deallocated when this function finishes
    // The reference would point to an invalid String
//    &s
//}
