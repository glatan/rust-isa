# スケーラブルベクタと状態を持つ拡張の設計

この文書は、SVE、RVV、SME、AMX、PowerPC MMA の API 設計である。実装はまだない。

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

**決定**: スライス API だけを提供する。命令ごとの API は出さない。

```rust
pub fn add_f32(a: &[f32], b: &[f32], out: &mut [f32])
```

- C は `svcntw()` で VL を得て、`svld1` / `svadd` / `svst1` を使う。
- Rust 側に VL やレジスタ型を出さない。VL は C が決める。
- 引数の長さが揃っていない場合は panic する。境界の扱いは各関数の rustdoc に書く。
- AArch64 で確認済み。C のループは `#[target_feature(enable = "sve")]` の Rust カーネルへインラインされ、`cntw`、`whilelt`、`ld1w`、`add z0.s`、`st1w` が並び、`call` が無かった。

却下した案:

- 不透明なレジスタ型: ABI とゼロコストの保証が難しい。
- 命令ごとの関数 + ポインタ: 各命令がメモリを読み書きし、ゼロコストにならない。

## RVV

**決定**: 固定長 `V128` の命令 API と、スケーラブルなスライス API を分ける。

- 固定長 `V128` は `vadd.vv` / `vmul.vv` でゼロコストを確認済み。
- スケーラブル API はスライスを受け取り、VL は C が `vsetvl` で決める。
- `Vtype` は公開しない。要素型と LMUL は関数ごとに固定する。
- `v` は不安定なため、利用者はクレート全体を `-C target-feature=+v` でコンパイルする。
- RISC-V で確認済み。C のループはカーネルへインラインされ、`vsetvli`、`vle32.v`、`vadd.vv`、`vse32.v` が並び、`call` が無かった。

## SME

**決定**: `SmeGuard` でストリーミングモードを管理する。

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

- `__arm_streaming` は宣言子の後ろに置く。関数は `smstart` / `smstop` を出さないため、呼び出し側が guard を持つ。
- ZA のタイルは対象外とする。`SmeGuard` はストリーミング状態だけを管理する。

## AMX

**決定**: `TileConfig` と `AmxGuard` でタイル状態を管理する。

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
- OS のタイル状態の許可は、クレートの外で設定する。クレートは許可を行わない。
- `TileConfig` の構築は利用者が行う。最初は生のバイト列として扱う。

## PowerPC MMA

**決定**: `MmaAccumulator` でアキュムレータを管理する。

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
- `xxmfacc` に相当する値の取り出しは、別の関数にする。

## 実装状況

- SVE のスライス API: 実装済み。`add` / `mul` の f32 / f64 / i32。QEMU-AArch64 で
  値テストを実行する。C のループは `#[target_feature(enable = "sve")]` の
  カーネルへインラインされる。
- RVV のスライス API: 実装済み。`add` / `mul` / `sub` / `div` / `min` / `max`、比較、load / store の f32 / i32。QEMU-RISC-V で値テストを実行する。VL は C の `vsetvl` が決める。
- SME: `SmeGuard` とストリーミングの `add` / `mul` (f32) を実装した。ZA は対象外。 no_std の fixture は Arm SME ABI ランタイムの `__arm_get_current_vg` をリンク
  しないため、SME の値テストはまだない。
- AMX: `TileConfig`、`AmxGuard`、`tilezero` を実装した。タイル load / store と matrix multiply は未実装。OS のタイル状態の許可が必要なため、値テストはまだない。
- PowerPC MMA: `MmaAccumulator`、`xvi4ger8` 系、`xxmfacc`、`xxmtacc`、`xxsetaccz` を実装した。アキュムレータは 64 バイトの値として保持し、各命令
  の前後でハードウェアのアキュムレータへ移動する。QEMU の power10 で値テスト
  を実行する。

順序は、SVE / RVV のスライス API、SME / AMX / MMA の guard の順とする。

## 未決定

- SVE のスライス API の境界条件（長さが違う場合に panic するか、短い方に合わせるか）。
- RVV のスライス API で、要素型ごとにどの LMUL を使うか。
- AMX の `TileConfig` の構築 API。
- guard を safe にできる範囲。
