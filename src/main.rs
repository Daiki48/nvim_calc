use neovim_lib::{
    Neovim,
    NeovimApi,
    Session
};

struct Calc;

impl Calc {
    fn new() -> Calc {
        Calc{}
    }

    fn add(&self, nums: Vec<i64>) -> i64 {
        nums.iter().sum::<i64>()
    }

    fn multiply(&self, p: i64, q: i64) -> i64 {
        p * q
    }
}

struct EventHander {
    nvim: Neovim,
    calc: Calc,
}

impl EventHander {
    fn new() -> EventHander {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let calc = Calc::new();

        EventHander { nvim, calc }
    }

    fn recv(&mut self) {
        let rcv = self.nvim.session.start_event_loop_channel();

        for (event, values) in rcv {
            match Msg::from(event) {
                Msg::Add => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();

                    let sum = self.calc.add(nums);
                    self.nvim
                        .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                        .unwrap();
                }
                Msg::Multiply => {
                    let mut nums = values.iter();
                    let p = nums.next().unwrap().as_i64().unwrap();
                    let q = nums.next().unwrap().as_i64().unwrap();

                    let product = self.calc.multiply(p, q);
                    self.nvim
                        .command(&format!("echo \"Product: {}\"", product.to_string()))
                        .unwrap();
                }
                Msg::Unknown(unev) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", unev))
                        .unwrap();
                }
            }
        }
    }
}

enum Msg {
    Add,
    Multiply,
    Unknown(String),
}

impl From<String> for Msg {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Msg::Add,
            "multiply" => Msg::Multiply,
            _ => Msg::Unknown(event),
        }
    }
}

fn main() {
    let mut event_handler = EventHander::new();

    event_handler.recv();
}
