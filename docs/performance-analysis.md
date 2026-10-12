# Tab5_thinclient 性能計測

## 調査結果

このリポジトリには upstream のファームウェアソースと `platformio.ini` がなく、
`Tab5_thinclient` は `.gitignore` で除外されています。そのため、この環境で
PlatformIO ビルド、実機計測、フラッシュ書き込みは実施できません。

upstream revision `7790627` のソースを確認した範囲では、`loop()` は入力、Wi-Fi、
SSH、描画を単一タスクで順番に処理します。`draw()` は `M5Canvas` 全画面
（表示サイズ）の Sprite を作成し、毎回 `pushSprite(0, 0)` で転送します。
`drawTerminal()` はローカル端末では viewport の各行を走査します。一方、
`TerminalEmulator` にはセル単位の dirty フラグがありますが、現在の描画経路が
それを利用することは、計測前に最適化の効果を断定しないため変更していません。
Tab5 キーボードの `update()` は M5Unit の更新とイベント変換を同じループで行い、
I2C 更新時間は実機で測定する必要があります。

## 計測パッチ

[`patches/0001-performance-instrumentation.patch`](../patches/0001-performance-instrumentation.patch)
は、upstream の同じ revision に適用する opt-in 計測実装です。

```sh
cd /path/to/Tab5_thinclient
git apply /path/to/M5TabDemo/patches/0001-performance-instrumentation.patch
pio run -e tab5 -D build_flags=-DTAB5_PERF_ENABLE=1
```

無効時は計測マクロが no-op です。有効時は `esp_timer_get_time()` で次を測定します。

- `main_loop`
- `keyboard_update` / `keyboard_update_internal`
- `key_action`
- `terminal_draw` と `sprite_push`
- `wifi_poll` と `ssh_poll`

5 秒ごとにシリアルへ、累積実行回数・平均・最大・直近128サンプルの p95 を出力します。
毎フレームのログは出しません。`key_action` はキーに起因するアクション全体の指標であり、
個別コマンドの内部時間を意味しません。コマンド実行、Wi-Fi 接続、SSH 接続ジョブの
個別計測は、この初回計測結果を取得してから追加します。

## 実機検証手順

1. upstream revision を固定し、パッチ適用後に `pio run -e tab5` を実行する。
2. `TAB5_PERF_ENABLE=1` で再ビルドする。書き込みは所有者の承認後にだけ行う。
3. USB シリアルを115200 baudで開き、起動後の5秒周期 `[tab5] perf` 行を保存する。
4. 無操作、連続キー入力、ローカルCLI、SSHで同じ時間だけ計測し、各 metric の
   `count/avg_us/max_us/p95_us` を比較する。
5. 画面表示、ANSI/VT、カーソル、日本語、SSHの安定性を確認し、変更前後のログと
   メモリ使用量を記録する。

この段階では dirty rectangle、部分 `pushSprite`、マルチタスク化は行っていません。
実機の p95 と最大値を根拠に、次の変更範囲を決定します。
