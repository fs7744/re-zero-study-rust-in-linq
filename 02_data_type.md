## data type

### 标量类型（scalar）

标量类型是单个值类型的统称。

Rust中内建了4种基础的标量类型：整数、浮点数、布尔值及字符。

#### 整型

整数型简称整型，按照比特位长度和有无符号分为一下种类：

| 位长度  | 有符号  | 无符号 |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台，如果是 32 位架构的处理器将使用 32 位位长度整型。

整数的表述方法有以下几种：

| Number literals  || Example       |
|------------------|-|---------------|
| Decimal          |十进制| `98_222`      |
| Hex              |十六进制| `0xff`        |
| Octal            |八进制| `0o77`        |
| Binary           |二进制| `0b1111_0000` |
| Byte (`u8` only) |字节(只能表示 u8 型)| `b'A'`        |

很显然，有的整数中间存在一个下划线，这种设计可以让人们在输入一个很大的数字时更容易判断数字的值大概是多少。

##### 整数溢出

* debug 模式会触发 panic
* release 模式会在溢出发生时执行二进制补码环绕。

#### 浮点数类型

* 单精度浮点数 `f32`
* 双精度浮点数 `f64`

#### 布尔类型

bool：true和false，它会占据单个字节的空间大小

#### 字符类型

`'c'`

Rust中的char类型占4字节，是一个Unicode标量值 非 ASCII

### 复合类型（compound)

#### 元组（tuple）

``` Rust
let tup = (3,3);
```

#### 数组（array）

不可变

``` Rust
let array = [1,3,4];
```

## 类型别名

我们可以用type关键字给同一个类型起个别名（type alias）

``` Rust
type Age = u32;
fn grow(age: Age) -> Age {
    age + 1
}
```

## 类型推导

``` Rust
let mut x = Vec::new(); 
x.push(1);
// x 被推导为 std::vec::Vec<i32>
```

Rust只允许“局部变量/全局变量”实现类型推导

## 类型转换

Rust提供了一个关键字`as`，专门用于类型转换, 但必须显式地标记出来，防止隐藏的bug。

## struct

``` Rust
struct Point {
    x: i32,
    y: i64
}
```

## tuple struct

``` Rust
struct Point(i32, i64);
```

tuple struct有一个特别有用的场景，那就是当它只包含一个元素的时候，就是所谓的newtype idiom。因为它实际上让我们非常方便地在一个类型的基础上创建了一个新的类型

## enum

``` Rust
enum Number{
    Int(i32),
    Float(f32)
}
```

## 类型递归定义

Rust 不能直接直接嵌套类型递归定义，如：

``` Rust
struct Number {
    i :i32,
    y : Number
}
```

这是因为Rust是允许用户手工控制内存布局的语言。直接使用类型递归定义的问题在于，当编译器计算Number这个类型大小的时候无法得出结果。

这种场景可以用指针间接引用，因为指针都是固定大小的

``` Rust
struct Number {
    i :i32,
    y : Box<Number>
}
```

## Rust 不支持

* Rust 函数没有尾递归调用优化 https://zhuanlan.zhihu.com/p/148028147
* 不支持 yield （可手动实现 iterator）
* 不支持类型继承（subtyping）