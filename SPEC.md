# pg_solver Architecture & Technical Specification

[ English | [日本語仕様書](#日本語仕様書) ]

Technical specification and architectural design for `pg_solver` (PostgreSQL Autonomous Query Optimizer & Slow Query Killer Extension).

---

## 1. System Overview
- **Product Name:** `pg_solver` (PostgreSQL Autonomous Query Optimizer Extension)
- **Target Audience:** Database Administrators (DBAs), Infrastructure Engineers, SaaS Backend Developers
- **Core Philosophy:** 
  - Solve query-workload and index-configuration combinatorial optimization problems globally using a multi-dimensional geometric constraint solver (Projection Onto Convex Sets: POCS).
  - Standalone native binary with zero network egress and zero external runtime dependencies.

---

## 2. Packaging & Delivery Model
1. **Plug & Play (Zero-Config Architecture):**
   - Copy `pg_solver.so`, `pg_solver.control`, and `pg_solver--0.1.0.sql` into standard PostgreSQL library and extension directories.
   - Activate instantaneously with `CREATE EXTENSION pg_solver;`.
2. **Two Distribution Editions:**
   - **72-Hour Free Trial:**
     - Runs for 72 hours from the user's first execution, then safely self-terminates into a non-operational state without affecting database integrity or existing indices.
   - **Full Pro Edition:**
     - Permanent, unrestricted lifetime license binary.

---

## 3. Technology Stack & Core Architecture
1. **Extension Core:** **Rust + `pgrx`**
   - Memory-safe architecture eliminating segfaults and null-pointer hazards from the PostgreSQL backend process.
   - Functions exposed safely via `#[pg_extern]` (`pg_solver_report()`, `pg_solver_apply()`).
2. **Solver Engine:** **Rust (POCS Geometric Optimization Core)**
   - C-level native execution speed with sub-millisecond convergence overhead (<15 µs).
3. **Database Integration:** **SPI (Server Programming Interface)**
   - Direct zero-copy access to internal execution statistics (`pg_stat_statements`) and query catalog metadata in the PostgreSQL process memory space.
4. **IP Protection & Binary Obfuscation:**
   - Compiled with Link-Time Optimization (`lto = true`) and fully stripped symbols (`strip = "symbols"`).
   - Proprietary constraint projection algorithms and trial countdown logic are irreversibly protected against reverse engineering.
5. **Zero Egress Security:**
   - 100% self-contained execution without outbound network requests or telemetry. Complies with air-gapped enterprise, financial, healthcare, and private VPC security standards.

---

## 4. Key Functional Requirements
1. **Non-Invasive Workload Sampling:**
   - Automatically detects sequential scan hotspots and problematic join operations without adding overhead to operational query execution.
2. **Global Combinatorial Optimization:**
   - Prevents index bloat and write degradation by discovering the minimal index set that achieves maximum global latency reduction.
3. **Dry-Run & Diagnostic Reporting:**
   - `SELECT * FROM pg_solver_report();` outputs terminal simulation reports detailing latency drops, solver convergence stats, and recommended `CREATE INDEX CONCURRENTLY` DDL statements.
4. **Safe One-Touch Application:**
   - `SELECT pg_solver_apply();` applies recommended optimizations non-blockingly via concurrent index builds.
5. **Self-Termination at Trial Expiry:**
   - When the 72-hour trial concludes, subsequent optimizer calls safely notify the operator via PostgreSQL `NOTICE` without disrupting normal database operations.

---

## 5. Benchmark Evidence (1,000,000 Rows Test)
- **Table:** `orders` (1,000,000 records)
- **Query:** `SELECT * FROM orders WHERE user_id = 42 AND status = 'pending';`
- **Before Optimization (Sequential Scan):** `23.867 ms`
- **After Optimization (Index Scan):** `0.060 ms`
- **Speedup:** **~99.7% Latency Reduction (~397x Faster)**

---

## 6. Commercial Model
- **Pricing:** **$99.00 USD (One-time payment, lifetime license)**
- **Merchant of Record:** Stripe Managed Payments ([Stripe Checkout](https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01))

---
---

<a id="日本語仕様書"></a>
# 日本語仕様書

## 1. 開発物概要
- **名称（仮称）**: `pg_solver`（PostgreSQL自動最適化・スロークエリキラー Extension）
- **ターゲット**: 中堅〜大企業インフラ担当、SaaS開発企業、DBA、個人開発者
- **コア思想**: 
  - 複雑化したクエリワークロードとインデックス設計の組み合わせ最適化問題を、多次元幾何学ソルバーで大域的に解き明かす。
  - 外部通信ゼロ、依存ライブラリゼロの「ポン付け」で動作する完全自立型バイナリ。

---

## 2. 提供形態・パッケージング
1. **ポン付け（ゼロ設定）アーキテクチャ**:
   - `pg_solver.so`、`pg_solver.control`、`pg_solver--0.1.0.sql` を所定のPostgreSQLライブラリ/Extensionディレクトリに配置するだけで導入完了。
   - `CREATE EXTENSION pg_solver;` のSQL 1発で即時稼働。
   - 検証用として、組み込み済みDocker環境も用意。
2. **2エディション展開**:
   - **72時間お試し版（Trial）**:
     - 初回実行から72時間（3日間）で自己消滅（機能停止・安全なノーオペレーション化）。
   - **製品版（Full / Pro）**:
     - 期限ロックなし、永久利用バイナリ。

---

## 3. 推奨技術スタック ＆ コアアーキテクチャ
1. **主要技術スタック**:
   - **アドオン基盤**: **Rust ＋ `pgrx`**
     - PostgreSQL公式・コミュニティ標準のモダンExtension開発フレームワーク。
     - メモリ安全性（メモリリーク・セグフォ根絶）により、アドオン起因の本番DBクラッシュリスクを構造的に排除。
     - `#[pg_extern]` マクロにより、SQL関数（`pg_solver_report()`, `pg_solver_apply()` 等）を最小限のコードで型安全に定義。
   - **最適化炉心**: **Rust（多次元幾何学ソルバー / POCS / 制約充足）**
     - C言語ネイティブ同等の極小オーバーヘッド・極大演算速度。
   - **DB内部連携**: **SPI（Server Programming Interface）**
     - PostgreSQLの同一メモリ空間から `pg_stat_statements` や統計カタログテーブルへナノ秒単位で安全に直接アクセス。
   - **パッケージング・ビルド**: **`cargo pgrx package`**
     - 各PGバージョンおよびアーキテクチャ（Linux x86_64）向けのバイナリをビルド。

2. **幾何学炉心の完全ブラックボックス化 ＆ 知財防護**:
   - `cargo build --release` ＋ LTO（Link-Time Optimization）＋ `strip`（シンボル完全除去）により、バイナリを極限まで難読化・最小化。
   - 多次元幾何学最適化ソルバーコア部および72時間時限チェック機構をバイナリ内部に完全に秘匿。外部逆コンパイルによる解析を無力化。

3. **完全ローカル完結（Zero Egress / Zero Network）**:
   - 外部インターネットへのライセンス認証通信やデータ送信は一切行わない。
   - 金融機関、医療機関、閉域イントラネット、オンプレミス、クラウドプライベートVPCでもセキュリティ審査を即座にパスする仕様。

---

## 4. 主要機能要件
1. **非侵襲サンプリング（低負荷常駐）**:
   - `pg_stat_statements` 等の内部統計情報および実行計画から、DB本体に負荷をかけずにスロークエリと全表走査（Seq Scan）を検知。
2. **幾何学的組み合わせ最適化（大域解算出）**:
   - インデックス爆発（INSERT/UPDATE遅延やディスク圧迫）を起こさず、**最小限のインデックス追加でシステム全体のクエリレイテンシを最小化**する最適解を算出。
3. **診断レポート出力（Before / After エビデンス提示・劇的改善の視覚化）**:
   - `SELECT * FROM pg_solver_report();` により、「どこがどう改善されるか」の具体的な差分・数値を誰でも一目でわかる形式で出力。
4. **ワンタッチ適用（安全弁）**:
   - `SELECT pg_solver_apply();` で推奨インデックスを一括適用（`CREATE INDEX CONCURRENTLY` でテーブルロックなし）。
5. **72時間後の自己消滅挙動**:
   - お試し期間（72時間）経過後は、関数実行時に案内（NOTICE）を表示し、アドオンの解析機能のみ安全に停止（DB本体の動作・データには一切影響を与えない）。

---

## 5. 実機ベンチマーク実証結果（Docker PG16 / 100万行）
- **対象テーブル**: `orders`（1,000,000 rows）
- **テストクエリ**: `SELECT * FROM orders WHERE user_id = 42 AND status = 'pending';`
- **Before（適用前）**:
  - `Parallel Seq Scan`（33万行フルスキャン）
  - 実行時間: **23.867 ms**
- **After（pg_solver_apply 適用後）**:
  - `Index Scan using idx_orders_user_id_status_opt`
  - 実行時間: **0.060 ms**
- **高速化エビデンス**: **約99.7% レイテンシ削減（処理速度 397倍速）**

---

## 6. 販売戦略・プライシング ＆ 納品パッケージ
- **プライシング戦略**:
  - **ローンチ記念特価**: **$99（買い切り・永久ライセンス）**
  - **競合比較優位性**:
    - AWS RDSスケールアップ（月額+$200〜$800の純増）や Datadog/pganalyze（月額$149〜$499のサブスク地獄）に対し、**「買い切り$99（飲み会1回分）」**という圧倒的破壊力。
- **完成パッケージ**:
  - `release_packages/pg_solver_trial_linux_x86_64.zip`（72時間時限版 / 無料配布）
  - `release_packages/pg_solver_pro_linux_x86_64.zip`（無制限Pro版）
- **販売プラットフォーム**: Stripe Managed Payments (Merchant of Record)
  - 決済URL: **https://buy.stripe.com/bJeeV6ahC7Qagno3J267S01**
