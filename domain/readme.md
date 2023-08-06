# domain

ドメイン層を記述する．

## features

フロントエンドでもormを利用するならfeaturesとする必要はない．

- server(+orm)
- orm

## 変更(追加)する部分

- アグリゲイト・エンティティやバリューオブジェクトの追加・ドメインロジックの追加など．
- ドメインイベントの追加

## test

```shell
cargo test
```
