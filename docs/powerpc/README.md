# PowerPC

対象は `powerpc64le-unknown-linux-gnu` である。

family は命令の種類に合わせて決める（`simd`、`crypto`、`matrix`、`integer` など）。公開パスは `powerpc::{family}::{extension}::{instruction}` とする。

## 実装状況

対象は各 extension の全命令である。次の表は、現在公開している命令と、
まだ公開していない命令の一覧である。値テストの列は、その extension の
値テストを実行する環境を示す。未実装命令も同じ表で管理する。

### SIMD

| extension          | 実装済み命令                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 未実装命令 | 値テスト |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------- |
| `altivec`          | `add`, `add_i16`, `add_i64`, `add_i8`, `and`, `cmpeq_i32`, `cmpgt_i32`, `fadd`, `fdiv`, `fmul`, `fsub`, `lvebx`, `lvehx`, `lvewx`, `lvx`, `mul`, `mul_i16`, `mul_i32`, `or`, `permute`, `splat_i32`, `stvebx`, `stvehx`, `stvewx`, `stvx`, `sub_i16`, `sub_i32`, `sub_i64`, `sub_i8`, `v128_load`, `v128_store`, `vaddubm`, `vaddudm`, `vadduhm`, `vadduwm`, `vand`, `vandc`, `vavg`, `vavgl`, `vcmp`, `vcmpeq`, `vcmpgt`, `vmadd`, `vmax`, `vmin`, `vmla`, `vmls`, `vmsummbm`, `vmsumshm`, `vmsumshs`, `vmsumubm`, `vmsumuhm`, `vmsumuhs`, `vmul`, `vmuleub`, `vmuleuh`, `vmuleuw`, `vmuloub`, `vmulouh`, `vmulouw`, `vmuluwm`, `vnmsub`, `vnor`, `vnot`, `vor`, `vperm`, `vpk`, `vsel`, `vsl`, `vsldoi`, `vslo`, `vsr`, `vsro`, `vsububm`, `vsubudm`, `vsubuhm`, `vsubuwm`, `vsum2sws`, `vsum4sbs`, `vsum4shs`, `vsum4ubs`, `vsumsws`, `vupk`, `vxor`, `xor` | なし       | QEMU     |
| `vsx`              | `abs_f64`, `add_f32`, `add_f64`, `div_f64`, `mul_f64`, `sqrt_f64`, `sub_f64`, `vcmpgt`, `xscvdpsp`, `xscvdpspn`, `xscvspdp`, `xscvspdpn`, `xvabs`, `xvadd`, `xvand`, `xvcmpeq`, `xvcpsgn`, `xvcvdpsxds`, `xvcvdpsxws`, `xvcvdpuxds`, `xvcvdpuxws`, `xvcvsxdsp`, `xvcvsxwsp`, `xvcvuxdsp`, `xvcvuxwsp`, `xvdiv`, `xvmax`, `xvmin`, `xvmul`, `xvneg`, `xvor`, `xvperm`, `xvrdpi`, `xvrdpic`, `xvrdpim`, `xvrdpip`, `xvrdpiz`, `xvsel`, `xvsl`, `vsldoi`, `xvsr`, `xvsub`, `xvxor`                                                                                                                                                                                                                                                                                                                                                                                | なし       | QEMU     |
| `power8-vector`    | `add_u64`, `mul_even_u16`, `mul_u32`, `sub_u64`, `vaddudm`, `vclzd`, `vmaxsd`, `vmaxud`, `vminsd`, `vminud`, `vmuleuh`, `vmuleuw`, `vmulouw`, `vmuluwm`, `vpkudum`, `vpksdss`, `vpksdus`, `vpopcntd`, `vsubudm`, `vupkhsw`, `vupklsw`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | なし       | QEMU     |
| `power9-vector`    | `abs_i32`, `neg_i32`, `neg_i64`, `vabsd`, `vabsduw`, `vctzb`, `vctzd`, `vctzh`, `vctzlsbb`, `vctzw`, `vextsb2d`, `vextsb2w`, `vextsh2d`, `vextsh2w`, `vextsw2d`, `vnegd`, `vnegw`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | なし       | QEMU     |
| `power9-altivec`   | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | なし       | 対象外   |
| `power10-vector`   | `cntlz_i32`, `vclzd`, `vpopcntd`, `vpopcntw`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | なし       | QEMU     |
| `quadword-atomics` | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | なし       | 対象外   |
| `partword-atomics` | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | なし       | 対象外   |
| `htm`              | なし                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | なし       | 対象外   |

