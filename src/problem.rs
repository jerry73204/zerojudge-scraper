use markdown::Block;

#[derive(Debug, Clone)]
pub struct Problem {
    pub title: String,
    pub content: Vec<Block>,
    pub input: String,
    pub output: String,
    pub hint: String,
    pub samples: Vec<Sample>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: String,
    pub output: String,
}
