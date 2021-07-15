# ch6 Enumerate

## 6.1 enum

* 指固定幾種其一的型態
* 如IP只有IPv4和IPv6兩種
* 可定義如下
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let v4 = IpAddr::V4(127, 0, 0, 1);
let v6 = IpAddr::V6(String::from("::1"));
```
**實際上內建函式庫有定義IP address的樣式**

* `Option`作為空值
```rust=
let some_number = Some(5);
let some_string = Some("一個字串");

let absent_number: Option<i32> = None;
```

## 6.2 match

* 類似switch...case...
* 可用於任意型別
* 語法
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
* 所有結果必須都被配對完成，若要表示其他可用`_`，表示不執行可用`()`

## 6.3 if let

* 如果只用來處理一種狀況用`match`太麻煩
* `if let`語句可以減少行數
```rust
 let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("三"),
    _ => println!("不是三"),
}
// equals to
 if let Some(3) = some_u8_value {
    println!("三");
}else {
    println!("不是三");
}
```