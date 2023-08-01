# イベントバス

- ✅ 非同期ランタイムを用いたイベントバス
- ⬜ RabbitMQ(lapin)を用いたイベントバス

## 特徴

- 

## 参考とするリポジトリ

- [JAD3N/event-bus](https://github.com/JAD3N/event-bus/tree/master)
  - ディスパッチ・サブスクライブのAPIをマクロで行う
  - イベントトレイトとしてキャンセルの状態を取得するメソッドを定義している．
  - グローバルなイベントハンドラを保持するために型へのdowncastを用いている．
  - 非同期なイベントハンドラを持つことはできない．
  
- [dimitri-br/Simple-Event-Bus](https://github.com/dimitri-br/Simple-Event-Bus)
  - どんな型もイベントとして取ることができる．シングルスレッドが前提．
- [sachanganesh/eventador-rs](https://github.com/sachanganesh/eventador-rs)
  - リングバッファを用いている
- [sizpounder/tram](https://github.com/sixpounder/tram)
  - 非同期ランタイムを使わない？
- [WeirdPtr/ebus-rs](https://github.com/WeirdPtr/ebus-rs)
  - クレートの説明でプルリクエストのやり方まで書いてあって親切

- [primait/event_sourcing.rs](https://github.com/primait/event_sourcing.rs)
  - RabbitMqを用いたイベントバス・kafkaを用いたイベントバスを実装している．
