use std::env;

fn foo(i: i32) {
    // println!("foo({}): begin", i);
    if i < 16 {
        bar(i);
    }
    else {
        baz(i);
    }
    println!("foo({}): end", i);
}

fn bar(i: i32) {
    // println!("bar({}): begin", i);
    for n in 0..i {
        qux(n);
    }
    println!("bar({}): end", i);
}

fn baz(i: i32) {
    // println!("baz({}): begin", i);
    for n in 0..i {
        corge(n);
    }
    println!("baz({}): end", i);
}

fn qux(i: i32) {
    // println!("qux({}): begin", i);
    grault(i);
    println!("qux({}): end", i);
}

fn corge(i: i32) {
    // println!("corge({}): begin", i);
    grault(i);
    println!("corge({}): end", i);
}

fn grault(i: i32) {
    // println!("grault({}): begin", i);
    if i > 0 {
        foo(i-1);
    }
    println!("grault({}): end", i);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() !=  2 {
        println!("ERROR: Missing numeric argument for test");
        std::process::exit(1);
    }
    let num: i32 = args[1].trim().parse().unwrap();
    foo(num);
}
