use serde::Deserialize;
use ureq;

#[derive(Deserialize)]
struct Joke {
    setup: String,
    punchline: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get("https://official-joke-api.appspot.com/random_joke")
        .call()?
        .into_json::<Joke>()?;

    println!("Setup: {}", response.setup);
    println!("");
    println!("Punchline: {}", response.punchline);

    Ok(())
}
