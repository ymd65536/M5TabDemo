# M5Stack Tab5 Portable Terminal

このリポジトリは、M5Stack Tab5 と純正 Tab5 Keyboard (SKU A164) を使った
ポータブルSSH端末の調査結果とセットアップ手順をまとめます。ファームウェアは
新規開発せず、既存OSSの
[Tab5_thinclient](https://github.com/airpocket-soundman/Tab5_thinclient) を利用します。

## 調査結果と状態

2026-10-12 に確認した upstream の `Tab5_thinclient` は、Tab5 / ESP32-P4 用の
PlatformIO + Arduino プロジェクトです。純正 Tab5 Keyboard、Wi-Fi、SSH、
ローカルCLI、組み込みMicroPythonの各実装と手順が存在するため、独自ファームウェアを
作るより同プロジェクトを使う方針です。参照した upstream revision:
[`7790627`](https://github.com/airpocket-soundman/Tab5_thinclient/tree/7790627d53490677b40d2220cb5141f687d4c9e9)。

このリポジトリ自体にはファームウェアソースや `platformio.ini` は含まれていません。
以下の手順は upstream を別ディレクトリに取得して実行します。現在の作業環境は
macOSでもTab5実機接続環境でもないため、ビルド・書き込み・キーボード・Wi-Fi/SSHの
実機確認は未実施です。コマンドは手順として記載していますが、実機への書き込みや
外部SSH接続をこの作業で実行していません。

## ハードウェアと公式資料

- M5Stack Tab5: ESP32-P4 搭載。製品仕様は
  [M5Stack Tab5 docs](https://docs.m5stack.com/en/core/Tab5) を参照。
- Tab5 Keyboard: SKU A164。I2C接続の仕様、ピン配置、対応キーは
  [製品ドキュメント](https://docs.m5stack.com/en/products/sku/A164) と
  [Arduinoチュートリアル](https://docs.m5stack.com/en/arduino/projects/tab5/tab5_keyboard)
  を参照。公式資料に記載のI2Cアドレスは `0x6D`。実機での認識は未確認。
- USBケーブルは開発ホストからの書き込み・シリアル診断に使います。
- microSDはローカルスクリプト、ファイル参照、デモに利用します。
- Tab5の無線接続には内蔵Wi-Fi機能を利用します。Tab5が接続可能なWi-Fi環境と
  到達可能なSSHサーバーが別途必要です。

## macOS 開発環境とビルド

1. upstream のソースを作業用ディレクトリに取得します。認証情報をソースやGit管理
   ファイルへ書き込まないでください。

   ```sh
   git clone https://github.com/airpocket-soundman/Tab5_thinclient.git
   cd Tab5_thinclient
   ```

2. Python仮想環境を作り、PlatformIO Coreをその環境にインストールします。

   ```sh
   python3 -m venv .venv
   source .venv/bin/activate
   python -m pip install --upgrade pip
   python -m pip install platformio
   ```

3. ファームウェアをビルドします。

   ```sh
   pio run -e tab5
   ```

`platformio.ini` が指定する主な構成は、`pioarduino/platform-espressif32` の stable
platform、Arduino framework、`esp32-p4-evboard`、LittleFS、独自 `partitions.csv`、
M5Unified、M5GFX、M5Unit-KEYBOARD、ArduinoJson、LibSSH-ESP32です。依存ライブラリは
PlatformIOが取得します。upstream READMEにあるビルドコマンドは `pio run -e tab5` です。
upstreamには生成済みMicroPython埋め込みコードも含まれます。これを再生成する場合の
手順は通常ビルドと別で、upstream generatorは外部MicroPython checkoutと特定名の
RISC-V compiler executableを参照します。macOSで再生成できるかは未検証です。

### ファームウェア書き込みについて

upstream READMEにあるPlatformIOの書き込みコマンドは次のとおりです。

```sh
pio run -e tab5 -t upload
```

**これは実機のファームウェアを書き換えます。このガイド作成時には実行していません。**
実行前に対象ポート、バックアップ、書き込み対象を確認し、所有者の明示的な許可を得て
ください。macOSでのシリアルポート選択とこの機器での書き込みは未検証です。

upstreamにあるbootloader・partition table・firmware・LittleFSをまとめて扱う
`flash_tab5.ps1` はPowerShell用です。macOSでの同等な完全書き込み手順はupstream READMEで
確認できませんでした。初回書き込みやLittleFSプロファイル領域を含めた更新が必要な場合、
確認なしに別の消去/書き込みコマンドを試さず、upstreamの最新手順と対象デバイスを確認
してください。

## 基本操作

起動後、画面上の `WIFI` でWi-Fiプロファイルを設定し、`SSH` で接続先を設定して
`CONNECT` します。CLIからは次のようなコマンドがREADMEに記載されています。

```text
help
status
wifi status
ssh list
ssh connect 0
```

接続先や認証情報はTab5上の設定画面で設定できます。upstreamの実装ではプロファイルを
LittleFS上のJSONへ保存し、Wi-Fi/SSHパスワードもシリアライズします。保存データが暗号化
されるとは確認できていないため、機密情報として扱ってください。秘密情報をこのリポジトリ、
プロファイル例、スクリーンショット、共有ログへ含めないでください。ソース内およびGit
管理対象ファイルには実際のWi-FiパスワードやSSH認証情報を保存しないでください。

組み込みCLIはLinux風のコマンドインターフェースであり、Tab5上でLinuxが動作するわけ
ではありません。upstream READMEは、パイプ、リダイレクト、シェル展開、バックグラウンド
ジョブに対応しないと明記しています。コマンド一覧は
[upstreamの一覧](https://github.com/airpocket-soundman/Tab5_thinclient/blob/7790627d53490677b40d2220cb5141f687d4c9e9/docs/COMMANDS.ja.md)
を参照してください。

## SSH

upstreamは `LibSSH-ESP32` / libssh を使い、PTY付き対話シェル、コマンド実行、SCP風転送
を実装しています。ソースの `SshClient.cpp` で確認できる認証方式はパスワード認証です。
公開鍵認証を使えるとは確認できませんでした。接続コードでサーバーホスト鍵の検証も確認
できないため、サーバーの本人性を検証すると仮定せず、信頼できるネットワークとサーバーで
のみ試験してください。

READMEはANSI/VT系の画面処理と `vim` / `nano` などを掲げていますが、互換性や操作性は
実機で確認されていません。ファームウェア自身も実験的と説明しています。日本語入力、
IME、全角文字の描画は確認できておらず、US/JP配列設定が日本語IMEやUnicode表示に対応
することを意味しません。実際のSSH接続・認証・対話操作は、所有者の許可を得た後に
検証してください。

READMEのEsc操作説明は、upstream `docs/ARCHITECTURE.md` のEscに関する説明と一致しません。
キーマッパーのソースではEscがメニュー操作として扱われる経路もあります。どの画面/入力経路
でどの動作になるかは実機で確認してください。

## MicroPythonとmicroSD

MicroPythonはファームウェアに組み込まれた実行環境です。通常のMicroPythonファームウェア
への置換やLinux起動を必要としません。upstream READMEと `docs/PYTHON.md` には以下の
ローカル機能が記載されています。

```text
python
python -c print('hello')
python /life.py
```

`python` はREPL、`python -c` は式/文の実行、`python <path>` はmicroSD上のスクリプト
実行用です。`gfx` オブジェクトを通じてM5GFXスプライトへの描画と表示ができます。
利用可能なAPI、引数、GPIO制限については
[upstream Python docs](https://github.com/airpocket-soundman/Tab5_thinclient/blob/7790627d53490677b40d2220cb5141f687d4c9e9/docs/PYTHON.md)
を参照してください。実機上のスクリプト実行・描画は未確認です。

## ライセンス

upstreamのプロジェクト所有コードはMIT Licenseです。依存物には個別ライセンスがあり、
READMEは特にSSH/SCP依存のLibSSH-ESP32/libsshについてLGPL-2.1-or-laterを記載して
います。upstreamの `THIRD_PARTY_LICENSES.md` を確認し、ソースを再配布する場合は
各ライセンス条件を守ってください。このリポジトリの `LICENSE` は本リポジトリの文書等に
適用され、upstreamや第三者コードのライセンスを置き換えません。

## 検証状況と制約

調査で確認できた実装と、未実施の実機検証を
[docs/verification.md](docs/verification.md) に分けて記載しています。システム構成は
[docs/architecture.md](docs/architecture.md) を参照してください。

現状は文書のみのリポジトリであり、upstreamソースのビルド結果はありません。この実行環境
では `pio` が利用できず、macOS、Tab5、Tab5 Keyboard、microSD、SSHサーバーも操作して
いません。実機の書き込み・消去・既存データ変更・キーボード側STM32更新・外部SSH接続は
行っていません。
