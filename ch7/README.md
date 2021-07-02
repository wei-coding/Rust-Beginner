# ch7 套件管理

* 使用`pub`關鍵字設為公開
* 跟目錄上的`mod`是公開的，`fn`不是
* 其他預設皆為不公開
* `super`可取得上級`mod`
* `struct`裡可以混和公開和非公開
* `enum`設為公開時，所有變體為公開


* `use`語法
```rust=
use std::{cmp::Ordering, io};
// 可以單行引用多個
use std::io::{self, Write};
// 等同於
// use std::io;
// use std::io::Write;
use std::collections::*;
// collections下的所有都引用
```

* 可以分開檔案，但要確保模組樹不變
```
src
|   lib.rs  front_of_house.rs
front_of_house
|   hosting.rs
```