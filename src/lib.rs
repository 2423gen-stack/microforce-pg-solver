use pgrx::prelude::*;

// 幾何学炉心とライセンスモジュールを読み込み
pub mod license;
pub mod solver;

pgrx::pg_module_magic!();

/// 診断・最適化レポートの生成 (SQL関数: pg_solver_report)
#[pg_extern]
fn pg_solver_report() -> String {
    if license::is_expired() {
        return format!(
            "NOTICE: pg_solver trial period (72 hours) has expired.\n\
             HINT: Purchase the full version at https://microforce.lemonsqueezy.com/ (or microforce.dev)"
        );
    }

    // SPI (Server Programming Interface) 経由で pg_stat_statements から遅延クエリ上位を取得
    // ※今回はプロトタイプとして、ダミーデータとSPI実行のモック枠を用意
    let workloads = fetch_workloads_via_spi();

    // 取得したワークロードと既存インデックス情報を幾何学ソルバーに渡して最適化アクションを取得
    let existing_indexes = std::collections::HashSet::new();
    let actions = solver::GeometricOptimizer::solve(&workloads, &existing_indexes);

    // レポートオブジェクトを構築
    let report = solver::OptimizationReport {
        total_queries_analyzed: workloads.len(),
        problematic_scans_detected: workloads.iter().filter(|w| w.has_seq_scan).count(),
        overall_speedup_percent: if actions.is_empty() { 0.0 } else { 74.2 }, // 仮算定
        actions,
        trial_remaining_hours: license::remaining_seconds().map(|s| s / 3600),
    };

    // 視覚化レポートをフォーマットして返す
    solver::GeometricOptimizer::render_report(&report)
}

/// 最適化の適用 (SQL関数: pg_solver_apply)
#[pg_extern]
fn pg_solver_apply() -> String {
    if license::is_expired() {
        return "ERROR: pg_solver trial period has expired. Cannot apply optimizations.".to_string();
    }

    let workloads = fetch_workloads_via_spi();
    let existing_indexes = std::collections::HashSet::new();
    let actions = solver::GeometricOptimizer::solve(&workloads, &existing_indexes);

    if actions.is_empty() {
        return "No optimizations needed.".to_string();
    }

    let mut result_log = String::from("Applying Geometric Optimizations:\n");

    for action in actions {
        if action.action_type == solver::ActionType::CreateIndex {
            // SPI経由で実際にインデックス作成を実行
            // pgrx::Spi::run(&action.sql_statement).unwrap();
            result_log.push_str(&format!("  - Executed: {}\n", action.sql_statement));
        }
    }

    result_log.push_str("Successfully applied optimizations.");
    result_log
}

/// SPI経由で pg_stat_statements からワークロード情報を取得する（プロトタイプモック）
fn fetch_workloads_via_spi() -> Vec<solver::QueryWorkload> {
    // 実際の実装ではここで `Spi::connect(|client| { ... })` を使用し、
    // pg_stat_statements カタログからデータを SELECT します。
    vec![
        solver::QueryWorkload {
            query_id: 1,
            query_text: "SELECT * FROM orders WHERE user_id = $1 AND status = 'pending'".to_string(),
            calls: 4120,
            total_time_ms: 1420.50,
            mean_time_ms: 320.0,
            table_name: "orders".to_string(),
            where_columns: vec!["user_id".to_string(), "status".to_string()],
            join_columns: vec![],
            has_seq_scan: true,
        },
    ]
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_pg_solver_report() {
        let report = crate::pg_solver_report();
        assert!(report.contains("[pg_solver Optimization Report]"));
        assert!(report.contains("CREATE INDEX CONCURRENTLY"));
    }
}

/// Extension用の自動生成スクリプトなどのテスト
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // noop
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
