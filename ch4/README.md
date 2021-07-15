# CH4 所有權

## 4.1

### heap 和 stacks

此部分同其他程式語言

* stack: LIFO 結構，分配快
* heap: 任意擺放，分配較慢，有release問題

### 所有權規則

1. 每個變數都有一個擁有者
2. 一次只有一個擁有者
3. 擁有者離開作用域，數值即被丟棄

### 變數的作用域

> 目前都跟其他語言一樣

```rust
{
    let s = "Hello";
    // 可以使用s
}// 此處s不再有效
```

### 變數與資料：Clone & move

就一般primitive資料型別使用clone，其他用move

```rust
let x = 5;
let y = x;
// x is still avaliable here
```

非primitive

```rust
let s1 = String::from("hello");
let s2 = s1;
// s1 is unavailable here, because data had moved to s2
```

如果要使s1仍能使用則要用`.clone()`方法

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
// s1 is still avaliable here
```

### 所有權與函式

函式也是一作用域，同樣適用所有權規則

```rust
fn main() {
    let s = String::from("Hello");
    // String是堆積上物件，所有權轉移到函數內
    take_ownership(s);
    // s 不再能取用
    let x = 5;
    // i32是堆疊上物件，執行copy
    makes_copy(x);
    // 仍可存取x
}

fn take_onwership(some_string: String) {
    println!("{}", some_string);
}// some_string離開作用域，調用drop方法

fn males_copy(some_int: i32) {
    println!("{}", some_int);
}// some_int離開作用域，沒事發生
```

#### 回傳會交回所有權

```rust
fn main() {
    let s = String::from("Hello");
    // String是堆積上物件，所有權轉移到函數內
    let return_string = take_ownership(s);
    // s 不再能取用，some_string轉移到return_string
    let x = 5;
    // i32是堆疊上物件，執行copy
    makes_copy(x);
    // 仍可存取x
}

fn take_onwership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn males_copy(some_int: i32) {
    println!("{}", some_int);
}// some_int離開作用域，沒事發生
```

## 4.2 引用及借用

考慮有一函數，接收字串，回傳字串長度。
如在離開函數後需再使用該字串，則必須額外回傳原字串以保留所有權
此時會「借用」該變數來做操作

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}' 的長度為 {}。", s1, len);
}

fn calculate_length(s: &String) -> usize { 
    // &代表使用引用型態，不奪取擁有權
    s.len()
}
```

### 可變引用

在函數內修改引用的值要加`mut`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

但要注意，一個作用域裡面每個變數只能有一個可變引用（避免race condition）
下列程式碼是無法正常編譯的

```rust
 let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

同樣的，不能混用mutable和inmutable引用（避免意外更動裡面的值）

```rust
let mut s = String::from("hello");

let r1 = &s; // 沒問題
let r2 = &s; // 沒問題
let r3 = &mut s; // 很有問題！

println!("{}, {}, and {}", r1, r2, r3);
```

### dangling pointer

下列的`dangle()`函數中，s會在函數離開時釋放掉，但回傳了該指標
因此無法編譯

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

## 4.3 切片

`&str`
* 要取得一段String(陣列)的一部份時所用的資料型別
* 用法同python
* 必定為不可變引用
* `let s = "Hello World"`，此時s也是切片

```rust
let s = String::from("Hello World");
let hello = &s[0..5];
let world = &s[6..10];
```
