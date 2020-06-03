use std::time::{Instant, Duration};
use crossbeam::channel;
use light_spike::light_client::{Supervisor, Event as LEvent};
use light_spike::relayer::Relayer;

fn main() {
    // Spawn a master process
    let mut light_client = Supervisor::new();

    // Get a handler specifically for the relayer
    let relayer_light_client = light_client.handler();

    // Get a handler specifically for the node
    let mut node_light_client = light_client.handler();

    let relayer = Relayer::new(); // pass the handler here

    // Run (consume master)
    light_client.run();

    // RPC
    // TODO:

    // we should be able to verify to target here
    let test = node_light_client.verify_to_target(32);

    // should be able to termiante
    node_light_client.terminate();
}
