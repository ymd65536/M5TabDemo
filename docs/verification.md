# 動作検証チェックリスト

## 判定基準

- **ソース/資料で確認:** README、PlatformIO設定、ソースコード、または公式資料に記載がある。
  実機動作の成功を意味しない。
- **未実施:** この環境では実機・macOS・外部SSHホストを使用していないため未確認。
- **要許可:** 実機の書き込み・消去・データ上書き、キーボードSTM32更新、外部SSH接続を行う前に
  所有者の許可を得る。

## 現時点の環境

| 項目 | 結果 |
| --- | --- |
| 作業リポジトリ | Rust ESP-IDF Hello World crateを追加。Arduino/PlatformIO upstream source、Tab5 BSP統合、testsはなし。 |
| ホスト | Linux x86_64環境。macOS Apple Siliconではない。 |
| Python | Python 3.12.3を確認。 |
| PlatformIO | `pio` コマンドは見つからず、未インストール。 |
| 実機 | Tab5 / Tab5 Keyboard / microSDは接続・操作していない。 |
| 許可対象操作 | ファームウェア書き込み、フラッシュ消去、既存データ上書き、STM32更新、SSH接続を未実施。 |
| Rust build | `cargo build --verbose`を実行。ESP-IDF source/toolchain準備中に`dl.espressif.com`の名前解決が失敗し、constraints fileを取得できず中断。build不成功。 |
| PlatformIO build | この実行環境では`pio`がなく、既存C++ upstream sourceもcheckoutしていないため未実施。 |
| test / HIL | 自動テストなし。Tab5実機での書き込み・動作試験なし。 |

## Phase 1 — 既存ファームウェア調査

| 確認事項 | 調査結果 |
| --- | --- |
| Tab5 / 純正キーボード対応 | upstream READMEと設定はTab5 / ESP32-P4 / Tab5 Keyboardを対象とする。Keyboardの公式I2Cアドレス `0x6D` は公式資料で確認。実機認識は未実施。 |
| ビルド環境 | PlatformIO、Arduino framework、`tab5`環境。依存ライブラリは `platformio.ini` に記載。 |
| SSH実装 | `LibSSH-ESP32` / libsshを使う実装とPTY shell、exec、SCPのコードを確認。パスワード認証関数を確認。ネットワーク実接続は未実施。 |
| ローカルCLI / Python | コマンド一覧、PythonRunner、組み込みMicroPython、`gfx`のAPI説明を確認。実機実行は未実施。 |
| READMEとソース | READMEの機能主張をコード構成と照合。キーボードマッパーではShift/Ctrlの一部処理を確認したが、HID mapperでAlt modifier処理は確認できない。SSH接続コードはパスワード認証を呼ぶ。upstream READMEと `docs/ARCHITECTURE.md` のEsc操作説明は一致しない。 |
| 制約 | README自身が実験的と説明。完全なPOSIX shellではなく、日本語入力/表示・全ANSI互換性・実機性能は未検証。接続ソースでホスト鍵検証も確認できないため、本人性が検証される前提で運用しない。 |
| ライセンス | upstream自作部分はMITとREADMEに記載。第三者依存物は別ライセンスで、SSH/libsshはLGPL-2.1-or-laterとの説明。再配布前にthird-party noticesを確認。 |
| 公式資料 | Tab5とSKU A164 keyboardの公式製品ページ、keyboard tutorialを参照。 |

## Phase 2 — macOS環境 / build

- [ ] macOS Apple Silicon上でPython仮想環境を作成する。
- [ ] 仮想環境内へPlatformIO Coreを導入する。
- [ ] upstreamの調査対象revisionを取得する。
- [ ] `pio run -e tab5` を実行し、終了コードと生成物を記録する。
- [ ] PlatformIOの依存取得失敗やビルドエラーがあればログを保存し、再現条件を記録する。
- [ ] MicroPython生成済みファイルで通常buildできるかを記録する。再生成が必要ならupstream generatorの外部checkout/compiler要件を別に検証する。
- [ ] PlatformIOのシリアルポート一覧とTab5接続時のデバイス名を確認する。
- [ ] **要許可:** 正しいデバイスと書き込み対象を確認後に限りuploadを試す。
- [ ] 初回書き込みやLittleFS/partitionを含む更新方法について、macOSで検証された手順を確立する。Windows用PowerShellスクリプトをmacOSで実行できるとみなさない。

この作業ではmacOSもupstream C++ソースのローカルコピーも利用できませんでした。Rust
prototype buildは試しましたが、download serverのDNS failureでESP-IDF build setupが停止しました。

## Phase 3 — Tab5 Keyboard

- [ ] **要許可・実機:** I2CスキャンでKeyboardの `0x6D` 応答を確認し、使用したバス/ポートも記録する。
- [ ] 文字入力、連続入力、キーリピート、同時押しを確認する。
- [ ] Shift、Ctrl、Alt、Tab、Escを個別および組み合わせで確認する。
- [ ] 上下左右キーが設定画面とSSHターミナルで期待する用途に割り当たるか確認する。
- [ ] 既定とJP設定を切り替えてUS/JP記号キーの出力を比較する。
- [ ] CLIでTab、Esc、カーソルキー、Ctrl-Cを試し、通常コマンド入力との挙動を分けて記録する。
- [ ] upstream READMEと `docs/ARCHITECTURE.md` で異なるEsc操作の記述を、画面ごとの実機動作と照合する。
- [ ] **要許可:** STM32 firmware更新を促されても、別途許可されるまで実施しない。

## Phase 4 — ローカルCLI / MicroPython

- [ ] **要許可・実機:** 起動後のCLI表示と `help` / `status` を確認する。
- [ ] microSDのマウント状態、ルート参照、ファイル一覧を確認する。
- [ ] `python` でREPLが起動するか確認する。
- [ ] `python -c print('hello')` の出力を確認する。
- [ ] microSD上の最小 `.py` スクリプトを起動する。
- [ ] `gfx` APIで描画し、`gfx.present()`後に表示されるか確認する。
- [ ] 上記を通常のLinux shell/Pythonとは異なる内蔵実装として記録する。

## Phase 5 — SSH

- [ ] **要許可:** 許可済みテストネットワークでWi-Fi接続を確認する。
- [ ] **要許可:** 許可済みLinuxホストへの疎通とパスワード認証を確認する。実際の資格情報をリポジトリ/ログへ保存しない。
- [ ] 対話シェル、基本コマンド、正常切断と再接続を確認する。
- [ ] ANSI表示、カーソル移動、画面消去、スクロールを確認する。
- [ ] `vim` / `nano` を利用する場合、入力モード切替・Esc・保存・終了を確認する。
- [ ] `Ctrl-C`、Tab、矢印、Ctrl+矢印の動作を確認する。
- [ ] 日本語入力、UTF-8受信、全角文字幅、フォント表示を個別に判定する。未対応/未確認の区別をつける。
- [ ] サーバーホスト鍵の検証方法が不明な場合は、信頼できるネットワーク以外での利用を避ける。

## Phase 6 — 記録と再現性

- [ ] upstream commit SHA、macOS/Python/PlatformIOバージョン、build結果を記録する。
- [ ] 実機試験はハードウェア構成、接続状態、試験入力、観測結果を記録する。
- [ ] 実機で成功した項目と、README/ソース上だけで確認した項目を混同しない。
- [ ] 失敗時はシリアルログから個人情報・Wi-Fi/SSH秘密情報を除いて記録する。
- [ ] フラッシュ消去や既存データ上書きは、別途明確に許可された場合に限る。
