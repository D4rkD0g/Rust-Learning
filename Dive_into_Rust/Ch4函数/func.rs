#![feature(const_fn)]

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32{
    x + y
}

fn add3(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let p = (1, 2);
    let mut func = add1;
    println!("Add {}", func(p));

    //func = add2;// expected fn item, found a different fn item
    //使用以下两种方法转化后方可
    //let mut func = add1 as fn((i32, i32)) -> i32;
    let mut func: fn((i32, i32)) -> i32 = add1;
    func = add2;

    //func = add3; //incorrect number of function parameters

    //diverges();
    //test_inner();

    for arg in std::env::args() {
        println!("Arg: {}", arg);
    }

    //println!("fib(8): {}", fib(8));

    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("ARR: {:?}", ARR);

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value); //不是unicode就会panic
    }

    getenv();
}

fn diverges() -> ! {
    panic!("never return");
}

fn test_inner() {
    static INNER_STATIC: i64 = 42;

    fn internal_incr(x: i64) -> i64 {
        x + 1
    }

    struct InnerTemp(i64);

    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = internal_incr(self.0);
        }
    }

    let mut t = InnerTemp(INNER_STATIC);
    t.incr();
    println!("{}", t.0);
}

fn fib(index: u32) -> u64 {
    if index == 1 || index == 2 {
        1
    } else {
        fib(index - 1) + fib(index - 2) //暂不支持尾递归
    }
}

const fn cube(num: usize) -> usize {
    num * num * num
}

fn getenv() {
    for arg in std::env::args() {
        match std::env::var(&arg) {
            Ok(val) => println!("{}: {:?}", &arg, val),
            Err(e) => println!("can't find {}: {:?}", &arg, e),
        }
    }
    println!("All {}", std::env::vars().count());
}
