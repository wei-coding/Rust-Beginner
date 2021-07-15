# 錯誤處理

## 9.1 panic用於無法修復的錯誤

舉例：

```rust
fn main() {
    let v = vec![1, 2, 3];

    println!("{}", v[99]};
}
```

明顯超過vec的範圍了，會噴panic

## 9.2 Result用於可修復的錯誤

### 一般處理

對於有可能失敗的函數會回傳`Result`枚舉，其中有兩的變體

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

例如開啟一個檔案

```rust
use std::fd::File;

fn main() {
    let f = File::open("hello.txt");
}
```

此時`f`會是`Result`型別，可以用`match`來處理

```rust
use std::fd::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("開始時發生錯誤：{:?}", error),
    };
}
```

也可以針對不同的錯誤來做處理

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("建立檔案時發生問題：{:?}", e),
            },
            other_error => {
                panic!("開啟檔案時發生問題：{:?}", other_error)
            }
        },
    };
}
```

當然也有更進階的技巧（第十三章會講）

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("建立檔案時發生問題：{:?}", error);
            })
        } else {
            panic!("開啟檔案時發生問題：{:?}", error);
        }
    });
}
```

### 強制panic

