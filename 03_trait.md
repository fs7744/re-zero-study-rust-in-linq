# trait

在Rust中，trait这一个概念承担了多种职责。在中文里，trait可以翻译为“特征”“特点”“特性”等.

trait中可以包含：函数、常量、类型等。

trait本身既不是具体类型，也不是指针类型，它只是定义了针对类型的、抽象的“约束”。不同的类型可以实现同一个trait，满足同一个trait的类型可能具有不同的大小。因此，trait在编译阶段没有固定大小，目前我们不能直接使用trait作为实例变量、参数、返回值。

## 函数

``` Rust
trait Shape {
    fn area(&self) -> f64; //关联函数（associated function）
}
```

函数的第一个参数如果是Self相关的类型，且命名为self（小写s），这个参数可以被称为“receiver”（接收者）。具有receiver参数的函数，我们称为“方法”（method），可以通过变量实例使用小数点来调用。没有receiver参数的函数，我们称为“静态函数”（static function），可以通过类型加双冒号：:的方式来调用。在Rust中，函数和方法没有本质区别。

### slef 参数

``` Rust
trait T {
    fn method1(self: Self); 
    fn method2(self: &Self); 
    fn method3(self: &Mut Self); 
}
// 等同于
trait T {
    fn method1(self); 
    fn method2(&self); 
    fn method3(&mut self); 
}
```

### 为struct实现trait

``` Rust
trait T {
    fn method1(self: Self) -> f64; 
}

struct C {
    r: f64;
}

impl T for C {
    fn method1(self) -> f64 {
        self.r
    }
}
```

### 内在方法（inherent methods）

``` Rust
impl C {
    fn method2(&self) -> f64 {
        self.r * 64
    }
}
```

我们可以把这段代码看作是为Circle类型impl了一个匿名的trait

### trait中可以包含方法的默认实现

trait中可以包含方法的默认实现。如果这个方法在trait中已经有了方法体，那么在针对具体类型实现的时候，就可以选择不用重写。当然，如果需要针对特殊类型作特殊处理，也可以选择重新实现来“override”默认的实现方式。比如，在标准库中，迭代器Iterator这个trait中就包含了十多个方法，但是，其中只有fnnext(&mut self) ->Option<Self::Item>是没有默认实现的。其他的方法均有其默认实现，在实现迭代器的时候只需挑选需要重写的方法来实现即可。

## 静态方法

有receiver参数的方法（第一个参数不是self参数的方法）称作“静态方法”。静态方法可以通过Type::FunctionName()的方式调用。需要注意的是，即便我们的第一个参数是Self相关类型，只要变量名字不是self，就不能使用小数点的语法调用函数。

## Rust中没有“构造函数”的概念。

Defaulttrait实际上可以看作一个针对无参数构造函数的统一抽象。

``` Rust
pub trait Default {
    fn default() -> Self;
}
```

## 扩展方法

我们还可以利用trait给其他的类型添加成员方法，哪怕这个类型不是我们自己写的。

但我们也不是随随便便就可以这么做的，Rust对此有一个规定。在声明trait和impl trait的时候，Rust规定了一个Coherence Rule（一致性规则）或称为Orphan Rule（孤儿规则）:impl块要么与trait的声明在同一个的crate中，要么与类型的声明在同一个crate中。

## 完整函数调用语法

`<T as TraitName>::item`

因为不同trait 可以有相似的方法签名， 

当同一个struct 都有这样不同trait 实现时，

编译器无法识别，

所以需要明确

## 泛型约束

``` Rust
fn method1<T : Debug>(x：T);
//or
fn method1<T>(x：T) where T : Debug;
```

## trait继承

`trait Derived : Base {}`等同于`trait Derived where Self: Base {}`。这两种写法没有本质上的区别，都是给Derived这个trait加了一个约束条件，即实现Derived trait的具体类型，也必须满足Base trait的约束。

## Derive

Rust提供了一个特殊的attribute，它可以帮我们自动impl某些trait。

``` Rust
#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, RustEncodanle, FromPromotove, Send, Sync])]

```

## trait别名

``` Rust
trait HttpService = Service<Request = http:Request, Response = http:Response>;
```