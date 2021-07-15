# ch5 結構體

## ch5.1 結構體定義

如同C語言，結構體可以包裹不同型別，並且有名稱
下面展示基本的結構體

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

宣告一個使用者可以使用

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

當然，如果希望是mutable可以在`let` 後加上 `mut`

同樣，可以用函數回傳struct

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

在使用函數建立struct時，可以省略掉跟參數同名的部分，如下：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

也可以使用別的結構體來製作新的結構體

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // others same as user1
};
```

### 無名結構體

如點、色彩等

```rust
struct Color(u32, u32, u32);
struct Point(i32, i32, i32);

let white = Color(255, 255, 255);
let origin = Point(0, 0, 0);
```

**目前結構體內還無法包裹引用，會在第十章補充**

## 5.2 結構體程式範例

## 5.3 method

* `impl`用於定義method
* method定義時可傳入`&self`作為參數，指結構本身
* 相較於C/C++，引用不需使用`->`作為解引用符號，在這邊會自動解引用，也就是`p1.some_method(&p2);`等價於`(*p1).some_method(&p2);`