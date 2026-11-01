# 命令 manifest

この directory は、extension ごとの命令 form と公開 API の対応を置く。

- 対象は拡張命令のみ。base ISA は含めない。
- 1 form = 1 public function を原則とする。
- `status = "implemented" | "missing" | "excluded"` のいずれか。
- `status = "implemented"` は `test_ids` を 1 件以上持つ。値は `tests/<file>.rs::<function>` または test source の path で、`coverage.py` が実在を検査する。
- `excluded` は `exclusion_class` と `reason` を必須にする。
- `reexport` は使わない。各 form は `owner_extension` を 1 つだけ持つ。
- `source` は `tools/upstream/SOURCE.toml` の `name` を指す。
- `make manifest-pilot-check` で schema を検査する。

## 形式

```toml
schema = 1
arch = "x86_64"
extension = "avx512bmm"
module = "crate::x86::simd::avx512bmm"
kind = "fixed" # fixed | scalable | stateful | subextension
parent = ""
source = "llvm/llvm-project"
coverage = "pilot"

[[instruction]]
mnemonic = "vbitrevb"
api = "vbitrevb_v512"
widths = [512]
required_features = ["avx512bmm"]
status = "missing"
test_ids = []
```

pilot の間は `coverage = "pilot"` とし、全 extension の `missing 0` を要求しない。

上流の抽出データをまだ API 名に正規化していない manifest は `raw = true` とし、`api = ""` / `status = "missing"` で置く。`raw` の entry は API 名の検査対象外で、後で 1:1 命名に正規化する。

## 実装済み API の manifest

raw manifest を持たない architecture と x86 の固定幅 extension は、arch の README にある実装済み API 一覧から 1 API = 1 entry で manifest を起こす。macro が生成する wrapper は `generated = true` とする。`generated` でも API 名の実在検査は行う。`required_features` は extension 名を持ち、form ごとの追加 feature は上流データを正規化するときに詰める。

SVE、LSX、LASX、RISC-V の raw manifest は、上流 mnemonic を実装済み API に対応づけた entry に置き換える。複数の上流 form が 1 つの slice API に対応する場合は `forms` に元の mnemonic を残す。この正規化を行った manifest は `normalization = "mnemonic+api"` を持つ。

`status = "missing"` の entry は、上流データを API 名へ正規化する前の `raw = true` の entry だけである。raw entry は `api = ""` のままでよい。

## バッチ運用

`tools/isa/queue.toml` に次の実装バッチを置く。1 batch は 1 命令グループとし、`make test` が通るまで完了にしない。進捗報告は batch 完了時、blocker 発生時、coverage が大きく動いたときだけ行う。
