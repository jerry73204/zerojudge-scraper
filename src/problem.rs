use markdown::Block;

#[derive(Debug, Clone)]
pub struct Problem {
    pub title: String,
    pub content: Vec<Block>,
    pub input_desc: Vec<Block>,
    pub output_desc: Vec<Block>,
    pub hint: String,
    pub samples: Vec<Sample>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: String,
    pub output: String,
}
