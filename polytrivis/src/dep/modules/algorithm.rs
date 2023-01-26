//Option enums for the Menu Page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    EarClipping,
    DelaunyTriangulation,
}

impl<'a> Algorithm {
    pub fn all() -> [Algorithm; 2] {
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
            Algorithm::DelaunyTriangulation => "Delauny Triangulation (tbi)",
        })
    }
}