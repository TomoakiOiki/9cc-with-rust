# 座学
- ステップ4 エラーメッセージの改良
  - こういうのを出すようにする関数を追加する
```
$ ./9cc "1+3++" > tmp.s
1+3++
    ^ 数ではありません

$ ./9cc "1 + foo + 5" > tmp.s
1 + foo + 5
    ^ トークナイズできません
```
# 実装
- ステップ3,4完了
- 現状の出力集
- `cargo run "1+2+3"`
```
.intel_syntax noprefix
.global main
main:
  mov rax, 1
  add rax, 2
  add rax, 3
  ret
```
- `cargo run "1+2               -                                    3"`
```
.intel_syntax noprefix
.global main
main:
  mov rax, 1
  add rax, 2
  sub rax, 3
  ret
```
- `cargo run "1++"`
```
1++
  ^ 数値ではありません
```
- `cargo run "1+*"`
```
1+2*3
   ^ 不明なトークンです
```
# 感想
- ステップ3やっと抜けれた・・・！
- ステップ4は軽かった
- Rustと少し仲良くなれた
- 明日から本格的な構文解析周りの実装が始まるはず。
