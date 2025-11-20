# bevy-2dshooting-game

`bevy-2dshooting-game`は、ゲームエンジンBevyで作られた2Dのシューティングゲームです。

## ゲーム概要

スクロールしていく画面で敵を倒していき、いっぱい敵を倒して高スコアを目指すゲームです。

## ゲーム情報

ゲームタイトル `いっとく2Dシューティングゲーム`

画面サイズ `640x480`

## 遊び方

リポジトリをクローンしてから、`cargo run`を実行することで遊ぶことができます。

## 操作方法

- ゲームを始める: 左クリック
- AWSDキー: 移動
- スペース: 弾を発射


## Wasmに変換する

ゲームをWasmに変換する場合は、以下のコマンドを実行します。

```sh
# ビルド
cargo build --release --target wasm32-unknown-unknown
# 変換
wasm-bindgen --target web --out-dir ./examples --no-typescript \
target/wasm32-unknown-unknown/release/ittoku_2dshooting_game.wasm
```

## クレジット

開発者 [ittokunvim](https://github.com/ittokunvim)

ゲームエンジン [Bevy](https://bevyengine.org)

戦闘機画像 [Void - Main Ship](https://foozlecc.itch.io/void-main-ship)

敵機画像 [Void - Fleet Pack 1 - Kla'ed](https://foozlecc.itch.io/void-fleet-pack-1)

背景画像 [Void - Environment Pack](https://foozlecc.itch.io/void-environment-pack)

ハート画像 [hp heart](https://gft123456er.itch.io/hp-heart)

フォント [美咲フォント](https://littlelimit.net/misaki.htm)

効果音 [効果音ラボ](https://soundeffect-lab.info/)

画像編集 [Pixlr](https://pixlr.com)

Wasm変換 [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
