# Rust 学习笔记 - 进阶特 性

> 记录日期：2026-03-10

---

## 📚 目录
1. [泛型 (Generics)](#泛型-generics)
2. [Trait (特质/特征)](#trait-特质特征)
3. [Option 与 unwrap](#option-与-unwrap)
4. [unsafe 编程](#unsafe-编程)

---

## 🔷 泛型 (Generics)

### 1. 泛型结构体
```rust
// ❌ 只能存储一种类型
struct Wrapper {
    value: u32,  // 固定类型
}

// ✅ 可以存储任意类型
struct Wrapper<T> {
    value: T,  // 泛型类型参数
}

// 使用
let int_wrapper = Wrapper { value: 42 };        // Wrapper<i32>
let str_wrapper = Wrapper { value: "hello" };   // Wrapper<&str>
```

### 2. 泛型实现 - 关键理解 ⭐
```rust
impl<T> Wrapper<T> {
//   ↑         ↑
//  声明      使用
}
```

**为什么要写两次 `<T>`？**

| 位置 | 作用 | 说明 |
|------|------|------|
| `impl<T>` | **声明**泛型参数 | 告诉编译器："T 是一个类型参数" |
| `Wrapper<T>` | **使用**泛型参数 | 指定为哪个类型实现方法 |

**设计哲学**：声明和使用分离，提供灵活性

```rust
// 场景 1：为所有泛型类型实现
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// 场景 2：只为特定类型实现
impl Wrapper<String> {  // 不需要 <T>，因为没有泛型
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

// 场景 3：引入额外的泛型参数
impl<T> Wrapper<T> {
    pub fn convert<U>(self) -> Wrapper<U>  // 新的泛型 U
    where T: Into<U> {
        Wrapper { value: self.value.into() }
    }
}
```

### 3. self 的可变性
```rust
fn method(self) -> Self           // 获取所有权（不可变）
fn method(mut self) -> Self       // 获取所有权（可变）⭐
fn method(&self)                  // 借用（不可变）
fn method(&mut self)              // 借用（可变）
```

---

## 🔷 Trait (特质/特征)

### 1. 基本概念
**Trait** = 接口，定义一组方法签名

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}
```

### 2. Trait 的默认实现 ⭐
```rust
trait Licensed {
    fn licensing_info(&self) -> String {
        // 提供默认实现
        String::from("Some information")
    }
}

// 空的 impl 自动获得默认实现
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// 也可以选择覆盖
impl Licensed for CustomSoftware {
    fn licensing_info(&self) -> String {
        String::from("Custom license")
    }
}
```

### 3. Trait 作为参数 - 重要 ⭐⭐⭐

#### 方式 1：`impl Trait` 语法（静态分发）
```rust
fn compare(a: impl Licensed, b: impl Licensed) -> bool {
    a.licensing_info() == b.licensing_info()
}

// 调用
compare(software1, software2);  // 直接传值
```

#### 方式 2：泛型 + Trait Bound
```rust
fn compare<T: Licensed>(a: T, b: T) -> bool {
    a.licensing_info() == b.licensing_info()
}

// 等价写法（where 子句）
fn compare<T>(a: T, b: T) -> bool 
where T: Licensed {
    a.licensing_info() == b.licensing_info()
}
```

#### 方式 3：Trait Object（动态分发）
```rust
fn compare(a: &dyn Licensed, b: &dyn Licensed) -> bool {
    a.licensing_info() == b.licensing_info()
}

// 调用
compare(&software1, &software2);  // 必须传引用
```

### 4. 静态分发 vs 动态分发

| 特性 | `impl Trait` (静态) | `&dyn Trait` (动态) |
|------|---------------------|---------------------|
| **性能** | ✅ 更快（编译时确定） | ⚠️ 稍慢（运行时查表） |
| **代码大小** | ⚠️ 可能更大（为每种类型生成代码） | ✅ 更小（只有一份代码） |
| **传参方式** | 值/引用都可 | 必须用引用 |
| **异构集合** | ❌ 不支持 | ✅ 支持 |
| **使用场景** | 大多数情况 | 需要存储不同类型到同一集合 |

**示例：异构集合（只能用动态分发）**
```rust
let items: Vec<&dyn Licensed> = vec![
    &some_software,
    &other_software,  // 不同类型可以在同一个 Vec
];
```

---

## 🔷 Option 与 unwrap

### 1. Option 类型
```rust
enum Option<T> {
    Some(T),    // 有值
    None,       // 没有值
}
```

### 2. unwrap - 危险操作 ⚠️
```rust
let some_value = Some(42);
let x = some_value.unwrap();  // ✅ 得到 42

let no_value: Option<i32> = None;
let y = no_value.unwrap();     // ❌ panic! 程序崩溃
```

### 3. 更安全的替代方法

| 方法 | 行为 | 示例 |
|------|------|------|
| `unwrap_or(default)` | None 时返回默认值 | `opt.unwrap_or(0)` |
| `unwrap_or_else(f)` | None 时调用函数 | `opt.unwrap_or_else(\|\| calc())` |
| `expect("msg")` | 带自定义错误信息的 unwrap | `opt.expect("不应为空")` |
| `match` | 完全控制两种情况 | 见下方 |
| `if let` | 简化的 match | `if let Some(x) = opt { }` |

```rust
// ✅ 最安全：使用 match
match some_option {
    Some(value) => println!("Got: {}", value),
    None => println!("No value"),
}

// ✅ 提供默认值
let x = some_option.unwrap_or(0);

// ✅ 返回 Option 让调用者处理
fn safe_fn(s: &str) -> Option<char> {
    s.chars().next()  // 不 unwrap，返回 Option
}
```

### 4. 何时使用 unwrap？

| 场景 | 是否可用 |
|------|----------|
| 学习/原型开发 | ✅ |
| 你 100% 确定不会是 None | ✅ |
| 测试代码 | ✅ |
| 生产代码 | ❌ |
| 处理用户输入 | ❌ |

---

## 🔷 unsafe 编程

### 1. unsafe 的两种用途

#### 用途 1：声明 unsafe 函数
```rust
/// # Safety
/// 
/// 调用者必须保证 `address` 指向有效的 `u32` 值
unsafe fn modify_by_address(address: usize) {
    // ...
}
```

#### 用途 2：unsafe 代码块
```rust
unsafe {
    // 在这里可以进行 unsafe 操作
}
```

### 2. 裸指针操作
```rust
// 裸指针类型
*const T    // 不可变裸指针
*mut T      // 可变裸指针

// 操作流程
let mut t: u32 = 0x12345678;
let address = &mut t as *mut u32 as usize;  // 获取地址

unsafe {
    // SAFETY: address 来自有效的局部变量引用
    let ptr = address as *mut u32;  // usize -> 裸指针
    *ptr = 0xAABBCCDD;             // 解引用并修改
}
```

### 3. SAFETY 注释 - 必须添加 ⭐
```rust
unsafe {
    // SAFETY: The address is guaranteed to be valid and contains
    // a unique reference to a `u32` local variable.
    let ptr = address as *mut u32;
    *ptr = 0xAABBCCDD;
}
```

**注释应说明**：
- ✅ 为什么这个地址是有效的
- ✅ 为什么可以安全地解引用
- ✅ 满足了什么契约（contract）

### 4. 为什么需要 unsafe？

裸指针是 unsafe 的原因：
- ❌ 可能指向无效内存
- ❌ 可能违反别名规则
- ❌ 可能造成数据竞争
- ❌ 编译器无法验证安全性

**你的责任**：通过注释和代码证明操作是安全的

---

## 💡 学习心得

1. **泛型的威力**：让代码可以处理多种类型，避免重复
2. **Trait 的灵活性**：
   - 默认实现减少重复代码
   - 作为参数让函数更通用
   - 静态分发性能好，动态分发更灵活
3. **unwrap 是双刃剑**：学习阶段方便，生产环境危险
4. **unsafe 需谨慎**：必须添加 SAFETY 注释证明安全性

---

## 📖 常用模式速查

### String 拼接
```rust
// 方式 1：+ 运算符（消耗左操作数）
let s = s + "Bar";

// 方式 2：format! 宏
let s = format!("{}Bar", s);

// 方式 3：push_str（需要可变）
let mut s = s;
s.push_str("Bar");
s
```

### Vec 添加元素
```rust
let mut vec = vec;
vec.push(element);
vec
```

### 类型转换
```rust
let s = "hello";
let string = s.to_string();           // &str -> String
let string = String::from(s);         // &str -> String
let str_ref = &string;                // String -> &str
let str_ref = string.as_str();        // String -> &str
```
| 大小写 | `.to_uppercase()` | String |

### 模块系统

```rust
// 可见性（默认私有）
mod my_mod {
    fn private() {}       // 私有
    pub fn public() {}    // 公开
}

// use 导入和重命名
use std::collections::HashMap;
use super::Command;               // 从父模块
use self::inner::Item as NewName; // 重命名

// pub use 转发
pub use self::inner::ITEM as item;  // 导入并重新导出
```

### HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(key, value);
map.get(&key)              // Option<&V>
map.contains_key(&key)     // bool（需要引用！）
map.values().sum::<u32>()  // 求和

// 条件插入
map.entry(key).or_insert(0);
```

### Option 类型

```rust
enum Option<T> {
    Some(T),    // 有值
    None,       // 无值
}

// 提取值
x.unwrap()              // 如果是 None 会 panic
x.unwrap_or(0)          // 提供默认值
x.expect("错误信息")    // 带自定义信息

// if let - 简化的 match
if let Some(value) = option {
    // 只处理 Some 的情况
}

// while let - 循环匹配
while let Some(value) = vec.pop() {
    // 持续处理直到 None
}
```

### match 中的引用

```rust
// 方式1：匹配引用
match &y {
    Some(p) => { /* p 是 &T */ }
}

// 方式2：ref 关键字
match y {
    Some(ref p) => { /* p 是 &T */ }
}
```

---

## 调试技巧

```rust
// println! 调试
println!("Debug: x = {}", x);

// dbg! 宏（推荐）
dbg!(x);
dbg!(&x, &y);  // 多个变量

// 查看类型（编译错误法）
let _: () = x;  // 编译器会告诉你 x 的真实类型
```