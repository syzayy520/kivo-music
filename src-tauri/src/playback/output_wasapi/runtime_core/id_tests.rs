use super::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};

#[test]
fn runtime_id_new_preserves_value() {
    let id = OutputThreadRuntimeId::new(42);
    assert_eq!(id.0, 42);
}

#[test]
fn runtime_id_zero_is_empty() {
    let id = OutputThreadRuntimeId::new(0);
    assert!(id.is_empty());

    let non_zero = OutputThreadRuntimeId::new(1);
    assert!(!non_zero.is_empty());
}

#[test]
fn generation_new_preserves_value() {
    let gen = OutputThreadRuntimeGeneration::new(7);
    assert_eq!(gen.0, 7);
}

#[test]
fn generation_next_increments() {
    let gen = OutputThreadRuntimeGeneration::new(3);
    let next = gen.next();
    assert_eq!(next.0, 4);
}

#[test]
fn generation_zero_is_initial() {
    let initial = OutputThreadRuntimeGeneration::new(0);
    assert!(initial.is_initial());

    let not_initial = OutputThreadRuntimeGeneration::new(1);
    assert!(!not_initial.is_initial());
}
