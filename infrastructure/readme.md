# infrastructure

インフラ層を記述する．

## 特徴

- sea-ormを用いている．
- sea-ormはモジュールを使ってモデルを作成することになるため、各リポジトリをジェネリックなパラメーターを用いてまとめることは困難であり，やるとしてもマクロを使うことになる．

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
