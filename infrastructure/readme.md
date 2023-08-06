# infrastructure

インフラ層を記述する．

## test

マイグレーション

```shell
cd ../migration
source ../env.nu
cargo run -- fresh
```

```shell
source ../env.nu
cargo test
cargo test -- --ignored
```
