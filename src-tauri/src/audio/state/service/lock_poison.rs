use std::sync::{Mutex, MutexGuard};

/// Acquire a mutex lock while tolerating a poisoned mutex.
///
/// This is a **minimal** stability hardening for long-running service threads:
/// - Previously: a poisoned mutex could panic during lock acquisition.
/// - Now: we recover the inner value with `into_inner()`.
///
/// Behavior is otherwise unchanged: the same mutex is acquired, and the same
/// critical sections run under the same lock.
#[inline]
pub(super) fn lock_poison<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::lock_poison;
    use std::sync::{Arc, Mutex};

    #[test]
    fn poisoned_mutex_lock_does_not_panic() {
        let m = Arc::new(Mutex::new(42_i32));

        // Intentionally poison the mutex.
        // TEST ONLY: using `unwrap()` is acceptable to create the poisoned state.
        let m2 = m.clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = m2.lock().unwrap();
            panic!("poison mutex");
        }));

        // The helper must tolerate poison and still return a guard.
        {
            let mut g = lock_poison(&m);
            assert_eq!(*g, 42);
            *g = 43;
        } // drop guard before locking again

        let g2 = lock_poison(&m);
        assert_eq!(*g2, 43);
    }
}
