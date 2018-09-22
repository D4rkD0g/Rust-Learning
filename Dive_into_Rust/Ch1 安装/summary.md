
## 0x00 原则

    1.安全
        接近硬件的系统编程语言

    2.并发
        编译阶段消灭数据竞争

    3.实用
        后端LLVM

## 0x01 国内包源

>➜  ~ cat .cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

## 0x02 版本发布

> RFC -> nightly -> bete -> stable

在nightly中使用特性时要加`#![feature(...name...)]`

## 0x03 Prelude

crate: 完整的编译单元，=> lib、exe
mod: 可以理解为namespace

编译器会为用户写的每个crate自动插入一句话：`use std::prelude::*;`