pub fn allocate() {
  use std::mem;
  let mut bad_vec: Vec<i16> = Vec::with_capacity(1024);
  for i in 0..8*1024 {
    bad_vec.push(i);
  }
  mem::forget(bad_vec);
}


// Double free
// Example from https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
pub fn double_free() {
    let x = Box::new(String::from("hello"));
    let ptr = Box::into_raw(x);
    // Construct a box from raw pointer
    // After calling this function, the raw pointer is owned by the resulting Box. 
    let _x = unsafe { Box::from_raw(ptr) };
    let _y = unsafe { Box::from_raw(ptr) };
    let _xp = Box::into_raw(_x);
    let _yp = Box::into_raw(_y);
    unsafe{
      std::ptr::write(_yp, "hi".to_string());
    }
    unsafe {
      println!("Before boxing {}. After boxing, {}! {}", Box::from_raw(ptr), Box::from_raw( _xp), Box::from_raw(_yp));
    }
}

// https://doc.rust-lang.org/nomicon/unchecked-uninit.html
pub fn uninitialized_var() {
  use std::mem;
  use std::ptr;
  
  // size of the array is hard-coded but easy to change. This means we can't
  // use [a, b, c] syntax to initialize the array, though!
  const SIZE: usize = 10;
  
  let mut x: [Box<u32>; SIZE];
  unsafe {
      // convince Rust that x is Totally Initialized
      x = mem::uninitialized();
      for i in 0..SIZE {
          println!("{}", i);
          if i == 9 {
              println!("Going for segfault");
              println!("{:?}", &mut x[i]);
          }
          // very carefully overwrite each index without reading it
          // NOTE: exception safety is not a concern; Box can't panic
          ptr::write(&mut x[i], Box::new(i as u32));
      }
  }
}
 
// Uninitialized memory 
// Example from https://doc.rust-lang.org/std/mem/fn.uninitialized.html
pub fn uninitialized() {
  use std::mem;
  use std::ptr;

  // Only declare the array. This safely leaves it
  // uninitialized in a way that Rust will track for us.
  // However we can't initialize it element-by-element
  // safely, and we can't use the `[value; 1000]`
  // constructor because it only works with `Copy` data.
  let mut data: [Vec<u32>; 1000];
  
  unsafe {
      // So we need to do this to initialize it.
      data = mem::uninitialized();
  
      // DANGER ZONE: if anything panics or otherwise
      // incorrectly reads the array here, we will have
      // Undefined Behavior.
      ptr::write(&mut data[0], Vec::new());
      println!("Data 0: {:?}", &data[0]);
      let _x = &mut data[1];
      _x.push(1);
      _x.push(2);
      _x.push(3);
       println!("Data1: {:?}", &data[1]);
 
      // It's ok to mutably iterate the data, since this
      // doesn't involve reading it at all.
      // (ptr and len are statically known for arrays)
//      for elem in &mut data[..] {
          // *elem = Vec::new() would try to drop the
          // uninitialized memory at `elem` -- bad!
          //
          // Vec::new doesn't allocate or do really
          // anything. It's only safe to call here
          // because we know it won't panic.
//          ptr::write(elem, Vec::new());
//      }
  
      // SAFE ZONE: everything is initialized.
  }

  println!("Data0: {:?}", &data[0]);
}
fn main() {
 for _i in 0..15 {
      allocate();
  }
  double_free();
  uninitialized_var();
  uninitialized();
}

