# mitsuami-unit-rust

ミツアミ（Mitsuami）は、出自の違う3つのAIが合議して答える、球体の話しかけガジェットです。
名前は「三つ編み」から取りました。3本を編んで1本にするように、3つの意見をそれぞれの形のまま編み合わせて、1つの答えにします。

このリポジトリは、3つのユニットのうち「本能」です。司令塔から質問を受け取り、賛否と理由を返します。

## 現在の状態

axum の雛形だけがあります。動くのはヘルスチェック（`GET /healthz`）のみで、質問に答える窓口（`POST /judge`）と推論エンジンはこれから作ります。

## 設計

3つのユニットは、言語、推論エンジン、Webフレームワークをすべて別にしています。司令塔が知っているのは各ユニットのURLと名前だけで、共通の窓口（`POST /judge`）を通してやり取りします。

| ユニット | 言語 | 推論エンジン | モデル形式 | Webフレームワーク |
|---|---|---|---|---|
| 理性 | Python | PyTorch（transformers） | safetensors | FastAPI |
| 感情 | Go | llama.cpp | GGUF | 標準の net/http |
| 本能 | Rust | ort（ONNX Runtime） | ONNX | axum |

モデルは各ユニットが自分で抱えます。モデルのファイルは、このリポジトリには入れません。

## 技術

- Rust 1.100（edition 2024）
- axum / tokio

## 起動

Docker が必要です。

```sh
docker build -t mitsuami-unit-rust .
docker run --rm -p 8080:8080 mitsuami-unit-rust
curl http://localhost:8080/healthz
```

待ち受けるポートは、環境変数 `PORT` で変えられます（既定は 8080）。

## CI

Drone CI を使います。main への push と PR のたびに、イメージのビルドが通ることを確認します。

## 関連リポジトリ

| リポジトリ | 役割 |
|---|---|
| [mitsuami-backend](https://github.com/terumitt-dev/mitsuami-backend) | 司令塔。3つのユニットに質問を送り、多数決で結論を返す（Rails） |
| [mitsuami-frontend](https://github.com/terumitt-dev/mitsuami-frontend) | 画面。質問を送り、賛否と結論を表示する（React） |
| [mitsuami-unit-python](https://github.com/terumitt-dev/mitsuami-unit-python) | ユニット「理性」（Python、FastAPI、PyTorch） |
| [mitsuami-unit-go](https://github.com/terumitt-dev/mitsuami-unit-go) | ユニット「感情」（Go、net/http、llama.cpp） |
| [mitsuami-unit-rust](https://github.com/terumitt-dev/mitsuami-unit-rust) | ユニット「本能」（Rust、axum、ONNX Runtime） |
| mitsuami-personas（非公開） | 人格の定義、学習データ、追加学習と変換のスクリプト |
