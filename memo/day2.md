# 座学(復習)
## 電卓レベルの言語作成
- 再帰下降構文解析法とやらを使うらしい
- ステップ1: 整数１個をコンパイルする言語
  - 要は下記を出力するようなコンパイラ
```
.intel_syntax noprefix
.globl main

main:
        mov rax, 42
        ret
```
- 豆知識
  - アセンブラの記法として、Intelの他にAT&Tとやらがある
    - 違いとしてはAT&Tは結果レジスタが第２引数に来たり、メモリ参照時に[]の代わりに()を使ったりする
```
mov rbp, rsp   // Intel
mov %rsp, %rbp // AT&T

mov rax, 8     // Intel
mov $8, %rax   // AT&T

mov [rbp + rcx * 4 - 8], rax // Intel
mov %rax, -8(rbp, rcx, 4)    // AT&T
```
- テスト作成
  - `compiler/tests/test.sh`を作成した
  - `bash -x test.sh`で実行のトレースを見れる
- makefile追加
  - ビルドスクリプトのため
# 実装
- 数字を引数に受けて、その数字を出力するアセンブリを出力するプログラムを書いた
- 足し算、引き算の式を入力した時に、式通りに演算するアセンブリを出力するプログラムを書いた
# TODO
- rustの基礎を見直したほうが良さそう
- ステップ３から
