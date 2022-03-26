# 座学
- [Tour of Rust](https://tourofrust.com/00_ja.html)でざっと言語仕様把握
  - golangといい最近の言語はplaygroundつきのチュートリアルあって良い
  - Result型、よく使うので覚えておきたい
    - match文でOkならvalue,Errならエラーを返す記述のシンタックスシュガーとして`hoge?`という記法がある
    - 値を取り出すのみで良ければ`hoge.unwrap()`でも良い
  - 関数に指定するライフタイムを理解した
    - `do_something`の返り値`x`のライフタイムと`foo_b`のライフタイムを`'b`にすると、`x`がその後の`println`で使われたあとにdropするタイミングで`foo_b`もdrop
```
// foo_b と戻り値はライフタイムを共有
// foo_a のライフタイムは別
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // ここから先は foo_b のライフタイムしか存在しないため、
    // foo_a はここでドロップ
    println!("{}", x);
    // x はここでドロップ
    // foo_b はここでドロップ
}
```
  - スタティックライフタイム
    - プログラムの開始から終了まで存在するリソースを作る時に使用
    - `'static`を指定する
    - スタティックライフタイムを指定した変数はdropされない
  - 文字列から各種型への変換には`str.parse::<T>()`を使う
    - Resultが返ってくるのだけ注意
  - Rustには継承がない
  - Rustのカプセル化
    - `impl Struct名`で構造体に関数を追加する
    - 引数に`self`を受け取る
      - `&self`or`&mut self`
  - トレイト
    - `train hoge`で抽象インターフェースみたいなのを作れる
    - `impl hoge for fuga`でfugaにhogeトレイトを実装する
    - トレイトは他のトレイトからメソッドを継承できる
      - `trait Hoge: Fuga`
    - 動的 vs 静的ディスパッチ
      - `&dyn Trait`で渡されたTraitを実装しているインスタンス
  - Box
    - データをスタックからヒープに移動するデータ構造
  - `.`オペレーター
    - `let hoge = &&&a`の値を取り出す時に`hoge.value`とできる
      - `(***hoge).value`のシンタックスシュガー
  - モジュール
    - Rustのプログラム、ライブラリはすべてクレート
    - クレートはモジュールの階層で構成されている
    - プログラムの場合は`main.rs`はルートモジュール
    - ライブラリの場合は`lib.rs`がルートモジュール
    - [コミュニティ作成のクレートはここ](https：//crates.io)
    - モジュール`foo`の表現方法
      - `foo.rs`
      - `foo/mod.rs`
    - `mod foo;`を親モジュールに書き込む
    - インラインモジュール
      - `mod hoge`で始まり、`use super::*;`で親モジュールを使用できる
      - テストを書く際などに使う
    - exporting
      - `pub`キーワードを付けることでモジュールのメンバーにアクセス可
# 実装
- 実装はなし
# 感想
- スマートポインタあたりは別途勉強する必要がありそう
- 明日は実装メインで進める
