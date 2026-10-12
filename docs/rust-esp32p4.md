# ESP32-P4 / Tab5 Rust feasibility

**調査基準日: 2026-10-12.** `main` / `master` は更新されるブランチです。下記リンクと
crate/versionのスナップショットを使用前に再確認してください。

## 結論

**RustでESP32-P4向けアプリを作ることは可能です。** 今回はESP-IDF上のRustを選びました。
Tab5の機能豊富なアプリに必要な公式Tab5 BSPとC6 hosted-radio経路をC/ESP-IDF側で利用
しながら、アプリケーションロジックだけRustにできます。RustでTab5の全機能をゼロから
再実装できるという意味ではなく、Cの公式BSPやコンポーネントとの統合作業は残ります。

## 公式ESP-RSサポート

### Targetと成熟度

Rust target tripleはSoCだけでなく実行モデルで異なります。混同しないでください。

| SoC / runtime | Target triple | 調査結果 |
| --- | --- | --- |
| ESP32-P4 / bare-metal `esp-hal` | `riscv32imafc-unknown-none-elf` | `no_std`; P4 revision 3.0以上。 |
| ESP32-C6 / bare-metal `esp-hal` | `riscv32imac-unknown-none-elf` | `no_std`; C6自身のネイティブWi-Fiにはesp-radio等が必要。 |
| ESP32-P4 / ESP-IDF | `riscv32imafc-esp-espidf` | ESP-IDF上のRust/stdアプリのtarget。 |
| ESP32-C6 / ESP-IDF | `riscv32imac-esp-espidf` | C6 co-processorファームウェア用target。 |

出典:

