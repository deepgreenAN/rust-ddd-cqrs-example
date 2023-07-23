# イベントバス

- 非同期ランタイムを用いたイベントバス
- RabbitMQ(lapin)を用いたイベントバス

## 参考とするリポジトリ

- [JAD3N/event-bus](https://github.com/JAD3N/event-bus/tree/master)
  - ディスパッチ・サブスクライブのAPIをマクロで行う
  - イベントトレイトとしてキャンセルの状態を取得するメソッドを定義している．
  
- [dimitri-br/Simple-Event-Bus](https://github.com/dimitri-br/Simple-Event-Bus)
  - どんな型もイベントとして取ることができる．
- [sachanganesh/eventador-rs](https://github.com/sachanganesh/eventador-rs)
  - リングバッファを用いている
- [sizpounder/tram](https://github.com/sixpounder/tram)
  - 非同期ランタイムを使わない？
- [WeirdPtr/ebus-rs](https://github.com/WeirdPtr/ebus-rs)
  - クレートの説明でプルリクエストのやり方まで書いてあって親切

- [primait/event_sourcing.rs](https://github.com/primait/event_sourcing.rs)
  - RabbitMqを用いたイベントバス・kafkaを用いたイベントバスを実装している．