`power10_vector` は power10 固有の命令だけを公開する。`power9_altivec` は target feature 名に対応するモジュールで、命令 API は持たない。

### 行列

| extension | 実装済み命令                                                                                                                                                   | 未実装命令 | 値テスト |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------- |
| `mma`     | `xvi4ger8`, `xvi4ger8pp`, `xvi8ger4`, `xvi8ger4pp`, `xvi8ger4spp`, `xvi16ger2`, `xvi16ger2s`, `xvi16ger2pp`, `xvi16ger2spp`, `xxmfacc`, `xxmtacc`, `xxsetaccz` | なし       | QEMU     |

### 暗号

| extension       | 実装済み命令                                                                                                                                                                                        | 未実装命令 | 値テスト |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------- |
| `power8-crypto` | `cipher`, `cipherlast`, `ncipher`, `ncipherlast`, `vcipher`, `vncipher`, `vcipherlast`, `vncipherlast`, `vpermxor`, `vpmsumb`, `vpmsumh`, `vpmsumw`, `vpmsumd`, `vsbox`, `vshasigmaw`, `vshasigmad` | なし       | QEMU     |

`vsha256h`、`vsha256h2`、`vsha256su0`、`vsha256su1` は ARM Crypto の命令で
あり、PowerPC の命令ではない。以前の表にあったこれらは削除した。PowerPC の SHA 命令は `vshasigmaw` と `vshasigmad` である。

## 実装方式

- 今回測った feature はすべて不安定または LLVM のみであり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。`<altivec.h>` を使う。
- 値は `V128` をポインタで渡す。C の `vector` 型は境界では使わない。
- C は extension ごとに分ける。`-maltivec` と `-mvsx` を使う。関数単位の `__attribute__((target("altivec")))` も使える。
- `asm!` は補助として使える。`vreg` と `vsreg` は feature を要求するため、オペランドの型を確認する必要がある。
- MMA は C の `__builtin_mma_*` を使う。`MmaAccumulator` は 64 バイトのアキュムレータを値として保持し、各命令の前後で `xxmtacc` と `xxmfacc` を実行する。命令は `&mut MmaAccumulator` を引数に取る。`xxmfacc` に相当する値の取り出しは `xxmfacc` 関数に分ける。
- `vshasigmaw` と `vshasigmad` の `ST` と `SIX` は const generics で受け、C は 32 通りの即値を持つ switch に展開する。
- `vpmsum*` は C の `vec_pmsum_be` を型ごとに使い、`vpermxor` は `vec_permxor` を使う。

## ゼロコスト

- 結果は [ゼロコストの確認](../zero-cost.md) を見る。
- `.TOC.` は lld が `addReservedSymbols` で定義するが、LTO 後のオブジェクトにだけ参照があると定義されない。`.TOC.` を参照する非 LTO オブジェクトをリンクに加えると解決する。`.quad .TOC.` を持つ小さなアセンブリオブジェクトで確認した。
- ELFv2 ABI のベクタ引数と戻り値は、C の関数境界で確認する必要がある。
- MMA はアキュムレータをメモリに保持するため、命令の前後で 64 バイトの store/load が残る。ゼロコストの対象にしない。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- LTO のリンクでは、`.quad .TOC.` を持つ非 LTO オブジェクトを追加する。`toc.s` を `clang -c` でコンパイルし、リンク引数に渡す。
- clang 23.1.1 で、`-maltivec -mvsx` の `vec_add` が `vadduwm` を出すことを確認した。
- `__attribute__((target("altivec")))` をファイル全体の `-maltivec` なしで付けた関数も、`vadduwm` を出した。
- MMA は `-mcpu=power10 -mmma` で C がコンパイルでき、`xvi4ger8`、`xxmfacc`、`xxsetaccz` を出した。QEMU 11.1.2 の `-cpu power10` で値テストを実行した。

## 実装の順序

1. `altivec`、`vsx`
2. `power8-vector`、`power9-vector`、`power10-vector`
3. `power8-crypto` の AES、SHA、`vpmsum*`、`vpermxor`
4. `quadword-atomics`、`partword-atomics`
5. `mma`

## 未対応

- `quadword-atomics`、`partword-atomics`、`htm`。同期・制御命令のため、
  値 API にしない。
- ELFv2 のベクタ引数と戻り値の ABI は、C の境界ではポインタだけを使う現在の
  方式で回避している。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
