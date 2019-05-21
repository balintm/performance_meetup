use std::mem;

fn main() {
    // Out of bounds, comment out
    let xs = [0, 1, 2, 3];
    let y = unsafe { *xs.as_ptr().offset(4) };

    // Use after free
    let a = vec![0, 1, 2, 3];
    let w = a.as_ptr();
    drop(a);
    let z = unsafe { *w };

    // Memory leak
    let b = vec![0, 1, 2, 3];
    mem::forget(b);
}
