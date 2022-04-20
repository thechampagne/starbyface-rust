# StarByFace

[![](https://img.shields.io/github/v/tag/thechampagne/starbyface-rust?label=version)](https://github.com/thechampagne/starbyface-rust/releases/latest) [![](https://img.shields.io/github/license/thechampagne/starbyface-rust)](https://github.com/thechampagne/starbyface-rust/blob/main/LICENSE)

Celebrity look alike face-recognition API for **Rust**.

### Download
[Crates](https://crates.io/crates/starbyface/)

Add the following line to your Cargo.toml file:

```
starbyface = "1.0.0"
```

### Example

```rust
use starbyface::StarByFace;

fn main() {
    let star = StarByFace::new("http://image-url.example");
    println!("{:?}",star.get_data().unwrap())
}
```

### License

StarByFace is released under the [Apache License 2.0](https://github.com/thechampagne/starbyface-rust/blob/main/LICENSE).

```
 Copyright 2022 XXIV

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
```