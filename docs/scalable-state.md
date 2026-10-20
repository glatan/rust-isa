# スケーラブルベクタと状態を持つ拡張

この文書は、SVE、RVV、SME、AMX、PowerPC MMA の API 設計の検討案である。実装はまだない。

## 共通の考え方

- 固定長の `V128` / `V256` で表せない値は、命令ごとの関数にしない。
- 状態を持つ拡張は、状態の開始と終了を guard 型で表す。
- guard の作成は `unsafe fn` とする。CPU の対応と状態の初期化は呼び出し側の責任である。
- guard はスレッドをまたいで移動しない。`PhantomData<*const ()>` などで `Send` と `Sync` を外す。
- guard の `Drop` で状態を終了する。panic 時も終了する。
- guard の作成中に別の guard を作らない。入れ子を許さない。
- guard を持つ関数だけが、その状態を要求する命令を呼べる。
- `Drop` は panic しない。

## SVE

SVE のベクタ長は実行時に決まる。固定長の値型では表せない。

検討案:

1. **スライス API**（推奨）
   - `fn svadd_f32(a: &[f32], b: &[f32], out: &mut [f32])` のように、スライスを受け取る。
   - C は `svcntw()` で VL を得て、`svld1` / `svadd` / `svst1` を使う。
   - 命令ごとの API ではなく、SVE のベクタ長に依存しない関数になる。
   - AArch64 で確認した。C のループは `#[target_feature(enable = "sve")]` の Rust カーネルへインラインされ、`cntw`、`whilelt`、`ld1w`、`add z0.s`、`st1w` が並び、`call` が無かった。
2. **不透明なレジスタ型**
   - `SReg` を最大サイズのバイト列で表し、VL を別に持つ。
   - ABI とゼロコストの保証が難しい。
3. **命令ごとの関数 + ポインタ**
   - 各命令がメモリを読み書きする。ゼロコストにはならない。

SVE は 1 のスライス API を先に設計する。命令ごとの API は出さない。

## RVV

RVV の固定長 `V128` 部分集合は、`vadd.vv` / `vmul.vv` でゼロコストを確認した。`v` は不安定なため、利用者はクレート全体を `-C target-feature=+v` でコンパイルする。

スケーラブルな API は別に設計する。

- `Vl(usize)` と `Vtype` を引数に取る。
- 値はスライスで渡し、C が `vle` / `vse` を使う。
- 命令ごとの関数にする場合も、`vl` と `vtype` を明示する。
- 固定長 `V128` とスケーラブル API は別のモジュールに分ける。
- RISC-V で確認した。C のループは Rust のカーネルへインラインされ、`vsetvli`、`vle32.v`、`vadd.vv`、`vse32.v` が並び、`call` が無かった。

## SME

SME はストリーミングモードと ZA の状態を持つ。

- `SMSTART` で入り、`SMSTOP` で出る。
- clang の `__arm_streaming` は宣言子の後ろに置く。関数は `smstart` / `smstop` を出さないため、呼び出し側が guard を持つ。
- 検討案:

```rust
pub struct SmeGuard {
    _not_send: PhantomData<*const ()>,
}

impl SmeGuard {
    /// Enter streaming mode. The CPU must support SME.
    pub unsafe fn enter() -> Self { /* SMSTART */ }
}

impl Drop for SmeGuard {
    fn drop(&mut self) { /* SMSTOP */ }
}

/// The caller must hold a guard.
pub unsafe fn add_f32(_guard: &SmeGuard, a: &[f32], b: &[f32], out: &mut [f32]) { /* ... */ }
```

- ZA のタイルは `&mut SmeGuard` または別の型で表す。

## AMX

AMX はタイル設定とタイル解放の状態を持つ。

- `LDTILECFG` で設定し、`TILERELEASE` で解放する。
- OS がタイル状態を許可している必要がある。
- 検討案:

```rust
#[repr(C, align(64))]
pub struct TileConfig([u8; 64]);

pub struct AmxGuard {
    _not_send: PhantomData<*const ()>,
}

impl AmxGuard {
    /// The CPU and OS must support AMX tile state.
    pub unsafe fn enter(_config: &TileConfig) -> Self { /* LDTILECFG */ }
}

impl Drop for AmxGuard {
    fn drop(&mut self) { /* TILERELEASE */ }
}
```

- タイル命令は `&AmxGuard` を引数に取る。
- OS のタイル状態の許可は、クレートの外で設定する。

## PowerPC MMA

PowerPC の MMA はアキュムレータの状態を持つ。

- `xvi4ger8` などはアキュムレータを更新する。
- `xxmfacc` でアキュムレータをベクタレジスタへ移す。
- 検討案:

```rust
pub struct MmaAccumulator {
    _not_send: PhantomData<*const ()>,
}

impl MmaAccumulator {
    /// The CPU must support MMA.
    pub unsafe fn new() -> Self { /* ... */ }
}

/// The caller must hold the accumulator.
pub unsafe fn xvi4ger8(_acc: &mut MmaAccumulator, a: &[u8], b: &[u8]) { /* ... */ }
```

- 命令は `&mut MmaAccumulator` を引数に取る。

## 未決定

- SVE のスライス API の形（VL を明示するか、C に任せるか）。
- RVV の `Vtype` の表現。
- SME の ZA タイルの型。
- AMX のタイル設定の型と、OS サポートの確認方法。
- guard を safe にできる範囲。
