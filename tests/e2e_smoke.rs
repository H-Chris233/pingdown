mod common;
use common::StubSystem;

use std::sync::{Arc, Mutex};

use pingdown::config::Config;
use pingdown::monitor::test_emergency_loop;
use pingdown::ping::check_status;
use pingdown::runtime::Metrics;

fn cfg(addrs: Vec<&str>, strict: bool, tries: u64) -> Config {
    Config {
        vec_address: addrs.into_iter().map(|s| s.to_string()).collect(),
        strict,
        secs_for_normal_loop: 0,
        secs_for_emergency_loop: 0,
        times_for_emergency_loop: tries,
        quiet: true,
        status_only: true,
        progress: false,
        verbose: 0,
    }
}

#[test]
fn smoke_any_success_and_strict_modes() {
    let sys = StubSystem::with_static(&[("good", true), ("bad", false)]);
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    let any = cfg(vec!["bad", "good"], false, 1);
    let (status_any, su, fa) = check_status(&any, &metrics, &sys);
    assert!(status_any);
    assert_eq!((su, fa), (1, 1));

    let strict = cfg(vec!["bad", "good"], true, 1);
    let (status_strict, su, fa) = check_status(&strict, &metrics, &sys);
    assert!(!status_strict);
    assert_eq!((su, fa), (1, 1));
}

#[test]
fn emergency_escalates_and_recovers_without_shutdown() {
    let sys = StubSystem::new();
    // First attempt fails, second succeeds -> recovery before shutdown
    sys.push_sequence("down", vec![false, true]);

    let cfg = cfg(vec!["down"], false, 1);
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    test_emergency_loop(&cfg, &metrics, &sys);
    // No shutdown should have been invoked
    assert_eq!(sys.take_shutdowns(), 0);
}
