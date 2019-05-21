use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::fs::File;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;


pub fn syn_files() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    for i in 1..1002 {
      f.write_all(b"Hello, world!")?; 
     //  f.sync_all()?;
    }
    Ok(())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
#[warn(unused_variables)]
pub fn write_none (l: &String, writer: &mut BufWriter<&File>) {
}

pub fn write_rev(l: &String, writer: &mut BufWriter<&File>) {
    let rev = l.chars().rev().collect::<String>();
    writer.write(rev.as_bytes());
    writer.write(b"\n").unwrap();
}

pub fn write_hash(l: &String, writer: &mut BufWriter<&File>) {
    let hashnum = calculate_hash(&l);
    writer.write(hashnum.to_string().as_bytes()).unwrap();
    writer.write(b"\n").unwrap();
    return;
}

pub fn write_copy(l: &String, writer: &mut BufWriter<&File>) {
    writer.write((l).as_bytes()).unwrap();
    writer.write(b"\n").unwrap();
}

pub fn write_all(l: &String, writer: &mut BufWriter<&File>) {
    write_copy(l, writer);
    write_hash(l, writer);
    write_rev(l, writer);
}

pub fn read_write_files() ->  std::io::Result<()> {
    let fr = File::open("foo_read.txt").unwrap();
    let fw = File::create("foo_write.txt").unwrap();

    let rfile = BufReader::new(&fr);
    let mut writer = BufWriter::new(&fw);

    let mut oldl = 5;
    for (num, line) in rfile.lines().enumerate() {
        let l = line.unwrap();
        match (l.len() * num) % (oldl + 1) {
            0 =>  write_none(&l, &mut writer),
            1 =>  write_hash(&l, &mut writer),
            2 =>  write_rev(&l, &mut writer),
            3 =>  write_copy(&l, &mut writer),
            4 =>  write_all(&l, &mut writer),
            _ => write_all(&l, &mut writer)
        };
        oldl = l.len();
    }           
    Ok(())
}

pub fn allocate() {
  use std::mem;
  let mut bad_vec: Vec<i16> = Vec::with_capacity(1024);
  for i in 0..8*1024 {
    bad_vec.push(i);
  }
  mem::forget(bad_vec);
}

const MX_SIZE: usize = 1000;

pub fn mx_sum(mx: &[[f32; MX_SIZE as usize]],  ) -> f32 {
    let mut ret = 0.;
    for i in 0.. MX_SIZE {
        for j in 0..MX_SIZE {
            ret += mx[i as usize][j as usize];
        }
    }
    return ret;
}

pub fn alloc_matrix() {
    let mut mx: [[f32; MX_SIZE]; MX_SIZE];
    unsafe {
      mx = mem::uninitialized();
    }
    for i in 0.. MX_SIZE {
        for j in 0..MX_SIZE {
            mx[i][j] = 0.1;
        }
    }
    let mut ret = 0.;
    for i in 0..MX_SIZE {
        for j in 0..MX_SIZE {
            ret += mx[i as usize][j as usize]*mx[1][1];
        }
    }
    println!("matrix sum: {}", mx_sum(&mx));
    println!("matrix {}", ret);
}

fn main() {
    syn_files();
    read_write_files();
    #[warn(unused_variables)]
    for i in 1..100 {
        allocate();
    }
    alloc_matrix();
}

