/// Make a dummy light client which executes and updates a trusted state while seperating
/// Start With IO and Verifier
/// Add Detector Later
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
    // XXX: will probably need &mut self
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

// How do we stop this thing?
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
                return self.io.handle(event).into(); // We require From IOEvent => Event
            },
            Event::VerifierEvent(event) => {
                return self.verifier.handle(event).into();
            },
            Event::NoOp() => return Event::NoOp(), // XXX: Should not happen
        }
    }
}

enum Event {
    IOEvent(IOEvent),
    VerifierEvent(VerifierEvent),
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
    let control = Control::new(io, verifier);

    // TODO: Setup channels and runtime
    // probably run this in a thread conncted with a channel
    // Add some tickers to drive the process

    // How will this run?
    // A queue reading loop
    // We can then break the loop by sending a terminate 
    // We then need some way of getting events out
    // Q: What is an event out? 
}
