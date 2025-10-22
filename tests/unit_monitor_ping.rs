mod common;
use common::StubSystem;

use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use pingdown::MonitorConfig;
use pingdown::ping::check_status;
use pingdown::runtime::{add_one, MetricEvent, Metrics};

fn base_config(strict: bool, addrs: Vec<&str>) -> MonitorConfig {
    MonitorConfig {
        targets: addrs.into_iter().map(|s| s.to_string()).collect(),
        strict,
        normal_interval: Duration::from_secs(1),
        emergency_interval: Duration::from_secs(1),
        emergency_retries: NonZeroU32::new(1).unwrap(),
        quiet: true,
        status_only: true,
        progress: false,
        verbose: 0,
    }
}

#[test]
fn any_success_mode_reports_ok_when_one_target_succeeds() {
    let sys = StubSystem::with_static(&[("a", false), ("b", true)]);
    let cfg = base_config(false, vec!["a", "b"]);
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    let (status, succ, fail) = check_status(&cfg, &metrics, &sys);
    assert!(status);
    assert_eq!(succ, 1);
    assert_eq!(fail, 1);
}

#[test]
fn strict_mode_requires_all_success() {
    let sys = StubSystem::with_static(&[("a", true), ("b", false)]);
    let cfg = base_config(true, vec!["a", "b"]);
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    let (status, succ, fail) = check_status(&cfg, &metrics, &sys);
    assert!(!status);
    assert_eq!(succ, 1);
    assert_eq!(fail, 1);
}

#[test]
fn metrics_are_incremented() {
    let sys = StubSystem::with_static(&[("a", true), ("b", false)]);
    let cfg = base_config(false, vec!["a", "b"]);
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    let _ = check_status(&cfg, &metrics, &sys);
    let m = metrics.lock().unwrap();
    assert_eq!(m.total_succeeds, 1);
    assert_eq!(m.total_failures, 1);

    drop(m);
    add_one(&metrics, MetricEvent::NormalLoopTimes);
    let m = metrics.lock().unwrap();
    assert_eq!(m.total_normal_loop_times, 1);
}
