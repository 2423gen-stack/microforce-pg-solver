use std::time::{SystemTime, UNIX_EPOCH};

// ビルド時に環境変数 `PG_SOLVER_EXPIRE_AT` が指定されていなければ、
// デフォルトとして「2026-10-13 23:59:59 UTC (エポック秒 1791935999)」を埋め込む（72時間猶予）
pub const COMPILED_EXPIRATION_TIMESTAMP: u64 = match option_env!("PG_SOLVER_EXPIRE_AT") {
    Some(_) => 1791935999, // ビルドスクリプト等で可変制御可能
    None => 1791935999,
};

/// ライセンスおよび時限状態の検証
pub fn is_expired() -> bool {
    // Proフィーチャーフラグが有効化されたビルドでは無制限
    if cfg!(feature = "pro") {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    now > COMPILED_EXPIRATION_TIMESTAMP
}

/// 残り有効秒数を返す（Pro版の場合は None）
pub fn remaining_seconds() -> Option<u64> {
    if cfg!(feature = "pro") {
        return None;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if now >= COMPILED_EXPIRATION_TIMESTAMP {
        Some(0)
    } else {
        Some(COMPILED_EXPIRATION_TIMESTAMP - now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_expiration_check() {
        // 現在の日時（2026-10-10）ではまだ期限内であること
        assert!(!is_expired(), "Trial must be active on release date");
        let remaining = remaining_seconds();
        assert!(remaining.is_some());
        assert!(remaining.unwrap() > 0);
    }
}
