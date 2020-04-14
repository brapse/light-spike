
/// A spike on the light client event architecture.
/// The goal:
/// 1. Split up IO from logic
/// 1.1 Allow determinstic simulation
/// 2. Have the light client generated the trusted state that provide read only access to that
///    trusted state in seperate threads
use std::{thread,time};
use std::time::{Duration};
use crossbeam::{channel, tick, select};
use light_spike::{TrustedState, TSReadWriter};

// IO
enum IOEvent {
    NoOp(),
    Request(),
    Response(),
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
    NoOp(),
    NextRequest(),
    Request(),
    Response(),
    Verified(),
}

struct Verifier {
}

impl Verifier {
    fn new() -> Verifier {
        return Verifier { }
    }

    fn handle(&mut self, event: VerifierEvent) -> VerifierEvent {
        return VerifierEvent::NoOp()
    }
}

struct LightClient {
    io: IO,
    verifier: Verifier,
    trusted_state: TSReadWriter,
}

// Tick => Verifier(NextRequest) => IO(Request) => Verifier(Response) => 
impl LightClient {
    fn new(trusted_state: TSReadWriter, io: IO, verifier: Verifier) -> LightClient {
        return LightClient {
            trusted_state: trusted_state,
            io,
            verifier,
        }
    }

    fn handle(&mut self, event: Event) -> Event {
        println!("event!");
        match event {
            Event::Tick() => {
                // XXX: Right now we initial a single flow, directly to verifiers.
                // In the case in which we have multiple finite state machine flows, including
                // timeouts, we can initiate different flows in a deterministic order.
                let next = VerifierEvent::NextRequest();
                return self.verifier.handle(next).into()
            },
            Event::VerifierEvent(VerifierEvent::Request()) => {
                let next = IOEvent::Request();
                return self.io.handle(next).into()
            },
            Event::IOEvent(IOEvent::Response()) => {
                let next = VerifierEvent::Response();
                return self.verifier.handle(next).into()
            },
            Event::VerifierEvent(VerifierEvent::Verified()) => {
                let next = VerifierEvent::NextRequest();
                return self.verifier.handle(next).into()
            },
            Event::NoOp() => {
                // Probably route to the verifier
                return Event::Tick();
            },
            _ => return Event::Tick(), // TODO:  Remove this when complete
        }
    }

    fn run(mut self, sender: channel::Sender<Event>, receiver: channel::Receiver<Event>) {
        thread::spawn(move || {
            // XXX: We can have a timeout here for liveliness if we like
            loop {
                let event = receiver.recv().unwrap();
                match event {
                    Event::Terminate() => {
                        println!("Terminating node");
                        return
                    },
                    _ => {
                        let next = self.handle(event);
                        sender.send(next).unwrap(); // TODO: handle error
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
    Tick(),
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
    let (ts_reader, ts_read_writer) = trusted_state.split();
    // verifier here needs the trusted state
    let verifier = Verifier::new();
    let io = IO::new();
    let light_client = LightClient::new(ts_read_writer, io, verifier);

    let (sender, receiver) = channel::bounded::<Event>(1);
    let internal_sender = sender.clone();
    light_client.run(internal_sender, receiver);

    sender.send(Event::Tick());
    println!("sleeping");
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    println!("done sleeping");
    // neeed to synchronize this
    sender.send(Event::Terminate());

    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    // TODO: remove sleeps with actual termination
}
