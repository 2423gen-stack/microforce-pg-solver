# pg_solver (PostgreSQL Autonomous Query Optimizer)

[ English | [日本語](#日本語ドキュメント) ]

Autonomous PostgreSQL 16 extension powered by a multi-dimensional geometric constraint solver (POCS). It automatically detects slow sequential scans and query bottlenecks, synthesizing globally optimal index sets to minimize overall system latency with minimal index overhead.

* **Website & Docs:** [https://pg.microforce.dev/](https://pg.microforce.dev/)
* **Free 72h Trial Binary (Linux x86_64, PG16):** [Direct Download .zip](https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip) *(No signup, no credit card required)*
* **Pro Lifetime License ($99.00 USD):** [Buy Pro via Stripe Checkout](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01)

---

## Key Highlights

- **Plug & Play (Zero Complex Config)**:
  Drop `.so` and extension files into PostgreSQL directory, then run `CREATE EXTENSION pg_solver;`.
- **400x Latency Reduction (1M Rows Benchmark)**:
  - **Before (Sequential Scan)**: `23.867 ms`
  - **After (Index Scan)**: `0.060 ms` (**~99.7% latency cut / ~397x speedup**)
- **Dry-Run & Terminal Simulation**:
  Review latency improvements, algorithmic convergence, and recommended index SQL statements before modifying your production schema.
- **Zero-Crash Memory Safety**:
  Engineered in Rust with `pgrx` type safety. Guarantees zero undefined behavior or crash hazards to PostgreSQL core engine.
- **100% Local & Zero Egress**:
  No telemetry, no external API calls, completely self-contained. Fully compatible with air-gapped, financial, healthcare, and enterprise VPC environments.

---

## Quick Start (3 Steps)

### Step 1: Install Extension Files
Extract `pg_solver.zip` and copy files to your PostgreSQL library and shared directory:

```bash
# Copy shared library
sudo cp pg_solver.so $(pg_config --pkglibdir)/

# Copy control and SQL files
sudo cp pg_solver.control pg_solver--0.1.0.sql $(pg_config --sharedir)/extension/
```

### Step 2: Enable Extension
Connect to your PostgreSQL database and enable `pg_solver`:

```sql
CREATE EXTENSION pg_solver;
```

### Step 3: Simulate in Terminal & Safely Apply

```sql
-- 1. Dry-run: Generate optimization diagnostic report in terminal
SELECT * FROM pg_solver_report();

-- 2. Apply recommended index optimizations (executed via CONCURRENTLY, zero table locking)
SELECT pg_solver_apply();
```

---

## Editions

| Feature | 72-Hour Free Trial | Pro Lifetime License |
| :--- | :--- | :--- |
| **Duration** | **72 hours** from first execution | **Unlimited Lifetime** |
| **Optimization Simulation** | Included (with trial countdown) | Included (Professional tier) |
| **Index Application** | Included (`CONCURRENTLY`) | Included (`CONCURRENTLY`) |
| **Binary Protection** | LTO + stripped native binary | LTO + stripped native binary |
| **Price** | **[Free Download (.zip)](https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip)** | **$99.00 USD** ([Stripe Checkout](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01)) |

---
---

<a id="日本語ドキュメント"></a>
# 日本語ドキュメント

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
| **価格** | [無料ダウンロード（72h Trial .zip）](https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip) | $99（[Stripeで購入](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01)） |

👉 **[72時間無料お試し版（Linux x86_64 / PG16）を直接ダウンロード](https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip)**
*(登録不要・クレジットカード不要・解凍して配置するだけで即使えます)*

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
