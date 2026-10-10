# pg_solver Launch Drafts (Hacker News & Zenn)

このドキュメントは、PostgreSQL外部オプティマイザ拡張機能「`pg_solver`」の公開告知用ドラフト原稿（Hacker News向け英文、およびZenn向け和文）の保管庫です。

---

## 1. Hacker News (Show HN) 投稿用原稿

### 【Title Options】
* Option A (Recommended):
  `Show HN: pg_solver – A Rust/pgrx extension that turns slow Postgres queries into 0.06ms via geometric solver`
* Option B:
  `Show HN: pg_solver – Autonomous PostgreSQL optimizer (23.8ms to 0.06ms) written in Rust`

### 【Body (Markdown)】

Hi HN,

Like many of you, I've spent countless late nights fighting with PostgreSQL’s native planner. You have a table with 1M rows, complex JOINs and WHERE clauses, and suddenly the planner picks a catastrophic sequential scan or an explosive nested loop. Tuning indices by hand or wrestling with `EXPLAIN ANALYZE` feels like brute-force guessing.

So I built **`pg_solver`**, a PostgreSQL 16 extension written in Rust (using `pgrx`) that optimizes queries by modeling constraints geometrically:
👉 **https://pg.microforce.dev/**

### 1. The Benchmark (1,000,000 Rows Test)
On a local benchmark with 1M rows involving multi-table joins and range filters:
* **Standard PG Planner:** Execution time **23.84 ms**
* **With pg_solver:** Execution time **0.06 ms (~400x speedup)**

### 2. Safety First: Terminal Simulation Before You Apply
As someone who manages production databases, I hate "black-box" magic tools that silently alter schemas or query plans. `pg_solver` is built with a dry-run-first philosophy:

```sql
-- Step 1: Run optimization simulation in your terminal
SELECT * FROM pg_solver_report('
    SELECT u.name, o.total 
    FROM users u 
    JOIN orders o ON u.id = o.user_id 
    WHERE o.created_at >= ''2026-01-01''
');
```

This outputs a detailed terminal simulation:
* Estimated execution time reduction (e.g., `23.84 ms -> 0.06 ms`)
* Algorithmic convergence stats (iterations, stress tensor)
* Exact index recommendation commands (e.g., `CREATE INDEX CONCURRENTLY ...`)

Only after reviewing and agreeing with the simulation in your terminal do you execute:
```sql
-- Step 2: Safely apply the recommended optimization
SELECT pg_solver_apply('query_id_or_sql');
```

### 3. How It Works (Technical Implementation)
Rather than relying on Postgres's default Genetic Query Optimizer (GEQO) or exhaustive combinatorial search, `pg_solver` maps query filter boundaries, join predicates, and selectivity space into a multi-dimensional constraint hypervolume. It then solves the execution path using **Projection Onto Convex Sets (POCS)** algorithms in Rust.
* Compiled with LTO and stripped symbols for minimal overhead (< 15 µs optimization phase).
* Directly integrates with Postgres via SPI hooks.

### 4. Try It Yourself
I’ve packed a **free 72-hour trial** (Linux x86_64, PostgreSQL 16) with no registration or credit card required. You can download it directly, drop it into your test/staging container, and see if it beats your slowest queries:
* Website / Docs: https://pg.microforce.dev/
* Direct Trial Download: https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip

### Technical Note / Discussion
I’d love to hear your thoughts on:
1. Edge cases where convex relaxation might produce suboptimal index sets for hyper-skewed distributions.
2. How you currently handle query plan drift in large Postgres clusters.

Feedback and brutal critiques are very welcome!

---

## 2. Zenn 投稿用原稿（和文ドラフト）

### 【タイトル案】
* `PostgreSQLで100万件のJOINが遅すぎて胃が痛かったので、Rustで自作オプティマイザ拡張を作って23msを0.06msにした話`

### 【本文構成案】

#### 導入
PostgreSQLでデータ規模が増えてきたとき、複合インデックスを貼ってもNested Loopが爆発したり、オプティマイザが意図しないSeq Scanを選択して頭を抱えた経験はないでしょうか。
`EXPLAIN ANALYZE`と格闘しながら手作業でインデックスを微調整する作業は、まさに深夜の当て物ゲームです。

そこで、クエリの制約条件を多次元幾何学空間に射影して高速に最適解を求めるPostgreSQL 16用アドオン **`pg_solver`** をRust (`pgrx`) で実装しました。

👉 **公式サイト**: https://pg.microforce.dev/ja.html

#### 100万行ベンチマークの実測結果
複数テーブルの結合（JOIN）と範囲条件を含む100万件テーブルでのテスト：
* 標準PostgreSQLプランナ: **23.84 ms**
* pg_solver最適化後: **0.06 ms（約400倍高速化）**

#### 安全設計：いきなり反映せず、まずターミナルでシミュレーション
本番DBを勝手に弄るツールほど怖いものはありません。`pg_solver` は「ドライラン第一主義」で設計しています。

```sql
-- 最適化の事前シミュレーション
SELECT * FROM pg_solver_report('対象クエリ');
```
これを実行すると、ターミナル上に以下が即座にレポートされます：
* 予測短縮時間（例: 23.84ms -> 0.06ms）
* ソルバーの収束回数と応力値
* 具体的に発行すべきインデックスDDL（`CREATE INDEX CONCURRENTLY ...`）

内容を目視で確認し、納得した上で反映関数を実行できます。
```sql
SELECT pg_solver_apply('対象クエリ');
```

#### 技術概要（どうやって解いているか）
Postgres標準の遺伝的アルゴリズム（GEQO）ではなく、制約条件充足問題を解く「凸集合射影法（POCS）」をRustでゼロから実装しています。
クエリのフィルタ境界と結合選択率を多次元幾何学の境界としてモデル化し、最適解へ瞬時に収束させます。

#### 無料試用版について
クレジットカード登録や会員登録なしで、ローカルや検証環境（Docker）で72時間試せるトライアルZIPを配備しています。
* サイト: https://pg.microforce.dev/ja.html
* 試用版DL: https://dl.microforce.dev/downloads/pg_solver_trial_linux_x86_64.zip
