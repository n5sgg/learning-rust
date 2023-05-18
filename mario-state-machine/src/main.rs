#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

#[derive(Debug)]
enum Transition {
    Feather,
    Flower,
    Mushroom,
}

struct Player {
    state: State,
}

impl Player {
    fn new() -> Self {
        Self { state: State::Mario }
    }

    fn collect(&mut self, power: Transition) {
        match (&self.state, power) {
            (State::Mario, Transition::Mushroom) => self.state = State::SuperMario,
            (_, Transition::Flower) => self.state = State::FireMario,
            (_, Transition::Feather) => self.state = State::CapeMario,
            (_, Transition::Mushroom) => {} // no change, 1up!
        }
    }
}

fn main() {
    let mut p1 = Player::new();
    p1.collect(Transition::Mushroom);
    p1.collect(Transition::Flower);
    p1.collect(Transition::Feather);
    p1.collect(Transition::Mushroom);
    p1.collect(Transition::Mushroom);
    assert!(p1.state == State::CapeMario)
}
