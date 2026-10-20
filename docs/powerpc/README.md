# PowerPC

対象は `powerpc64le-unknown-linux-gnu` である。

family は命令の種類に合わせて決める（`simd`、`crypto`、`integer` など）。公開パスは `powerpc::{family}::{extension}::{instruction}` とする。

## 実装状況

なし。

## 実装方式

- 今回測った feature はすべて不安定または LLVM のみであり、cfg ゲート方式を使えない。関数を無条件に置き、`unsafe fn` として公開する。
- C + clang を主方式とする。LTO は wasm32 と native のゼロコストに使う。`<altivec.h>` を使う。
- 値は `V128` をポインタで渡す。C の `vector` 型は境界では使わない。
- C は extension ごとに分ける。`-maltivec` と `-mvsx` を使う。関数単位の `__attribute__((target("altivec")))` も使える。
- `asm!` は補助として使える。`vreg` と `vsreg` は feature を要求するため、オペランドの型を確認する必要がある。

## ゼロコスト

- ゼロコストを確認した。`vadduwm` と `vmuluwm` が並び、`call` と命令間の store/load が無かった。
- `.TOC.` は lld が `addReservedSymbols` で定義するが、LTO 後のオブジェクトにだけ参照があると定義されない。`.TOC.` を参照する非 LTO オブジェクトをリンクに加えると解決する。`.quad .TOC.` を持つ小さなアセンブリオブジェクトで確認した。
- ELFv2 ABI のベクタ引数と戻り値は、C の関数境界で確認する必要がある。

## ツールチェーン

- clang、llvm-ar、llvm-objdump。
- LTO のリンクでは、`.quad .TOC.` を持つ非 LTO オブジェクトを追加する。`toc.s` を `clang -c` でコンパイルし、リンク引数に渡す。
- clang 23.1.1 で、`-maltivec -mvsx` の `vec_add` が `vadduwm` を出すことを確認した。
- `__attribute__((target("altivec")))` をファイル全体の `-maltivec` なしで付けた関数も、`vadduwm` を出した。
- MMA は `-mcpu=power10 -mmma` で C がコンパイルでき、`xvi4ger8` と `xxmfacc` を出した。アキュムレータの状態を持つ。

## 実装の順序

1. `altivec`、`vsx`
2. `power8-vector`、`power9-vector`、`power10-vector`
3. `power8-crypto`
4. `quadword-atomics`、`partword-atomics`
5. `mma`（LLVM のみ。型と状態の設計が必要）

## 未決定

- ELFv2 のベクタ引数と戻り値の ABI。
- MMA のタイル状態を API に出すか。

## 文書

- [実装可否](feasibility.md)
- [実装方針](../policy.md)
