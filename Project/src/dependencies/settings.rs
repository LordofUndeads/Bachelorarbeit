mod settings {
    //Option enums for the Menu Page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    EarClipping,
    DelaunyTriangulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Random,
    BiggestArea,
    BiggestMinAngle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Option {
    EdgeSwapping,

}
//Helping implementations to make Strings from enum type and make a list of all enum types for Algorithm, Heuristic and Option
impl Algorithm {
    fn all() -> [Algorithm; 2] {
        [
            Algorithm::EarClipping,
            Algorithm::DelaunyTriangulation,
        ]
    }
}

impl From<Algorithm> for String {
    fn from(algorithm: Algorithm) -> String {
        String::from(match algorithm {
            Algorithm::EarClipping => "Ear Clipping",
            Algorithm::DelaunyTriangulation => "Delauny Triangulation",
        })
    }
}

impl Heuristic {
    fn all() -> [Heuristic; 3] {
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

impl Option {
    fn all() -> [Option; 1] {
        [
            Option::EdgeSwapping, 
        ]
    }
}

impl From<Option> for String {
    fn from(option: Option) -> String {
        String::from(match option {
            Option::EdgeSwapping => "Edge Swapping",
        })
    }
}}