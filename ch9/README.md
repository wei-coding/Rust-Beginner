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

如果確定在出問題時要直接panic，可以使用`unwrap`和`expect`方法，其中`expect`可以自訂錯誤訊息內容

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // or
    let f = File::open("hello.txt").expect("開啟 hello.txt 失敗");
}
```

### 傳播錯誤（在函數間）

像python和java一樣，可以把exception一層層往外拋

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

* `?`操作符

在回傳`Ok`時繼續下一行，`Err`時回傳錯誤

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

也可以直接繼續接下去

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```