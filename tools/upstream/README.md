# 上流データ

この directory は、命令 manifest の候補生成に使う抽出データを置く。

- `SOURCE.toml` が版・commit・license・SHA256 の正本。
- build 時にネットワークへ取りに行かない。
- license と attribution は各 source の directory に保持する。
- 同梱可否と参照のみの仕様書は `docs/third-party.md` に書く。
- データを更新したら `SOURCE.toml` の SHA256 を更新し、`make manifest-pilot-check` を通す。

## 現在の pilot

- `xed/`: AMX-FP8 の 4 form
- `llvm/`: AVX512BMM の 7 form（encoding の一部を含む）
- `powerisa/`: Power10 MMA の 14 form
- `acle/`: SVE instruction table 824 row（CC-BY-SA-4.0）
- `loongarch/`: LSX 717 / LASX 735 entry（CC-BY-4.0）
- `riscv/`: Zba / Zbb / Zbc / Zbkb / Zbkc / Zbkx / Zbs / V / Zvbb / Zvbc / Zvkg / Zvkn / Zvks / Zfa / Zkn / Zks の 518 instruction（BSD-3-Clause）
