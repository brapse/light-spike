use std::{thread,time};
use std::collections::HashMap;
use crossbeam::channel;

pub type Height = u64;
pub type PeerID = String;

pub type VerificationResult = Result<Header, &'static str>;

pub struct Callback {
    inner: Box<dyn FnOnce(VerificationResult) + Send>,
}

impl std::fmt::Debug for Callback{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callback").finish()
    }
}

impl Callback {
    fn new(inner: impl FnOnce(VerificationResult) -> () + Send + 'static) -> Callback {
        return Callback {
            inner: Box::new(inner),
        }
    }

    fn call(self, result: VerificationResult) -> () {
        (self.inner)(result);
    }
}

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

#[derive(Debug)]
pub struct Header {
}

enum Fork {
    PassedVerification(PeerID),
    FailedVerification(PeerID),
}

struct Instance {
    peer_id: PeerID,
}

impl Instance {
    fn verify_to_target(&mut self, _height: Height) -> VerificationResult {
        // TODO
        return Err("not implemented")
    }
}

#[derive(Debug)]
pub enum Event {
    Terminate(channel::Sender<()>),
    VerifyToTarget(Height, Callback),
    Verified(Header),
    FailedVerification(),
}

// Supervisor
pub struct Supervisor {
    peers: PeerList,
    sender: channel::Sender<Event>,
    receiver: channel::Receiver<Event>,
}

impl Supervisor {
    pub fn new() -> Supervisor {
        let (sender, receiver) = channel::unbounded::<Event>();
        return Supervisor {
            sender,
            receiver,
            peers: PeerList {
                primary: PeerID::from("1"),
                peers: HashMap::new(),
            }
        }
    }

    fn verify_to_target(&mut self, height: Height) -> VerificationResult {
        // Check store or whatever
        while let Some(mut primary) = self.peers.primary() {
            let verified = primary.verify_to_target(height);

            match verified {
                Ok(header) => {
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
                            }
                        },
                        None => {
                            // TODO: update trusted state
                            // TODO: send to relayer, maybe the run method does this?
                        }
                    }
                },
                // Verification failed
                Err(_err) => {
                    self.peers.swap_primary();
                }
            }
        }

        return Err("not implemeneted");
    }

    fn report_evidence(&mut self, _header: &Header) {
        // TODO
    }

    fn detect_forks(&mut self, _header: &Header) -> Option<Vec<Fork>> {
        return None
     }

    pub fn handler(&mut self) -> Handler {
        let sender = self.sender.clone();

        return Handler::new(sender);
    }

    // Consume the instance here but return a runtime which will allow interaction
    // Maybe return an output channnel here?
    pub fn run(mut self) {
        thread::spawn(move || {
            loop {
                let event = self.receiver.recv().unwrap();
                match event {
                    Event::Terminate(sender) => {
                        println!("Terminating light client");
                        sender.send(()).unwrap();
                        return
                    },
                    Event::VerifyToTarget(height, callback) => {
                        let outcome = self.verify_to_target(height);
                        callback.call(outcome);
                    },
                    _ => {
                        // NoOp?
                    },
                }
            }
        });
    }
}



pub struct Handler {
    sender: channel::Sender<Event>,
}

impl Handler {
    // How do we connect with the runtime?
    pub fn new(sender: channel::Sender<Event>) -> Handler {
        return Handler {
            sender,
        }
    }

    pub fn verify_to_target(&mut self, height: Height) -> Result<Header, &'static str> {
        let (sender, receiver) = channel::bounded::<Event>(1);
        let callback = Callback::new(move |result| {
            // we need to create an event here
            let event = match result {
                Ok(header) => {
                    Event::Verified(header)
                },
                Err(err) => {
                    Event::FailedVerification()
                }
            };
            sender.send(event).unwrap();
        });

        self.sender.send(Event::VerifyToTarget(height, callback)).unwrap();

        match receiver.recv().unwrap() {
            Event::Verified(header) => {
                return Ok(header);
            },
            Event::FailedVerification() => {
                return Err("too bar");
            },
            _ => {
                return Err("that was unexpected");
            }

        }
    }

    pub fn verify_to_target_async(&mut self, height: Height, callback: fn(VerificationResult) -> ()) {
        let event = Event::VerifyToTarget(height, Callback::new(callback));
        self.sender.send(event).unwrap();
    }

    pub fn terminate(&mut self) {
        let (sender, receiver) = channel::bounded::<()>(1);

        self.sender.send(Event::Terminate(sender)).unwrap();
        receiver.recv().unwrap();
    }
}
