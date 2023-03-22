fn main() {
    let url = r"https://api.weather.gov/gridpoints/LZK/77,72/forecast/hourly";
    let result = weather::Client::new().load(url);
    let style = weather::style::elegant();

    match result {
        Ok(doc) => println!("{}", weather::hour::render(&doc, style)),
        Err(e) => println!("{}", e.to_string()),
    }
}
