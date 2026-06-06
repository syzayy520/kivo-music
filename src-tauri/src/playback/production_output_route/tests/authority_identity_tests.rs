use std::num::NonZeroU64;

use super::super::authority::ProductionOutputRouteIdentity;

#[test]
fn authority_identity_explicit_nonzero_identity() {
    let value = NonZeroU64::new(42).unwrap();
    let id = ProductionOutputRouteIdentity::new(value);
    assert_eq!(id.value(), value);
}

#[test]
fn authority_identity_try_new_accepts_nonzero() {
    let id = ProductionOutputRouteIdentity::try_new(100).expect("nonzero should succeed");
    assert_eq!(id.value().get(), 100);
}

#[test]
fn authority_identity_rejects_zero_when_try_new_is_used() {
    let result = ProductionOutputRouteIdentity::try_new(0);
    assert!(result.is_none(), "zero must not construct identity");
}

#[test]
fn authority_identity_equal_values_compare_equal() {
    let a = ProductionOutputRouteIdentity::try_new(7).unwrap();
    let b = ProductionOutputRouteIdentity::try_new(7).unwrap();
    assert_eq!(a, b);
}

#[test]
fn authority_identity_different_values_compare_different() {
    let a = ProductionOutputRouteIdentity::try_new(1).unwrap();
    let b = ProductionOutputRouteIdentity::try_new(2).unwrap();
    assert_ne!(a, b);
}

#[test]
fn authority_identity_is_copy_clone() {
    let id = ProductionOutputRouteIdentity::try_new(5).unwrap();
    let copied = id;
    let cloned = id.clone();
    assert_eq!(id, copied);
    assert_eq!(id, cloned);
}

#[test]
fn authority_identity_ordering_is_deterministic() {
    let a = ProductionOutputRouteIdentity::try_new(1).unwrap();
    let b = ProductionOutputRouteIdentity::try_new(2).unwrap();
    assert!(a < b);
}

#[test]
fn authority_identity_hash_is_stable() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let id = ProductionOutputRouteIdentity::try_new(99).unwrap();
    set.insert(id);
    assert!(set.contains(&id));
}
