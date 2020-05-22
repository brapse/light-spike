/// TODO: Motivate architecture with flows of events going through components
/// A spike on the light client event architecture.
/// The goal:
/// 1. Split up IO from logic
/// 1.1 Allow determinstic simulation
/// 2. Have the light client generated the trusted state that provide read only access to that
///    trusted state in seperate threads
use std::{thread,time};
use std::time::{Instant, Duration};
use crossbeam::channel;
use light_spike::{TrustedState, TSReadWriter};

// IO
#[derive(Debug)]
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

#[derive(Debug)]
enum VerifierEvent {
    NoOp(),
    NextRequest(),
    Request(),
    Response(),
    Verified(),
}

struct Verifier {
    // 
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
    last_progress: Instant,
    timeout: Duration,
}

impl LightClient {
    fn new(trusted_state: TSReadWriter, io: IO, verifier: Verifier) -> LightClient {
        return LightClient {
            trusted_state: trusted_state,
            io,
            verifier,
            last_progress: Instant::now(),
            timeout: Duration::from_millis(1),
        }
    }

    // TODO: Align Events with ADR
    fn handle(&mut self, event: Event) -> Event {
        println!("event! {:?}", event);
        match event {
            Event::Tick(instant) => {
                if instant.saturating_duration_since(self.last_progress) > self.timeout {
                    return Event::Timeout()
                }
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
                self.last_progress = Instant::now();
                let next = VerifierEvent::NextRequest();
                return self.verifier.handle(next).into()
            },
            _ => return Event::Tick(Instant::now()),
        }
    }

    fn run(mut self,
        sender: channel::Sender<Event>,
        receiver: channel::Receiver<Event>,
        output: channel::Sender<Event>) {
        self.last_progress = Instant::now();
        thread::spawn(move || {
            loop {
                let event = receiver.recv().unwrap();
                match event {
                    Event::Terminate() => {
                        println!("Terminating node");
                        output.send(Event::Terminated()).unwrap();
                        return
                    },
                    Event::Timeout() => {
                        output.send(event).unwrap();
                        return
                    },
                    _ => {
                        let next = self.handle(event);
                        sender.send(next).unwrap();
                    },
                }
            }
        });
    }
}


#[derive(Debug)]
enum Event {
    IOEvent(IOEvent),
    VerifierEvent(VerifierEvent),
    Terminate(),
    Terminated(),
    Tick(Instant),
    Timeout(),
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

    let (sender, receiver) = channel::unbounded::<Event>();
    let (output_sender, output_receiver) = channel::unbounded::<Event>();
    let internal_sender = sender.clone();
    light_client.run(internal_sender, receiver, output_sender);

    sender.send(Event::Tick(Instant::now()));
    let conclusion = output_receiver.recv().unwrap();
    println!("Conclusion: {:?}", conclusion);
}
