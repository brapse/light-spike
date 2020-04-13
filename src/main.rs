
/// A spike on the light client event architecture.
/// The goal:
/// 1. Split up IO from logic
/// 1.1 Allow determinstic simulation
/// 2. Have the light client generated the trusted state that provide read only access to that
///    trusted state in seperate threads
use std::thread;
use std::time::{Duration};
use crossbeam::{channel, tick, select};
use light_spike::{TrustedState, TSReadWriter};

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

struct LightClient {
    io: IO,
    verifier: Verifier,
    trusted_state: TSReadWriter,
}

impl LightClient{
    fn new(trusted_state: TSReadWriter, io: IO, verifier: Verifier) -> LightClient {
        return LightClient {
            trusted_state: trusted_state,
            io,
            verifier,
        }
    }

    // TODO: rough outline of how the protocol will proceed
    // Why does it need to be this way?
    // Such that we can represent any sequenceof atomic transformations
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

    fn run(mut self) {
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
                                let next = self.handle(event);
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
    let trusted_state =  TrustedState::new(); // TODO: Subjective intialization
    let (ts_reader, ts_writer) = trusted_state.split();
    let verifier = Verifier::new();
    let io = IO::new();
    let light_client = LightClient::new(ts_writer, io, verifier);

    light_client.run();
    //sleep for a few seconds
    // exit
}
