# 座学
## Rustメモ
### 文字列の扱いが複雑
  - charとStringと&strの関係性
    - `"abc"`は&str(文字列リテラル)
    - `&str -> String`は`String::from("abc")`でOK
    - `String -> &str`は`let s: &str = &s1`でOK(s1がString)
    - `char -> String`は`c.to_string()`
    - `Vec<char> -> String`は`c.iter().collect()`
    - `String or &str -> Vec<char>` は`"abcd".chars().collect();`
    - Stringにcharを追加`s.push("a")`
    - &strに直接charの追加は不可なのでStringに変換する
    - `Vec<char>`と`Vec<char>`の結合`a.extends(b.iter())`か`a.append(&mut b.clone())`
### Lifetime
  - Dangling Referenceを防ぐ仕組み
  - 下記の様に異なるLifetimeの変数を扱おうとすると起こられる
```
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```
  - 関数の引数に２つLifetimeが異なる変数が渡ってきたとして、そのどちらかを返すとした場合にその関数が返す値のLifetimeがわからなくなる
  - その対策としてLifetime Annotationを付ける
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
  - こうすると、下記みたいな使い方が怒られる(これおもろい)
```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
	// resultはstring1なので
        result = longest(string1.as_str(), string2.as_str());
    }
    // スコープ抜けたここでも有効なはずだけど
    println!("The longest string is {}", result);
}

// `aのlifetimeは短い方のstring2を用いて、Borrow Checkerがチェックしてる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
## 電卓レベルの言語作成
- ステップ3 トーカナイザーの導入
  - 現状の実装だと空白文字列が入ると動かない
    - 今回は空白文字列は除外
    - 演算子、数値、終端文字列の３種類に対応するやつ
# 実装
- Rustの特殊な部分の把握
- トーカナイザーの実装（途中）
  - Rustに慣れてなくてコンパイルが通らない
# TODO
- rustの基礎
- 引き続きステップ３
