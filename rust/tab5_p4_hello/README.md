# Rust Tab5 P4 Hello

独立した調査用ESP-IDF Rustバイナリです。既存の `Tab5_thinclient` をRustへ移植する
ものではありません。現在はP4向けビルドとシリアルHello Worldだけを対象とし、
Tab5への書き込みや周辺機器操作は行いません。

## 構成

- MCU: `esp32p4`
- ESP-IDF: `v5.5.5`
- Rust target: `riscv32imafc-esp-espidf`
- Framework: `esp-idf-svc` 0.53.0
- Toolchain: nightly Rust + `rust-src`

## 開発環境

ESP-IDF Rust公式テンプレートの前提に従い、ホスト側のESP-IDF build prerequisites
(CMake/Ninja/Python等)とRustを用意します。必要なCargo toolsとEspressif toolchainを
インストールし、各shellでexport環境を読み込みます。

```sh
cargo install ldproxy
cargo install espup
espup install
. "$HOME/export-esp.sh"
```

Linuxでは `espflash` を後でシリアルモニターに使う場合に `libudev` 開発パッケージも
必要です。OSごとの正確な追加パッケージは
[esp-idf-template prerequisites](https://github.com/esp-rs/esp-idf-template#prerequisites)
を参照してください。

## ビルド

プロジェクトディレクトリから実行します。

```sh
cargo build --locked
```

最初のbuildはESP-IDFソース、ツールチェーン、Python環境を自動取得するため時間と
ディスク領域を要します。`Cargo.lock` は依存解決を固定するため管理対象です。

## シリアルHello World

`src/main.rs` は標準出力へ次のメッセージを出し、プロセスを維持します。

```text
Hello from Rust on M5Stack Tab5 / ESP32-P4
```

ソース上の出力実装は用意されていますが、ビルド・書き込み・実機シリアル出力は未確認
です。実機へ書き込むには、所有者の明示的な許可が必要です。このプロジェクトのbuild/
`cargo run`だけではデバイスに転送しません。書き込みコマンドを実行しないでください。
