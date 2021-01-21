# linq 核心 iterator

## 什么是 iterator ？

https://zh.wikipedia.org/wiki/%E8%BF%AD%E4%BB%A3%E5%99%A8

## rust 里面的迭代器

``` Rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```