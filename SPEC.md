# PostgreSQL自動最適化・スロークエリキラー（pg_solver）開発構想・仕様書

## 1. 開発物概要
- **名称（仮称）**: `pg_solver`（PostgreSQL自動最適化・スロークエリキラー Extension）
- **ターゲット**: 中堅〜大企業インフラ担当、SaaS開発企業、DBA、個人開発者
- **コア思想**: 
  - 複雑化したクエリワークロードとインデックス設計の組み合わせ最適化問題を、旦那様の多次元幾何学ソルバーで大域的に解き明かす。
  - 外部通信ゼロ、依存ライブラリゼロの「ポン付け」で動作する完全自立型バイナリ。

---

## 2. 提供形態・パッケージング
1. **ポン付け（ゼロ設定）アーキテクチャ**:
   - `pg_solver.so`、`pg_solver.control`、`pg_solver--1.0.sql` を所定のPostgreSQLライブラリ/Extensionディレクトリに配置するだけで導入完了。
   - `CREATE EXTENSION pg_solver;` のSQL 1発で即時稼働。
   - 検証用として、組み込み済みDockerイメージ（`docker pull microforce/postgres-solver:16`）も用意。
2. **2エディション展開**:
   - **72時間お試し版（Trial）**:
     - インストール（または配布バイナリ固定日時）から72時間（3日間）で自己消滅（機能停止・安全なノーオペレーション化）。
   - **製品版（Full / Pro）**:
     - 期限ロックなし、永久利用バイナリ。

---

## 3. 推奨技術スタック ＆ コアアーキテクチャ
1. **主要技術スタック**:
   - **アドオン基盤**: **Rust ＋ `pgrx`（旧 pgx）**
     - PostgreSQL公式・コミュニティ標準のモダンExtension開発フレームワーク。
     - メモリ安全性（メモリリーク・セグフォ根絶）により、アドオン起因の本番DBクラッシュリスクを構造的に排除。
     - `#[pg_extern]` マクロにより、SQL関数（`pg_solver_report()`, `pg_solver_apply()` 等）を最小限のコードで型安全に定義。
   - **最適化炉心**: **Rust（多次元幾何学ソルバー / POCS / 制約充足）**
     - C言語ネイティブ同等の極小オーバーヘッド・極大演算速度。
   - **DB内部連携**: **SPI（Server Programming Interface）**
     - PostgreSQLの同一メモリ空間から `pg_stat_statements` や統計カタログテーブルへナノ秒単位で安全に直接アクセス。
   - **パッケージング・ビルド**: **`cargo pgrx package` ＋ GitHub Actions**
     - 各PGバージョン（PostgreSQL 14, 15, 16, 17）およびアーキテクチャ（Linux x86_64, aarch64）向けの `.so` バイナリを一括クロスビルド。

2. **幾何学炉心の完全ブラックボックス化 ＆ 知財防護**:
   - `cargo build --release` ＋ LTO（Link-Time Optimization）＋ `strip`（シンボル完全除去）により、バイナリを極限まで難読化・最小化。
   - 量子幾何学最適化ソルバーコア部および72時間時限チェック機構をバイナリ内部に完全に秘匿。外部逆コンパイルによる解析を無力化。

3. **完全ローカル完結（Zero Egress / Zero Network）**:
   - 外部インターネットへのライセンス認証通信やデータ送信は一切行わない。
   - 金融機関、医療機関、閉域イントラネット、オンプレミス、クラウドプライベートVPCでもセキュリティ審査を即座にパスする仕様。

---

## 4. 主要機能要件
1. **非侵襲サンプリング（低負荷常駐）**:
   - `pg_stat_statements` 等の内部統計情報および実行計画（EXPLAIN）から、DB本体に負荷をかけずにスロークエリと全表走査（Seq Scan）を検知。
