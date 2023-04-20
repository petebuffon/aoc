use std::collections::HashMap;

fn main() {
    let mut tunnels = HashMap::new();
    tunnels.insert("AA", Valve::new(0, Vec::from(["DD".to_string(), "II".to_string(), "BB".to_string()])));
    tunnels.insert("BB", Valve::new(13, Vec::from(["CC".to_string(), "AA".to_string()])));
    tunnels.insert("CC", Valve::new(2, Vec::from(["DD".to_string(), "BB".to_string()])));

    let trip = Trip::new(0, 0, Vec![]);
    let valve = tunnels.get("AA").unwrap();
    trip.traverse(valve);
}   

struct Valve {
    rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(rate: u32, tunnels: Vec<String>) -> Self {
        Self {
            rate: rate,
            tunnels: tunnels
        }
    }
}

struct Trip {
    minute: u32,
    pressure: u32,
    open_valves: Vec<String>,
}

impl Trip {
    fn new(minute: u32, pressure: u32, open_valves: Vec<String>) -> Self {
        Self {
            minute: minute,
            pressure: pressure,
            open_valves: open_valves,
        }
    }

    fn traverse(&self, valve: Valve) {
        for valve in valve.tunnels {
            
        }
    }
}