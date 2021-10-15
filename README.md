# Rust - Rocket Playground

## 構成環境

- Rust: 1.57.0-nightly
- rocket: 0.4.10

## ファイル構成

```sh
.
├── README.md
├── Cargo.toml
├── Cargo.lock
├── Rocket.toml
├── rust-toolchain
├── src
│   ├── main.rs
│   ├── routes.rs
│   ├── controllers.rs
│   └── models.rs
└── target/
```

## ローカル環境構築

### nightly 版の Rust をインストールする

Rocket は nightly 版の Rust である必要があるため, そのインストールを行う

```sh
# nightly 版をインストールする
rustup install nightly
```

### パッケージ・ライブラリのインストール

```sh
# パッケージのインストール
cargo install --path .
# ライブリロード用のライブラリをインストールする
cargo install cargo-watch
```

### rocket の起動

```sh
cargo watch -x run
# 動作環境を指定した起動方法
ROCKET_ENV=stage cargo watch -x run
```

- ROCKET_ENV は rocket の動作環境の設定
  - デフォルトでは dev
  - 動作確認の値: dev, stage, prod

### 動作確認

```sh
curl http://localhost:8000
```

## Test の実行

```sh
cargo test
```

## ドキュメント

https://api.rocket.rs/v0.4/rocket/
