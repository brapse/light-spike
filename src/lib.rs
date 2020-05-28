use std::sync::{Arc, RwLock};
mod light_client;

#[derive(Clone)]
struct Commit {
}

#[derive(Clone)]
pub struct ValidatorSet {
}

#[derive(Clone)]
pub struct TrustedState {
    commit: Commit,
    validator_set: ValidatorSet,
}

impl TrustedState {
    pub fn new() -> TrustedState {
        return TrustedState {
            commit: Commit {},
            validator_set: ValidatorSet{},
        }
    }

    pub fn split(self) -> (TSReader, TSReadWriter) {
        let ts = Arc::new(RwLock::new(self));

        let reader = TSReader { ts: Arc::clone(&ts) };
        let writer = TSReadWriter { ts };

        return (reader, writer)

    }
}

pub struct TSReader {
    ts: Arc<RwLock<TrustedState>>,
}

impl TSReader {
    pub fn get(&self) -> TrustedState {
        let ts = self.ts.read().unwrap();
        return ts.clone()
    }
}

pub struct TSReadWriter {
    ts: Arc<RwLock<TrustedState>>,
}

impl TSReadWriter {
    pub fn get(&self) -> TrustedState {
        let ts = self.ts.read().unwrap();
        return ts.clone()
    }

    pub fn set(&mut self, trusted_state: TrustedState) {
        let mut ts = self.ts.write().unwrap();

        ts.commit = trusted_state.commit;
        ts.validator_set = trusted_state.validator_set;
    }
}
