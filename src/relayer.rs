use crossbeam::channel;

#[derive(Debug)]
enum Event {
    Relay(), // 
    LightBlock(),
    Terminate(),
    Terminated(),
    Timeout(),
    NoOp(),
}

pub struct Relayer {
}

impl Relayer {
    pub fn new() -> Relayer {
        return Relayer {
        }
    }

    fn handle(&mut self, event: Event) -> Event {
        println!("event! {:?}", event);
        match event {
            Event::Relay() => {
                println!("new block from subscription");
            },
            Event::LightBlock() => {
                println!("requested light block from lightClient");
            },
            _ => {
                println!("dunno man");
            },
        }

        return Event::NoOp();
    }

    fn run(mut self,
        sender: channel::Sender<Event>,
        receiver: channel::Receiver<Event>,
        output: channel::Sender<Event>) {
        loop {
            // XXX: We will have to subscribe to updates from the lightClient
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
    }
}
