#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Random,
    BiggestArea,
    BiggestMinAngle,
}

impl Heuristic {
    pub fn all() -> [Heuristic; 3] {
        [
            Heuristic::Random,
            Heuristic::BiggestArea,
            Heuristic::BiggestMinAngle,
        ]
    }
}

impl From<Heuristic> for String {
    fn from(heuristic: Heuristic) -> String {
        String::from(match heuristic {
            Heuristic::Random => "Random",
            Heuristic::BiggestArea => "Biggest Area",
            Heuristic::BiggestMinAngle => "Biggest Minimal Angle",
        })
    }
}