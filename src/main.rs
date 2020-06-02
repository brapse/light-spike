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
use light_spike::light_client::{Supervisor, Event as LEvent};
use light_spike::relayer::Relayer;

// What is a good test?
// Run all the components,have them commuicate and exit
fn main() {
    // let's launch a supervisor an a relayer and stich them together with channels
    // TODO: Subjective initialization
    let light_client = Supervisor::new();
    let relayer = Relayer::new();

    // Node Channel
    let (node_sender, node_receiver) = channel::unbounded::<LEvent>();

    // RPC
    // TODO:

    // launch light_client
    let (light_client_sender, light_client_receiver)  = channel::unbounded::<LEvent>();
    let light_client_output = node_sender.clone();

    // TODO: This is where we simplify but run returning a handler to an internal runtime
    thread::spawn(move || {
        // so here now we need to provide an channel to read and write to
        light_client.run(light_client_receiver, light_client_output);
    });

    // So handlers work well for sending, what about receiving?
    // Handlers assuming that we want synchronous access to an asynchronous process
    // Do we ever want asynchronous access access to an asynchronous process?
    // Example would be: When we want non blocking component interaction
    // In this case we would need to have a handler specific subscription. Events processed by the 
    // The foreign component would then need to be routed to specific subcriptions
    // But when can we envision this happening? Maybe the relayer wants to accumulate events while
    // it waits for the light client to update
    // But for a first pass maybe we just provide synchronous access to asycnrhonous components

    // let (relayer_client_sender, relyaer_client_receiver)  = channel::unbounded::<Event>();
    // thread::spawn(move || {
        // realyer.run(light_client_sender, light_client_receiver);
    //});
    // launch relayer
    //
    //

    // Maybe we could simplify the all the channel nonsense with the handler pattern in which
    // construction of an instance returns a handler which hides all the events and channel
    // abstraction

    light_client_sender.send(LEvent::VerifyToTarget(32)).unwrap();
    light_client_sender.send(LEvent::Terminate()).unwrap();

    // maybe just put the node loop here
    while let event = node_receiver.recv().unwrap() {
        match event {
            LEvent::Terminated() => {
                println!("terminated");
                return
            },
            _ => {
                println!("{:?}", event);
            }
        }
    }
}
