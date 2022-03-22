# 座学(復習)
## 機械語のアセンブラ
### CPUとメモリ
- 現在実行中の命令のアドレスをPC(Program Counter)またはIP(Instruction Pointer)といい、CPUに保持する
  - Intel,AMDのCPUには64bit整数を保持できる領域(レジスタ)が16個ある
  - CPUはメモリ上の機械語を読み取ってレジスタ上の値を読み書きして処理を進めていく
- 特定の機械語の命令の総称を`命令セットアーキテクチャ(ISA)`といい、ISAはCPUごとに構築して良い
  - が、各々が自由に設計すると互換性がなくなるので`x86-64`、`arm`などが主に使われている
### アセンブラ
- 機械語は人間には読めないのでアセンブリ言語を使うことが多い
- それでも厳しいので実際にはより高級な言語からアセンブリを吐き出している
- 試しにrustのhello,worldプログラムから生成されるバイナリを逆アセンブルしてみたらくそ長い
```
$ objdump -d -M intel target/debug/compiler | wc -c
3707995
```
- アセンブリ言語の拡張子は`.s`
- シンプルに`return 42`をするだけのプログラムだとアセンブリのかなりシンプルっぽい
```
.intel_syntax noprefix
.globl main
main:
        mov rax, 42
        ret
```
- 関数呼び出しを含む場合
  - main関数のみだった場合と違って関数ごとにアセンブリが生成され、メモリ上での移動先、移動元アドレスをスタックに積んでおかないと行けない
  - 下記例の場合は、call plusの次のretをスタックに積んでおき、その後call先のアドレスにジャンプ、plus側のretに到達したらスタックからアドレスをポップ(mainのretのアドレス)されてmain側に戻ってくる
```
.intel_syntax noprefix
.globl plus, main

plus:
        add rsi, rdi
        mov rax, rsi
        ret

main:
        mov rdi, 3
        mov rsi, 4
        call plus
        ret
```
# 実装
- rustが動くdockerコンテナを用意した
# TODO
- まだ座学なのでなし
