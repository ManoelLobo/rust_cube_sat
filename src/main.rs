#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, receiver: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].receiver == receiver.id {
                return Some(self.messages.remove(i));
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    receiver: u64,
    content: String,
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let mut mailbox = Mailbox { messages: vec![] };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        // let sat = base.connect(sat_id);
        let msg = Message {
            receiver: sat_id,
            content: "Hey".to_string(),
        };

        base.send(&mut mailbox, msg)
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mailbox);
        println!("{:?}: {:?}", sat, msg);
    }
}
