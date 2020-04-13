use std::sync::{Arc, RwLock};

#[derive(Clone)]
struct Commit {
}

#[derive(Clone)]
struct ValidatorSet {
}

#[derive(Clone)]
struct TrustedState {
    commit: Commit,
    validator_set: ValidatorSet,
}

impl TrustedState {
    fn new() -> TrustedState {
        return TrustedState {
            commit: Commit {},
            validator_set: ValidatorSet{},
        }
    }

    fn split(self) -> (TSReader, TSReadWriter) {
        let ts = Arc::new(RwLock::new(self));

        let reader = TSReader { ts: Arc::clone(&ts) };
        let writer = TSReadWriter { ts };

        return (reader, writer)

    }
}

struct TSReader {
    // XXX: Make this a read writer mutex
    ts: Arc<RwLock<TrustedState>>,
}

impl TSReader {
    fn get(&self) -> TrustedState {
        let ts = self.ts.read().unwrap();
        return ts.clone()
    }
}

struct TSReadWriter {
    ts: Arc<RwLock<TrustedState>>,
}

impl TSReadWriter {
    fn get(&self) -> TrustedState {
        let ts = self.ts.read().unwrap();
        return ts.clone()
    }

    fn set(&mut self, trusted_state: TrustedState) {
        let mut ts = self.ts.write().unwrap();

        ts.commit = trusted_state.commit;
        ts.validator_set = trusted_state.validator_set;
    }
}
