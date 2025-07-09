use std::slice;

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("calling a rust function from C");
}

static HELLO_WORLD: &str = "hello world";

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined behavior, must
/// guarantee that it is only called from a single thread at a time
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 1;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    println!("{r1:?}, {r2:?}");

    unsafe  {
        println!("{}, {}", *r1, *r2);
    }

    let address = 0x12345usize;
    let r = address as *const i32;

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);

    println!("abs val of -5 according to C: {}", abs(-5));

    /// SAFETY: This is only called from a single thread in `main`
    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe trait foo {
    // methods
}

unsafe impl Foo for i32 {
    // impl goes here
}
