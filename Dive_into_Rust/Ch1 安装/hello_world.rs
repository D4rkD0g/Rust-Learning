//fn FOO(arg1: i32, arg2: u32) -> i32
//源码一定要使用utf-8编码

fn main() {
    let s = "hello lambdax";//静态强类型
    println!("{}", s);//宏，编译期语法扩展
    println!("{:?}", s);
    println!("{:#?}", s);
    println!("{a} {b} {b}", a = "sirius", b = "lambda");
}