2. **幾何学的組み合わせ最適化（大域解算出）**:
   - インデックス爆発（INSERT/UPDATE遅延やディスク圧迫）を起こさず、**最小限のインデックス追加でシステム全体のクエリレイテンシを最小化**する最適解を算出。
3. **診断レポート出力（Before / After エビデンス提示・劇的改善の視覚化）**:
   - `SELECT pg_solver_report();` または `SELECT * FROM pg_solver_diff();` により、「どこがどう改善されたか（されるか）」の具体的な差分・数値を誰でも一目でわかる形式で出力。
   - **出力イメージ例**:
     ```
     ================================================================================
     [pg_solver Optimization Report]
     Overall Estimated Latency Reduction: 74.2%  (P99: 1,420ms -> 85ms)
     Total Queries Analyzed: 18,420 | Problematic Scans Detected: 3
     --------------------------------------------------------------------------------
     [Issue #1] Heavy Sequential Scan on `orders` (Occurrences: 4,120/hour)
       - Target Query: SELECT * FROM orders WHERE user_id = $1 AND status = 'pending'
       - Before: Seq Scan on orders (Cost: 1420.50, Avg Latency: 320ms, CPU: High)
       - Action: Recommended Partial Index
                 CREATE INDEX CONCURRENTLY idx_orders_user_pending 
                 ON orders (user_id) WHERE status = 'pending';
       - After:  Index Scan using idx_orders_user_pending (Cost: 4.12, Avg Latency: 2.1ms)
       - Improvement: 99.3% Speedup! (Buffer Reads: 4,200 pages -> 3 pages)
     --------------------------------------------------------------------------------
     [Issue #2] Redundant / Unused Index Waste Detected
       - Unused Index: `idx_orders_created_at_old` (Zero reads in 3 days, 1.2 GB waste)
       - Recommendation: DROP INDEX CONCURRENTLY idx_orders_created_at_old;
       - Benefit: Reclaim 1.2 GB Disk & 15% INSERT Speedup
     ================================================================================
     Run `SELECT pg_solver_apply();` to execute these optimizations safely.
     ```
   - 現場エンジニアが上司・経営陣に製品版購入の稟議を通すための決定打（エビデンス）を提供。
4. **ワンタッチ適用 ＆ ワンタッチロールバック（安全弁）**:
   - `SELECT pg_solver_apply();` で推奨インデックス・チューニングを一括適用。
   - `SELECT pg_solver_revert();` で即座に元の状態へ差し戻し可能な安全設計。
5. **72時間後の自己消滅挙動**:
   - お試し期間（72時間）経過後は、関数実行時に以下のような案内を表示し、アドオンの解析機能のみ安全に停止（DB本体の動作・データには一切影響を与えない）：
     ```
     NOTICE: pg_solver trial period (72 hours) has expired.
     HINT: Purchase the full version at https://microforce.lemonsqueezy.com/... (or microforce.dev)
     ```

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
    - AWS RDSスケールアップ（月額+$200〜$800の純増）や Datadog/pganalyze（月額$149〜$499のサブスク地獄）に対し、**「買い切り$99（飲み会1回分 / 約1.5万円）」**という圧倒的破壊力。
    - 現場エンジニアが上司の稟議を通さず、部署カードや自腹で即決済できる衝動買いライン。
  - **将来展開**:
    - 初速の波（Show HN / 拡散）が落ち着いた後、通常価格 $199、企業向け $299 への段階的引き上げを検討。

- **完成パッケージ**:
  - `release_packages/pg_solver_trial_linux_x86_64.zip`（72時間時限版 / 無料配布）
  - `release_packages/pg_solver_pro_linux_x86_64.zip`（無制限Pro版 / Lemon Squeezy販売）
- **販売プラットフォーム**: Lemon Squeezy
  - 面倒な世界各国の消費税・VAT計算・納税代行（MoR）を完全委託。
  - microforce.dev LPに購入ボタンを配置し、バイナリを即時ダウンロード配布。
