# システム構成

## 方針

既存実装を優先し、ファームウェアを独自開発しません。参照実装は
[airpocket-soundman/Tab5_thinclient](https://github.com/airpocket-soundman/Tab5_thinclient)
です。以下は upstream のREADME、`platformio.ini`、主要ソースを2026-10-12に調査した
内容です。接続状態や動作は実機で測定していません。

## ハードウェアとファームウェア

| 要素 | 役割 / 調査で確認した情報 |
| --- | --- |
| M5Stack Tab5 | ESP32-P4搭載。ファームウェアはArduino frameworkとPlatformIOを使用。 |
| 無線 | Tab5のWi-FiでSSH先へ接続。実際の接続可否・周波数帯は環境依存で未検証。 |
| Tab5 Keyboard (A164) | 公式資料でI2Cアドレス `0x6D`、Tab5接続のSDA=`G0` / SCL=`G1` / INT=`G50` を確認。upstreamは `M5Unit-KEYBOARD` と専用入力実装を含む。バス上の認識・キー動作は未検証。 |
| microSD | ローカルスクリプトやファイルの保存先として利用。upstreamはLittleFSも構成しているが、これはmicroSDとは別領域。 |
| USB | 開発ホストからの書き込み/シリアル診断経路。書き込みは許可が必要。 |
| Tab5 firmware | M5Unified、M5GFX、M5Unit-KEYBOARD、ArduinoJson、LibSSH-ESP32などをPlatformIOで構築。 |

公式ハードウェア資料:
[Tab5](https://docs.m5stack.com/en/core/Tab5)、
[Tab5 Keyboard SKU A164](https://docs.m5stack.com/en/products/sku/A164)、
[Keyboard Arduino tutorial](https://docs.m5stack.com/en/arduino/projects/tab5/tab5_keyboard)。

## ソフトウェア構成

```text
Tab5 Keyboard
    │ I2C (公式資料のアドレス: 0x6D)
    ▼
Keyboard input → key/layout mapping ─────────────┐
                                                  │
microSD ── local CLI / MicroPython runner         │
                                                  ▼
Wi-Fi ── LibSSH-ESP32 ── SSH PTY / command exec ─ terminal emulator
                                                  │
                                                  ▼
                                         M5Unified / M5GFX display
```

- **ビルド:** `platformio.ini` の環境名は `tab5`。PlatformIO platformは
  `pioarduino/platform-espressif32` のstable配布、Arduino framework、
  `esp32-p4-evboard`。設定はPSRAM、LittleFS、カスタムpartition table、USB CDC、
  SSH等のビルドフラグを含みます。
- **依存関係:** M5Unified、M5GFX、M5Unit-KEYBOARD、ArduinoJson 7系、
  LibSSH-ESP32 5.8系。指定バージョンと制約はupstream
  [`platformio.ini`](https://github.com/airpocket-soundman/Tab5_thinclient/blob/7790627d53490677b40d2220cb5141f687d4c9e9/platformio.ini)
  を参照してください。安定版platformの実体や間接依存の完全な再現性は別途確認が必要です。
- **MicroPython生成物:** upstreamには生成済み埋め込みruntimeが含まれています。再生成用
  scriptは別途external MicroPython checkoutと特定名のRISC-V compiler executableを参照
  するため、再生成は通常のPlatformIO buildとは分けて扱い、macOSで可能か確認が必要です。
- **Keyboard:** Tab5側で入力イベントを処理し、US/JP配列のASCII記号マッピングを行う
  `KeyboardMapper` が存在します。ソース上はShift/Ctrlの処理が確認できますが、実機の
  キー行列や全キー組み合わせの成功を示すものではありません。HID mapperでAlt modifier
  の処理は確認できません。USB/BLEキーボード対応もREADMEに記載されていますが、今回の
  検証対象ではありません。
- **ローカルCLI / MicroPython:** CLIはファームウェア内のコマンド実装です。MicroPython
  runtimeは組み込みで、ソースではVM用ヒープを256 KiBで要求し、利用可能ならPSRAMを
  優先しています。REPL、短いコード、microSD上のスクリプトを実行し、ファームウェアの
  `gfx` API経由でスプライト描画します。通常のLinuxやデスクトップ版Pythonではありません。
- **SSH:** `SshClient.cpp` はLibSSH-ESP32 APIを使い、PTY/shell、コマンド実行、
  SCP転送の処理を持ちます。接続ソースで確認できるユーザー認証はパスワード方式です。
  サーバーホスト鍵の検証処理は調査した接続コードから確認できません。
- **設定/保存:** upstream READMEはWi-Fi/SSH profileをUIから編集しフラッシュに保存する
  と説明しています。ソースではプロファイルJSONへWi-Fi/SSHパスワードを保存します。
  暗号化は確認できないため、フラッシュ上でも機密情報として扱います。`platformio.ini`
  はLittleFSと `data/` を指定します。認証情報をGit管理ファイルへ追加しないこと。

## macOSホスト側

Python仮想環境にPlatformIO Coreを導入し、別ディレクトリのupstreamソースをビルドする
構成です。コンパイルはmacOS上で行えますが、この調査環境はLinuxであり、macOS固有の
USBポート名、ドライバ、書き込み挙動は確認していません。upstreamが完全書き込み用に
記載する `flash_tab5.ps1` はPowerShellスクリプトで、macOS用の置き換え手順として
検証されていません。

## 境界と非目標

- Tab5本体はLinuxホストではありません。シェル風CLIは限られたコマンド実装です。
- SSH先はユーザーが許可したホストに限ります。この調査では接続していません。
- Terminal/ANSI実装の互換性、US/JP物理配列との一致、日本語IME/表示は実機確認が必要です。
- upstream READMEと `docs/ARCHITECTURE.md` でEsc操作の説明に食い違いがあります。実機で
  確認するまでEscの文書上の説明を仕様として扱いません。
- キーボードSTM32の更新、フラッシュ消去、データ領域変更はこの作業では行いません。
- `scripts/` にセットアップ補助コードは追加していません。upstreamの標準手順だけで文書化
  でき、独自スクリプトを加える根拠がないためです。
