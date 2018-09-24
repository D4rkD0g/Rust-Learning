## 0x01 函数

函数的默认返回类型为`unit()`


    let mut func = add1 as fn((i32, i32)) -> i32; //as类型转换

    let mut func: fn((i32, i32)) -> i32 = add1; //显式类型标记

## 0x02 发散函数

返回类型为`!`，可以被转换为任一类型（if-else每条分之类型相同）

一下情况永远不会返回：
1. panic!以及基于它实现的各种函数/宏，`unimplemented!`、`unreacherable!`。
2. loop{}
3. std::process::exit以及类似libc中的exec函数

## 0x03 main函数

传递参数和返回状态由单独的API完成：`std::env::args`、`std::env::vars`

## 0x03 const fn

函数使用const修饰，在编译阶段被编译器执行，函数返回数字的话，可以当作常量数组的长度.想着反汇编看一看来着，发现“好复杂”。。。只是懒得看