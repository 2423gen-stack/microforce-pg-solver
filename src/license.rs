use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const TRIAL_DURATION_SECONDS: i64 = 72 * 3600; // 72時間 (259,200秒)
const TIMESTAMP_FILE: &str = "/tmp/.pg_solver_installed_at";

/// ライセンス状態の検証（初回実行時から72時間）
pub fn is_expired() -> bool {
    if cfg!(feature = "pro") {
        return false;
    }

    let installed_at = get_or_init_installed_time();
    let now = current_epoch_seconds();
    now - installed_at > TRIAL_DURATION_SECONDS
}

/// 残り有効時間を取得（時間単位、Pro版は None）
pub fn remaining_hours() -> Option<i64> {
    if cfg!(feature = "pro") {
        return None;
    }

    let installed_at = get_or_init_installed_time();
    let now = current_epoch_seconds();
    let elapsed = now - installed_at;
    if elapsed >= TRIAL_DURATION_SECONDS {
        Some(0)
    } else {
        Some((TRIAL_DURATION_SECONDS - elapsed) / 3600)
    }
}

fn current_epoch_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 初回実行時刻を取得（無ければ現在時刻を隠しタイムスタンプファイルに記録）
fn get_or_init_installed_time() -> i64 {
    if let Ok(content) = fs::read_to_string(TIMESTAMP_FILE) {
        if let Ok(ts) = content.trim().parse::<i64>() {
            return ts;
        }
    }

    let now = current_epoch_seconds();
    let _ = fs::write(TIMESTAMP_FILE, now.to_string());
    now
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_duration_constant() {
        assert_eq!(TRIAL_DURATION_SECONDS, 259200);
    }
}
