
/// What do we want
/// We to design a system posed of components which can be verified independently and
/// together.
/// We want the interface to independent and composition of components of be generic enough to
/// apply between runtimes. Events

enum Event{
    Foo()
}

struct Pex {
}

impl Pex {
    fn handle(event: Event) -> Event {
        return Event::Foo()
    }
}

struct Scheduler {
}

impl Scheduler {
    fn handle(event: Event) -> Event {
        return Event::Foo()
    }
}

struct Verifier {
}

impl Verifier {
    fn handle(event: Event) -> Event {
        return Event::Foo()
    }
}

struct Detector {
}

impl Detector {
    fn handle(event: Event) -> Event {
        return Event::Foo()
    }
}

struct Control {
    scheduler: Scheduler,
    detector: Detector,
    verifier: Verifier,
}

impl Control {
    fn handle(event) -> Event {
        match e {
            SchedulerEvent(event) => {
                return self.scheduler.handle(event);
            },
            DetectorEvent(event) => {
                return self.detector.handle(event);
            },
            Verifier(event) => {
                return self.detector.handle(event);
            },
        }
    }
}

// what about the pex?
fn main() {
    let node = Node::new();
    let dispatcher = Dispatcher::new();
    let event = Event::Init()
    let next = node.handle(event);
    loop {
        match next {
            Event::Terminate() => break;
            Event::Dispatch(event) => next = dispatcher.handle(event);
            Event::Node(event) => next = node.handle(event);
        }
    }
    // cleanup
}

// What are the expensive operaitons
// * IO: Writting to peers
// * Verification : Verifying headers
// * Disk: Writting headers to 
// What do we want to test
// * The Scheduler bisection algorithm
//      *  Sequence of events init, detectionError, VerifcationError, etc
// * That's really the only part that needs deterministic simulation, everything else can be done
// async
//
