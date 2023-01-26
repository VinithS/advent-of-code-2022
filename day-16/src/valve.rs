use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Valve {
    pub status: bool,
    pub rate: u32,
    pub child: Vec<String>,
}

impl Valve {
    pub fn new(rate: u32, child: Vec<&str>) -> Valve {
        Valve {
            status: false,
            rate,
            child: child.into_iter().map(String::from).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Volcano {
    pub valves: HashMap<String, Valve>,
    pub timer: i32,
    pub current: String,
    pub pressure: u32,
}

impl Volcano {
    pub fn move_to(&mut self, s: String) {
        self.timer -= 1;
        self.current = s;
    }

    pub fn open(&mut self) {
        let valve: &mut Valve = self.valves.get_mut(&self.current).unwrap();
        valve.status = true;

        self.pressure = valve.rate;
    }
}
