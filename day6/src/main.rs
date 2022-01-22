use std::fs::read_to_string;

fn main() {
    let mut world = World {
        fish_items: read_to_string("./input.txt")
            .unwrap()
            .split(",")
            .map(|x| Fish {
                value: x.parse().unwrap(),
            })
            .collect(),
    };

    (0..80).for_each(|_| {
        world.simulate_day();
    });

    dbg!(world.fish_items.len());
}

struct Fish {
    value: u8,
}

impl Fish {
    fn simulate_day(&mut self) -> Option<Fish> {
        if self.value == 0 {
            self.value = 6;
            return Some(Fish { value: 8 });
        }

        self.value -= 1;
        return None;
    }
}

struct World {
    fish_items: Vec<Fish>,
}

impl World {
    fn simulate_day(&mut self) {
        let mut new_fishes: Vec<Fish> = self
            .fish_items
            .iter_mut()
            .filter_map(|f| {
                return f.simulate_day();
            })
            .collect();

        self.fish_items.append(&mut new_fishes);
    }
}
