# application/common

application層のバックエンド・フロントエンドに共通する部分．コマンドはここで定義する．参照コマンドの作成はマクロで記述できそう．現状ormに依存しているため、依存をdomainやinfrastructureに移動すべき？．commonだけormに依存してよい？．

## features

- frontend
- server

## 変更(追加)する部分

- コマンドの追加．場合によっては対応する参照コマンドも定義する．
