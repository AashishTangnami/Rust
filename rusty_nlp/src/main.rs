mod basic_nlp;
use basic_nlp::main_code;
// use ndarray::prelude::*;

use rust_bert::piplelines::question_answering::{QaInput, QuestionAnsweringModel};
use rust_bert::pipelines::summarization::SummarizationModel;


fn main() {

    main_code();

    let a: Vec<u64> = [0 ,30 ,45 , 60 ,90].to_vec();
    println!("a: {:#?}", a);

    let num1: i32 = 10;
    let num2: i32 = 20;

    let result: i32 = num1 + num2;
    let result2: i32 = num1 * num2 * 2;
    let result3: i32 = result + result2;

    println!("The value of result is: {}", result3);

    println!("--------------------------------");

    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let question = String::from("Where does Amy live ?");
    let context = String::from("Amy lives in Amsterdam");
    let answers = qa_model.predict(&vec![QaInput { question, context }], 1, 32);


    let mut model = SummarizationModel::new(Default::default())?;
    let input = ["In findings published Tuesday in Cornell University's arXiv by a team of scientists
from the University of Montreal and a separate report published Wednesday in Nature Astronomy by a team
from University College London (UCL), the presence of water vapour was confirmed in the atmosphere of K2-18b,
a planet circling a star in the constellation Leo. This is the first such discovery in a planet in its star's
habitable zone — not too hot and not too cold for liquid water to exist. The Montreal team, led by Björn Benneke,
used data from the NASA's Hubble telescope to assess changes in the light coming from K2-18b's star as the planet
passed between it and Earth. They found that certain wavelengths of light, which are usually absorbed by water,
weakened when the planet was in the way, indicating not only does K2-18b have an atmosphere, but the atmosphere
contains water in vapour form. The team from UCL then analyzed the Montreal team's data using their own software
and confirmed their conclusion. This was not the first time scientists have found signs of water on an exoplanet,
but previous discoveries were made on planets with high temperatures or other pronounced differences from Earth.
\"This is the first potentially habitable planet where the temperature is right and where we now know there is water,\"
said UCL astronomer Angelos Tsiaras. \"It's the best candidate for habitability right now.\" \"It's a good sign\",
said Ryan Cloutier of the Harvard–Smithsonian Center for Astrophysics, who was not one of either study's authors.
\"Overall,\" he continued, \"the presence of water in its atmosphere certainly improves the prospect of K2-18b being
a potentially habitable planet, but further observations will be required to say for sure. \"
K2-18b was first identified in 2015 by the Kepler space telescope. It is about 110 light-years from Earth and larger
but less dense. Its star, a red dwarf, is cooler than the Sun, but the planet's orbit is much closer, such that a year
on K2-18b lasts 33 Earth days. According to The Guardian, astronomers were optimistic that NASA's James Webb space
telescope — scheduled for launch in 2021 — and the European Space Agency's 2028 ARIEL program, could reveal more
about exoplanets like K2-18b."];

let output = model.summarize(&input);

let sentiment_classifier = SentimentModel::new(Default::default())?;
                                                        
let input = [
    "Probably my all-time favorite movie, a story of selflessness, sacrifice and dedication to a noble cause, but it's not preachy or boring.",
    "This film tried to be too many things all at once: stinging political satire, Hollywood blockbuster, sappy romantic comedy, family values promo...",
    "If you like original gut wrenching laughter you will like this movie. If you are young or old then you will love this movie, hell even my mom liked it.",
];

let output = sentiment_classifier.predict(&input);

}

