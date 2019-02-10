#### 0x01 代码解析

教程里的只能猜一次，加了个循环判断是否退出

用到的库

```rust
use std::io::Read;
use rand::Rng;
use std::io;
use std::cmp::Ordering;
```

rand用来生成随机数    
`let secret = rand::thread_rng().gen_range(1, 101);`

readline读数据到String   
`io::stdin().read_line(&mut guess).expect("failed to read");`
或者用read_exact读到[u8;NUM]   
`io::stdin().read_exact(&mut choice).expect("failed to read");`

parse解析数据  
`let guess: u32 = match guess.trim().parse()`  
`let guess: u8 = match guess.trim().parse<u8>()`

cmp判断大小  
```rust
match guess.cmp(&secret) {
    Ordering::Equal => {}
    Ordering::Greater => {}
    Ordering::Less => {}
}
```

#### 0x02 教程内容

> The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.  
> The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.   
> Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. The number 0.3.14 is actually shorthand for ^0.3.14, which means “any version that has a public API compatible with version 0.3.14.”
> When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and use the versions specified there rather than doing all the work of figuring out versions again. 
> 