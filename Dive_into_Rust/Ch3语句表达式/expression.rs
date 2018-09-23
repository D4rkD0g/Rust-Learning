fn main() {
    let x: () = {println!("Hello.");};
    let y: i32 = {println!("Hello."); 5};
    let z: i32 = if y == 5 {1} else {0};

    //ifelse(z);
    //loops();
    //tablelife();

    let v = loop{
        break 10; //break后可以加表达式，表示loop的返回
    };
    println!("{}", v);

    //let vv = loop{}; //发散类型，造成后边代码不执行
    //println!("{:?}", vv);

    //r#while(); //使用关键字作为标识符

    //loopwhile();

    let array = &[1, 2, 3, 4];

    for i in array {
        println!("the num is {}", i);
    }
}

fn ifelse(n: i32) -> bool {
    println!("{:?}", n);
    if n < 0 {
        println!("{} is negative", n);
        true
    } else if n > 0 {
        println!("{} is positive", n);
        true
    } else {
        println!("{} is zero.", n);
        false
    }
}

fn loops() {
    let mut count = 0u32;

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        if count == 5 {
            println!("It's enough");
            break;
        }
    }
}

fn tablelife() {
    let mut m = 1;
    let n = 1;

    'a: loop {
        if m <= 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("Break");
                    break 'a;
                } else {
                    println!("Now m = {}, n = {}", m, n);
                    continue 'a;
                }
            }
        }
    }
}

fn r#while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn loopwhile() {
    let x;
    loop {
        x = 1;
        break;
    }
    println!("{}", x);

    let x;
    while true {
        x = 2;
        break;
    }
    //println!("{}", x);//use of possibly uninitialized `x`
}