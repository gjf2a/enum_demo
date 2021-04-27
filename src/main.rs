#[derive(Debug,Copy,Clone)]
struct Counter {
    count: usize
}

impl Counter {
    pub fn new() -> Self {Counter { count: 0}}

    pub fn bump(&mut self) {
        self.count += 1;
    }
}

#[derive(Debug, Copy, Clone)]
enum Letters {
    A(Counter),
    B(Counter),
    C(Counter)
}

impl Letters {
    pub fn update(&mut self) {
        match self {
            Letters::A(counter) => {counter.bump()}
            Letters::B(counter) => {counter.bump(); counter.bump()}
            Letters::C(counter) => {counter.bump(); counter.bump(); counter.bump()}
        }
    }
}

fn main() {
    let mut things = [Letters::A(Counter::new()), Letters::B(Counter::new()), Letters::C(Counter::new()), Letters::A(Counter::new())];

    println!("things: {:?}", things);
    for _ in 0..10 {
        for i in 0..things.len() {
            things[i].update();
        }
    }
    println!("after: {:?}", things);
}
