## Install Rust

1. download rustup-init.exe from https://www.rust-lang.org/learn/get-started

2. 安装vs2019 中的 c++ 

（因为我在window 10 使用 msvc 版本的 Rust, 需要VisualC++的工具链）

3. 选择 ide

（我使用vscode，因为我本来就有，哈哈）

4. 安装 vscode 插件 Rust 和 codelldb

## Hello Rust 顺便测试debug 插件

1. cargo new hello --bin

2. 打开 vscode

3. 点击 vs code 的 run and debug 会自动生成debug的json配置

## Hello world 中的Rust 基础

``` Rust
fn main() {
    println!("Hello, world!");
}
```

* `fn` 函数定义

* `println！` 是宏。Rust中的宏与C/C++中的宏是完全不一样的东西。简单点说，可以把它理解为一种安全版的编译期语法扩展。这里之所以使用宏，而不是函数，是因为标准输出宏可以完成编译期格式检查，更加安全。