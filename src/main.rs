use crate::lexical_analysis::LexicalAnalysis;

mod lexical_analysis;

fn main() {
    let json = "{ \"age\": 1, \"name\": \"Tom\", \"array\": [1, 2, 4, 3] }";
    let la = LexicalAnalysis::new(json);
    // println!("{json}");
    let result = la.extract();

    println!("{:?}", result.unwrap().1);
}
