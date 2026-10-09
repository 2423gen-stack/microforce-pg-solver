-- 実機ベンチマーク用ダミーデータ生成（100万行）

-- 1. テスト用テーブルの作成
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 2. 100万行のランダムデータを挿入 (数秒かかります)
INSERT INTO orders (user_id, amount, status, created_at)
SELECT 
    (random() * 100000)::INT, 
    (random() * 1000)::DECIMAL(10,2), 
    CASE WHEN random() > 0.8 THEN 'pending' ELSE 'completed' END,
    NOW() - (random() * 365 * 24 * 3600 * interval '1 second')
FROM generate_series(1, 1000000);

-- 3. 統計情報の更新（オプティマイザに最新の状態を知らせる）
ANALYZE orders;
