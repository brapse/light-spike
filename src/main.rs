/// Make a dummy light client which executes and updates a trusted state while seperating
/// Start With IO and Verifier
/// Add Detector Later
/// How will the light client be used?
/// In all likelyhood it will be initiated within a process and provide other components with a
/// TrustedState

use std::thread;
use std::time::{Duration, Instant};
use crossbeam::{channel, tick, select};

// IO
enum IOEvent {
    NoOp(),
}
struct IO {
}

impl IO {
    fn new() -> IO {
        return IO{}
    }

    fn handle(&mut self, event: IOEvent) -> IOEvent {
        return IOEvent::NoOp()
    }
}

// Verifier
enum VerifierEvent {
    NoOp()
}

struct Verifier {
}

impl Verifier {
    fn new() -> Verifier {
        return Verifier{}
    }
    fn handle(&mut self, event: VerifierEvent) -> VerifierEvent{
        return VerifierEvent::NoOp()
    }
}

// Combine them
struct Control {
    io: IO,
    verifier: Verifier,
}

// How do we export the trusted state?
impl Control {
    fn new(io: IO, verifier: Verifier) -> Control {
        return Control {
            io,
            verifier
        }
    }

    fn handle(&mut self, event: Event) -> Event {
        match event {
            Event::IOEvent(event) => {
                return self.io.handle(event).into();
            },
            Event::VerifierEvent(event) => {
                return self.verifier.handle(event).into();
            },
            // XXX: Detector
            _ => return Event::NoOp(), // TODO:  Remove this when complete
        }
    }
}

enum Event {
    IOEvent(IOEvent),
    VerifierEvent(VerifierEvent),
    Terminate(),
    NoOp(),
}

impl From<IOEvent> for Event {
    fn from(event: IOEvent) -> Self {
        Event::IOEvent(event)
    }
}

impl From<VerifierEvent> for Event {
    fn from(event: VerifierEvent) -> Self {
        Event::VerifierEvent(event)
    }
}

fn main() {
    let verifier = Verifier::new();
    let io = IO::new();
    let mut control = Control::new(io, verifier);

    // Should we export the trusted state somehow?
    let (control_sender, control_receiver) = channel::bounded::<Event>(1);
    let loop_sender = control_sender.clone();
    let ticker = tick(Duration::from_millis(100));
    thread::spawn(move || {
        // We can have a timeout here for liveliness if we like
        loop {
            select! {
                recv(control_receiver) -> maybe_event => { // TODO: Handle channel drop
                    let event = maybe_event.unwrap();
                    match event {
                        Event::Terminate() => {
                            println!("Terminating node");
                            return
                        },
                        _ => {
                            let next = control.handle(event); 
                            control_sender.send(next).unwrap(); // TODO: handle error
                        },
                    }
                },
                recv(ticker) -> tick => {
                    // Drive the FSM forward
                },
            }
        }
    });
}
