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
use light_spike::light_client::Supervisor;
use light_spike::relayer::Relayer;

/* TODO: Map events
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
*/

enum Event {
}
fn main() {
    // let's launch a supervisor an a relayer and stich them together with channels
    // TODO: Subjective initialization
    let light_client = Supervisor::new();
    let relayer = Relayer::new();

    // Node Channel
    let (node_sender, node_receiver) = channel::unbounded::<Event>();

    // launch light_client
    let (light_client_sender, light_client_receiver)  = channel::unbounded::<Event>();
    thread::spawn(move || {
        // light_client.run(light_client_sender, light_client_receiver);
    });

    let (relayer_client_sender, relyaer_client_receiver)  = channel::unbounded::<Event>();
    thread::spawn(move || {
        // realyer.run(light_client_sender, light_client_receiver);
    });
    // launch relayer
    //
    //
    //;Launch a runtime in this select over the channels
    thread::spawn(move || {
        let event = node_receiver.recv().unwrap();
            // Route event to relayer
            // Route event to light_client
            // handle termination
    });

    // 
    // send some events
    // Terminate
    // wait for terminated
    println!("Done");
}
