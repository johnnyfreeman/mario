use std::process;

fn main() {
    let mut player = Player::new();

    // player.move_forward();
    player.encounter(Transition::Mushroom);

    player.encounter(Transition::Flower);
    player.encounter(Transition::Feather);
    player.encounter(Transition::Damage);
    player.encounter(Transition::Damage);
    player.encounter(Transition::Mushroom);
    assert!(player.state == State::SuperMario);
    assert!(player.lives == 2);

    player.encounter(Transition::Damage);
    assert!(player.state == State::Mario);
    assert!(player.lives == 2);

    player.encounter(Transition::Damage);
    assert!(player.lives == 1);

    player.encounter(Transition::Damage);
    assert!(player.lives == 0);

    player.encounter(Transition::Damage);
    // game over.
}

struct Player {
    state: State,
    lives: i32,
    map: Map,
}

impl Player {
    fn new() -> Self {
        Self {
            state: State::Mario,
            lives: 3,
            map: Map::new(),
        }
    }

    fn encounter(&mut self, power: Transition) {
        match (&self.state, power) {
            (State::Mario, Transition::Mushroom) => self.state = State::SuperMario,
            (_, Transition::Flower) => self.state = State::FireMario,
            (_, Transition::Feather) => self.state = State::CapeMario,
            (_, Transition::Mushroom) => self.one_up(),
            (State::Mario, Transition::Damage) => self.die(),
            (_, Transition::Damage) => self.state = State::Mario,
        }
    }

    fn one_up(&mut self) {
        self.lives += 1;
    }

    fn die(&mut self) {
        match &self.lives {
            1.. => {
                self.lives -= 1;
                self.map.restart();
            }
            _ => self.game_over(),
        }
    }

    fn game_over(&self) {
        println!("Game over.");
        process::exit(0);
    }
}

enum Transition {
    Damage,
    Feather,
    Flower,
    Mushroom,
}

#[derive(PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

struct Map {
    position: i32,
}

impl Map {
    fn new() -> Self {
        Self { position: 0 }
    }

    fn restart(&mut self) {
        self.position = 0;
    }
}
