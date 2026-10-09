-- src/pg_solver.sql
-- (SQL生成エラー回避用手動定義)

CREATE OR REPLACE FUNCTION pg_solver_report() RETURNS text
    LANGUAGE c STRICT AS 'MODULE_PATHNAME', 'pg_solver_report_wrapper';

CREATE OR REPLACE FUNCTION pg_solver_apply() RETURNS text
    LANGUAGE c STRICT AS 'MODULE_PATHNAME', 'pg_solver_apply_wrapper';
