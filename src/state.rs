use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct State {
    name: String,
    population: u32,
    seats: u32,
    priority: f64,
}

impl State {
    pub fn new(name: String, population: u32) -> State {
        State {
            name,
            population,
            seats: 1,
            priority: population as f64 / 2f64.sqrt(),
        }
    }

    pub fn add_seat(&mut self) {
        self.seats = self.seats + 1;
        self.priority = self.population as f64 / ((self.seats * (self.seats + 1)) as f64).sqrt();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_population(&self) -> u32 {
        self.population
    }

    pub fn get_seats(&self) -> u32 {
        self.seats
    }

    pub fn get_people_per_seat(&self) -> f64 {
        self.population as f64 / self.seats as f64
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.priority - other.priority {
            x if x > 0.0 => Some(Ordering::Greater),
            x if x < 0.0 => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority - other.priority {
            x if x > 0.0 => Ordering::Greater,
            x if x < 0.0 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}