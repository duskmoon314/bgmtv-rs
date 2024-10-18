# bgmtv

bgmtv is an API wrapper for [bangumi.tv](https://bgm.tv) based on [reqwest](https://crates.io/reqwest).

## Usage

```rust
use bgmtv::prelude::*;

tokio_test::block_on(async {
    let client = Client::builder()
    .user_agent("duskmoon/bgmtv/0.1.0 (https://github.com/duskmoon314/bgmtv-rs)")
    .build()
    .expect("Failed to build client");

    let subject = client.get_subject(3559).await.expect("Failed to get subject");

    assert_eq!(subject.name, "とある魔術の禁書目録")
});
```