- [`esp-hal` supported chips/targets](https://github.com/esp-rs/esp-hal/blob/esp-hal-v1.2.0/esp-hal/README.md)
  はP4 v3.0+を示します。v1.2.0 changelogはP4の初期サポートを追加として示し、
  GPIO、UART、I2C、SPI、USB Serial/JTAG、SYSTIMER等が順次追加されたことを記録します。
  個々の周辺機能は同READMEのsupport matrixで確認してください。特にP4 Wi-Fiは
  esp-halの内蔵無線機能ではありません。
- [`esp-idf-sys` target table](https://github.com/esp-rs/esp-idf/tree/master/esp-idf-sys)
  と[`esp-idf-svc`](https://github.com/esp-rs/esp-idf/tree/master/esp-idf-svc) READMEは、
  P4のESP-IDF Rust targetを上記の`riscv32imafc-esp-espidf`として掲示します。
- 旧`esp-rs/esp-idf-sys`および`esp-idf-svc` repositoriesはread-onlyの移転告知を掲載し、
  現在の開発先は[`esp-rs/esp-idf`](https://github.com/esp-rs/esp-idf) monorepoです。
  esp-idf cratesはcommunity effortで、ESP-IDF自体に比べHILテスト・文書・更新追従が
  限られる旨をupstreamが明記しています。

### ツールチェーン

Rust版ESP-IDFの公式templateは、RISC-V向けにnightly Rustと`rust-src`を使い、
`build-std`でtarget用stdを構築します。build環境ではldproxy、ESP-IDF C toolchain、
Python/CMake/Ninja等が必要です。ESP-IDF templateが提示する一般的導入:

```sh
cargo install cargo-generate ldproxy espup espflash
espup install
. "$HOME/export-esp.sh"
```

このプロトタイプはtemplateのRust configに従い`riscv32imafc-esp-espidf`と`ESP32P4`
を選択します。ESP-IDF v5.5.5を明示しています。これは当日のESP-IDF templateが stable
optionとして掲載し、Tab5 BSPが要求する`>=5.4`を満たす組合せです。v5.5.5との実機
動作やmacOS Apple Silicon上のbuildはまだ確認されていません。

参考: [official esp-idf-template](https://github.com/esp-rs/esp-idf-template),
[`cargo` template config](https://github.com/esp-rs/esp-idf-template/tree/master/cargo)。

## Tab5 BSPと周辺機器

Espressif公式[`esp-bsp`](https://github.com/espressif/esp-bsp)には
[`bsp/m5stack_tab5`](https://github.com/espressif/esp-bsp/tree/master/bsp/m5stack_tab5)
があります。調査時manifest versionは1.3.2、targetは`esp32p4`、最低ESP-IDFは5.4です。
公式BSPはC/ESP-IDF実装で、display/touch、audio、SD card、camera、IMU等の初期化APIと
依存コンポーネントをまとめます。**Rust BSPではありません。**

Rustアプリからこれらを使う現実的な方針:

1. P4アプリと基本処理をRust (`esp-idf-svc`) で実装。
2. ESP-BSPをESP-IDF componentとしてbuildへ組み込む。
3. 使うC APIだけ薄いC shim / FFIでRustへ公開。
4. panel、backlight、expander、MIPI-DSI初期化を再実装せず、BSP側の初期化を使う。

ESP-IDF Rust templateの標準cargo構成にBSP componentが自動で入るわけではなく、component
依存・CMake build・bindingsの統合作業を別に設計する必要があります。BSP単体の機能表は
各SoC peripheralをRustから直接扱えることを保証しません。

## Tab5 Keyboard I2C

Tab5 Keyboard A164はホストP4へI2Cで接続するアクセサリです。今回の調査対象には
keyboardプロトコル向けRust driverやTab5 BSP内keyboard APIは確認できませんでした。
ESP-IDF `esp-idf-hal`のI2C master、または`esp-hal` P4 I2C masterでbus/device操作は
実装可能ですが、キースキャン・modifier等のプロトコルも別に必要です。公式Arduino
`M5Unit-KEYBOARD`等既存ライブラリが対象モジュールで利用可能ならC/C++側をshim経由で
再利用する方法も比較対象です。I2Cアドレス、ピン、poll/interrupt動作は製品資料と
接続revisionを確認してから実装してください。実機アクセス・ACK・文字入力は未実施。

## Display文字描画

ESP-IDF Rustから標準出力へ出すHello Worldと、Tab5 LCDへ描くHello Worldは別問題です。
Tab5は高解像度MIPI-DSIパネルです。ESP-IDF BSPがLCD controller/panel、power-enable、
LVGL/display initializationを提供するため、そのAPIをC FFIから呼ぶのが第一候補です。
`esp-hal`の基本GPIO/SPI supportだけでMIPI-DSI/panel initializationが済むわけでは
ありません。BSPとRustを結合したbuild/描画テストは未実施です。

## ESP32-C6経由Wi-Fi

Tab5はESP32-P4をmain SoC、ESP32-C6-MINI-1Uをwireless moduleとするdual-chip構成です。
P4にはWi-Fi radioが内蔵されていません。

Espressifの[`esp-hosted-mcu`](https://github.com/espressif/esp-hosted-mcu)公式READMEは、
**ESP32-P4 host + ESP32-C6 co-processor**の組合せを実例として示し、SDIO/SPI/UARTの
transport optionsを掲載します。仕組みはC6側co-processor firmwareとP4側host stackの
2つを用意する構成です。ESP-IDF側の[`esp_wifi_remote`](https://github.com/espressif/esp-wifi-remote)
はremote coprocessor経由の`esp_wifi` APIを提供し、`esp-idf-hal`のP4 modem supportも
このcomponentが有効な場合に条件付きになります。これらはRust固有のWi-Fi実装では
ありませんが、ESP-IDF Rust wrapperを利用できる可能性があります。

重要な未確定点: Tab5 board BSPのREADMEはC6無線モジュールを確認しますが、調査したBSP
資料だけでは**実機基板で使われているP4-C6 transport、pins、C6 firmware version /
configurationが特定できません**。ESP-Hostedの一般例をそのままTab5へ適用できるとは
扱わず、schematic、M5Stack/ESP-BSP board config、実際のIDF project設定から照合して
ください。BSPにHosted Wi-Fiが自動設定されるとも仮定しません。P4/C6両側の書き込み/
更新は実機許可を得るまで行いません。

## SSH feasibility

SSHはP4でTCP/IP/Wi-Fiが利用可能になった後に実装するアプリ層です。ESP-IDF Rustの
`esp-idf-svc`でネットワークサービスを利用し、Rust SSH client crateを評価するか、
既存libsshをC FFIで再利用する二つの選択肢があります。

- **Rust-native:** Rust内で型安全なprotocol/state handlingを持てる。P4 targetとの
  crate/crypto/backend compatibility、RAM/PSRAM、stack use、暗号suite、host-key checking、
  interactive PTY/terminal supportは候補crateごとに検証が必要。
- **C libssh reuse:** 既存実装/IDF C dependenciesを活用できるが、build configuration、
  ABI/unsafe境界、C library memory useとライセンス義務が残る。`Tab5_thinclient`の
  Arduino libssh integrationがESP-IDF Rustへそのままリンクできるとは限らない。
- **製品品質:** host keyを明示検証し、credentialをGit管理外で安全に扱える設計を前提に
  評価する。Rust化だけで暗号/認証安全性が確保されるわけではない。

**判定:** 技術的には実現可能だが未実装・未ビルド・未接続。まずESP-IDF/C6 transportを
確立し、次にC BSP経由の画面とI2C keyboardを個別にbring-upしてからSSHを選定する。

## プロトタイプと実施結果

[`rust/tab5_p4_hello`](../rust/tab5_p4_hello/README.md) は単独のCargo projectです。
ESP-IDF templateと同じ構成でP4 target、ESP-IDF v5.5.5、`esp-idf-svc` 0.53.0を設定し、
serial stdoutへHello Worldを出すRustバイナリです。

| Step | 結果 |
| --- | --- |
| 1. Rust project生成 | `cargo new`で独立binary crateを作成。upstream ESP-IDF template構成を使ってESP32-P4向け設定を追加。 |
| 2. P4向けbuild | `cargo build --verbose`を実行。nightly/Rust dependenciesとESP-IDF source取得までは進み、Espressif download serverのDNS失敗で`espidf.constraints.v5.5.txt`取得に失敗。**build成功ではない。** |
| 3. Serial Hello World | 出力コードを作成。P4へのflashを実施していないため実出力未確認。 |
| 4. Keyboard I2C | driver/protocol実機検証なし。詳細は上記節。 |
| 5. LCD text | display BSP/FFI統合も実機描画も未実施。 |
| 6. C6 Wi-Fi | 公式Hosted経路を調査。Tab5具体的transport configは未特定で未接続。 |
| 7. SSH | 候補経路を評価。client未実装・未接続。 |

実機へのfirmware書き込み、フラッシュ消去、既存データ上書き、Keyboard STM32更新、
C6 co-processor firmwareの変更、外部SSH server接続はいずれも実施していません。

## 選択指針

| 作業領域 | 推奨 |
| --- | --- |
| CLI/parser、アプリ状態、pure logic | Rust化する価値がある。target testsも容易。 |
| P4 bare-metal peripheral experiment | `esp-hal`で可能。P4 v3+とdriver support matrixを先に見る。 |
| Tab5 MIPI-DSI panel, expander, LVGL/audio/camera init | 公式C ESP-BSP/ESP-IDF componentを維持し、必要最小限FFI。 |
| A164 keyboard protocol | まず公式/library protocolとbus wiringを確認。Rust direct driverか既存C++ libの薄いwrapperを比較する。 |
| C6 Wi-Fi hosted | 公式ESP-Hosted / esp_wifi_remote C/IDF componentとC6 firmwareを利用し、Rustは上位application/APIから利用。 |
| SSH client | Wi-Fi動作後にRust-native crate対C libsshをメモリ、PTY、host-key、licenseで比較。 |
