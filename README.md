# pg_solver (PostgreSQL Autonomous Query Optimizer)

PostgreSQLの遅延クエリと全表走査（Seq Scan）を自動検知し、多次元幾何学最適化（POCS）によって**最小のインデックス数でシステム全体のレイテンシを極小化する**自律型PostgreSQLアドオン（Extension）。

---

## 1. 特徴・セールスポイント
- **ゼロ設定・ポン付け（Plug & Play）**:
  バイナリ（`.so`）と定義ファイルを置くだけで `CREATE EXTENSION` 1発で即時動作。
- **圧倒的な実証性能（100万行実機ベンチマーク）**:
  - 適用前（Seq Scan）: `23.867 ms`
  - 適用後（Index Scan）: `0.060 ms`（**約99.7%高速化 / レスポンス 1/390**）
- **Before / After 視覚化レポート**:
  `SELECT pg_solver_report();` を叩くだけで、現場エンジニアが上司に稟議を通すためのエビデンス（推定短縮率・発行SQL）を一目瞭然に出力。
- **ゼロクラッシュ・メモリ安全性**:
  Rust ＋ `pgrx` による型安全設計。アドオンの不具合でPostgreSQL本体を巻き込んで落ちるリスクを構造的に排除。
- **完全ローカル完結（Zero Egress）**:
  外部への通信は一切なし。金融・医療・オンプレ・閉域VPC環境にも完全適合。
- **知財防護（Blackbox Engine）**:
  幾何学炉心と72時間時限ロジックは LTO + シンボル完全除去（strip）により逆コンパイル不能に保護。

---

## 2. インストール手順（わずか3ステップ）

### ステップ1: ファイルの配置
ダウンロードした `pg_solver.zip` を解凍し、PostgreSQLの所定ディレクトリへコピーします。

```bash
# 共有ライブラリの配置
sudo cp pg_solver.so $(pg_config --pkglibdir)/

# コントロールファイルとSQL定義の配置
sudo cp pg_solver.control pg_solver--0.1.0.sql $(pg_config --sharedir)/extension/
```

### ステップ2: 拡張の有効化
PostgreSQLに接続し、拡張を有効化します。

```sql
CREATE EXTENSION pg_solver;
```

### ステップ3: レポート確認 ＆ 最適化適用
```sql
-- 1. 診断・改善差分レポートを出力
SELECT pg_solver_report();

-- 2. 推奨インデックスを一括適用（CONCURRENTLYで無停止実行）
SELECT pg_solver_apply();
```

---

## 3. エディション比較

| 項目 | お試し版（Trial） | 製品版（Pro） |
| :--- | :--- | :--- |
| **利用期限** | **72時間**（自己消滅・ノーオペレーション化） | **無制限・永久ライセンス** |
| **最適化レポート** | 利用可能（残時間カウントダウン表示） | 利用可能（Professional表示） |
| **最適化適用** | 利用可能 | 利用可能 |
| **知財防護** | LTO + Strip済みブラックボックスバイナリ | LTO + Strip済みブラックボックスバイナリ |
| **価格** | 無料ダウンロード | $99（[Stripeで購入](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01)） |

---

## 4. 購入・ライセンス
製品版（Pro）の永久ライセンスは、以下のStripe公式決済リンクより即時購入可能です：

👉 **[Buy pg_solver Pro ($99.00 USD) - Stripe Checkout](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01)**
- 世界各国の税務コンプライアンス（MoR）対応
- クレジットカード / Apple Pay / Google Pay 対応
- 購入完了後、製品版バイナリのダウンロードリンクが即時発行されます。

- **Q: 本番データベースが停止することはありますか？**
  - A: ありません。推奨インデックスはすべて `CREATE INDEX CONCURRENTLY` で発行されるため、テーブルの読み書きロックを発生させずに安全に作成されます。
- **Q: 72時間経過したらデータはどうなりますか？**
  - A: データベースや作成済みインデックスには一切影響を与えません。アドオンの新規レポート出力・適用機能のみが安全に自己停止（NOTICE案内）します。
- **Q: アンインストールしたい場合は？**
  - A: `DROP EXTENSION pg_solver;` を実行後、配置したファイルを削除するだけでゴミを残さず綺麗に撤去できます。
