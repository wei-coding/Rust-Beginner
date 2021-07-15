# ch8 集合

## ch8.1 向量

### 向量可以調整大小，任意增加/刪減元素

```rust
let v: Vec<i32> = Vec::new();
// 或利用巨集
let v = vec![1, 2, 3];
```

### 更新向量

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### 釋放向量也會釋放元素

### 讀取向量元素

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
// 取得第三個元素
// 若超出長度會噴panic

match v.get(2) {
    Some(third) => println!("第三個元素是{}", third),
    None => println!("第三個元素不存在"),
}
```

* 在持有元素時無法更改向量內容

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
// v.push(6);
// 上面那行執行會噴錯，即不能同時持有immutable ref和mutable ref
println!("第一個元素是 {}", first);
```

### 遍歷元素

```rust
// 列印元素值
let v = vec![1, 2, 3];
for i in &v {
    println!("{}", i);
}
// 或是可以改變值
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 50;
}
```

### 用enum存放多種值

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("藍色")),
    SpreadsheetCell::Float(10.12),
];
```

## 8.2 字串與UTF-8

### 字串的原理（略）

實際上String就是`Vec<u8>`的封裝

### 建立字串

```rust
let mut s = String::new();

// 建立特定內容值的字串
let s = String::from("Hello!");
// 是UTF-8編碼，任意語言皆可輸入
```

### 修改字串

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
// push_str不會取得所有權，s2仍可用
s1.push('!');
// push可以接上單一字元
```

### String不允許索引

有一中文字串如下

```rust
let s = String::from("哈囉，你好嗎");
```

存在u8裡共會有18個byte，但如果取出第一個並不會得到"哈"，而是不可預期的值

因此String不可以用索引來存取，需要的話應使用遍歷

```rust
for c in "哈囉，你好嗎".chars() {
    println!("{}", c);
}
```

## 8.3 hash map

### 建立新的hash map

* 直接建立

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("籃隊"), 10);
scores.insert(String::from("黃隊"), 50);
```

* 從`Vec`建立

```rust
use std::collections::HashMap;
let teams = vec![String::from("籃隊"), String::from("黃隊")];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

**其中`HashMap<_, _>`是必要的，`_`代表自動推定後面的型別**

### 對於有所有權的物件來說，插入HashMap會耗用所有權

### 取得HashMap裡面的值

* 直接使用get方法

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("籃隊"), 10);
scores.insert(String::from("黃隊"), 50);

let team_name = String::from("籃隊");
let score = scores.get(&team_name);
```

* 利用for迴圈遍歷

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("藍隊"), 10);
scores.insert(String::from("黃隊"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

此時不一定會依照順序印出，每次狀況可能不一樣

### 更新HashMap

* 可以重複插入一個對應，會以新的為主

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("藍隊"), 10);
scores.insert(String::from("藍隊"), 25);

println!("{:?}", scores);
//  {"籃隊": 25}
```

* 只在鍵沒有值時插入

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("藍隊"), 10);

scores.entry(String::from("黃隊")).or_insert(50); // 這行會更新
scores.entry(String::from("藍隊")).or_insert(50); // 這行不會

println!("{:?}", scores);
// {"黃隊": 50, "藍隊": 10}
```

* 依據舊值更新數值

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    // or_insert會回傳&mut V
    *count += 1;
}

println!("{:?}", map);
```

