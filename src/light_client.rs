use std::collections::HashMap;
type Height = u64;
pub type PeerID = String;


// TODO: Error handling of mutations
struct PeerList {
    peers: HashMap<PeerID, Instance>,
    primary: PeerID,
}

impl PeerList {
    fn primary(&mut self) -> Option<Instance> {
        return None
    }

    fn remove_secondary(&mut self, peer_id: PeerID) {
        // TODO
    }

    fn swap_primary(&mut self) {
        // TODO
    }
}
struct Header {
}

enum Fork {
    PassedVerification(PeerID),
    FailedVerification(PeerID),
}

struct Instance {
    peer_id: PeerID,
}

impl Instance {
    fn verify_to_target(&mut self, _height: Height) -> Option<Header>{
        // TODO
        return None
    }
}

// Supervisor
struct Supervisor {
    peers: PeerList,
}

impl Supervisor {
    fn verify_to_target(&mut self, height: Height) -> Option<Header> {
        while let Some(mut primary) = self.peers.primary() {
            let verified = primary.verify_to_target(height);

            match verified {
                Some(header) => {
                    // so here we need to pass in the the rest of the peers
                    // But I don't think we will be able to because it's already borrowed as
                    // mutable
                    let outcome = self.detect_forks(&header);

                    match outcome {
                        Some(forks) => {
                            let mut detected = false;
                            for fork in forks {
                                match fork {
                                    Fork::PassedVerification(_peer_id) => {
                                        self.report_evidence(&header);
                                        detected = true;
                                    },
                                    Fork::FailedVerification(peer_id) => {
                                        // mutate peer list
                                        self.peers.remove_secondary(peer_id);
                                    },
                                }
                            }
                            if detected {
                                println!("Fork detected, exiting");
                                return None;
                            }
                        },
                        None => {
                            // TODO: update trusted state
                            // TODO: send to relayer
                        }
                    }
                },
                // Verification failed
                None => {
                    self.peers.swap_primary();
                }
            }
        }

        return None
    }

    fn report_evidence(&mut self, _header: &Header) {
        // TODO
    }

    fn detect_forks(&mut self, _header: &Header) -> Option<Vec<Fork>> {
        return None
     }

    // TODO: Simple runtime which checks replies to requests from subscriptions (relayer or RPC) to 
}
