#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, receiver: &mut CubeSat, msg: Message) {
        receiver.mailbox.messages.push(msg);
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: MailBox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello sat"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();

    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
