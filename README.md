# AWS Cost Notifier

## 概要

AWS のコスト情報を取得し、Slack に通知する Rust アプリケーションです。

## 機能

- AWS Cost Explorer API を使用して現在の月のコスト情報を取得
- サービスごとのコスト内訳を計算
- 総コストとサービスごとのコスト内訳を Slack に通知

## 前提条件

- Rust (2021 Edition)
- AWS アカウントと適切な IAM 権限
- Slack ワークスペースと Incoming Webhook URL

## セットアップ

1. このリポジトリをクローンします：

   ```
   git clone https://github.com/yourusername/aws_cost_notifier_rs.git
   cd aws_cost_notifier_rs
   ```

2. `.env` ファイルを作成し、以下の環境変数を設定します：

   ```
   AWS_REGION=your-aws-region
   SLACK_WEBHOOK_URL=your-slack-webhook-url
   ```

3. 依存関係をインストールします：

   ```
   cargo build
   ```

## 使用方法

アプリケーションを実行するには、以下のコマンドを使用します：

```
cargo run
```

これにより、現在の月のAWSコスト情報が取得され、設定された Slack チャンネルに通知が送信されます。

## 設定

`config.rs` ファイルで、アプリケーションの動作をカスタマイズできます。主な設定オプションには以下が含まれます：

- AWS リージョン
- Slack 通知フォーマット

## TODO

- テストコード書く
