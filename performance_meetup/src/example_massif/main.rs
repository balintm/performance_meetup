use std::alloc::System;

#[global_allocator]
static A: System = System;

use std::mem;

pub fn vect_alloc() -> i32 {
     let mut nums: Vec<i32> = (1..100).collect();
//    let mut nums: Vec<i32> = Vec::new();
    for i in 1..31000 {
        nums.push(0xFFF + i);
    }

    let mut ret = 0;
    for i in &nums {
        ret += i;
    }
    let x = nums[10];
    std::mem::forget(nums);
    return ret + x;
}

pub fn looping() -> i32 {
//     let mut nums:: Vec<i32> = (1..100).collect();
    let mut nums: Vec<i32> = Vec::new();
    for i in 1..31000 {
        nums.push(0xFFF + i);
    }

    let mut ret = 0;
    for i in &nums {
        ret += i;
    }
    let x = nums[10];
    std::mem::forget(nums);
    return ret + x;
}

fn main() ->Result<(), ()> {
    let mut ret:i64 = 0;
    for i in 1..10 {
        ret += vect_alloc() as i64;
    }
    if ret > 100 {
        Ok(())
    } else {
        Err(())
    }
}
