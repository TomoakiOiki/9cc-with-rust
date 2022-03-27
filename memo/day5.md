# 座学
- [これ](https://qiita.com/syuuu/items/ac3f72370ee07998be70)を読んだ
- [ユニットテストの書き方](https://doc.rust-jp.rs/book-ja/ch11-01-writing-tests.html)把握
- [よく使うメソッド](https://www.amusement-creators.info/articles/rust/introduce_iterator_methods/)
  - rustも割とワンライナーするっぽい
  - peekとかFPSのjump peekのpeek(覗き見る)だったのね・・・。
- [CopyとCloneの違い](https://yossan.hatenablog.com/entry/2020/11/15/130206)
# 実装
- コードの見通しが悪くなってきたので一旦モジュールを分けることにした
  - rustはそのへんのお作法もちょっと特殊
  - 例えば`hoge/fuga.rs::{fn piyo}`みたいな感じでコードを書いたら、`src/hoge.rs`に`mod fuga`と書かないと`piyo`を`main.rs`から使えない
- tokenize関数がちゃんと動かない
  - strtol周りがバグってそう
  - 動いた
# 感想
- 今日は主に実装だったので座学は少なめ
- tokenize自体はうまく出来てるが、単方向リストをたどって処理していくだけなのにRustだとやたら複雑になる・・・
  - 正直慣れてる言語だったら1時間もかからん実装に8時間くらいかかってる
- 明日こそステップ３を超えたい
