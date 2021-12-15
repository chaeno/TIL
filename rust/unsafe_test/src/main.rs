static HELLO_WORLD: &str = "안녕하세요!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc
    }
}

fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    println!("인삿말: {}", HELLO_WORLD);

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {:?}", *r1);
        println!("r2 = {:?}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
    // segmentation fault 
    // println!("{:?}", slice);

    unsafe {
        println!("C 언어에 따르면 -3의 절대값은 {}입니다.", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), 
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
