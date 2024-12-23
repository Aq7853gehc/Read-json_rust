use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String
}

#[derive(Deserialize, Serialize)]
struct Artical {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json: &str = r#"
    {
            "article":"how to read your json data",
            "author":"Aquib",
            "paragraph":[
                {
                    "name" : "Start of paragraph"
                },
                {
                    "name" : "Body of the paragraph"
                },
                {
                    "name":"Ending of the paragraph"
                }
            ]

    }"#;

    let parased:Artical = read_json_data(json); 
    println!("\n\n The name of the first paragraph is: {}",parased.paragraph[0].name);
}

fn read_json_data(raw_json : &str)->Artical {

    let parsed:Artical = serde_json::from_str(raw_json).unwrap();

    return parsed

}
