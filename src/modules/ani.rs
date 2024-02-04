use crate::AniData;

pub fn get_ani_schedule(week_of_the_day: i32) -> Result<Vec<String>, reqwest::Error> {
    // parse web
    let url = format!("https://api.anissia.net/anime/schedule/{}", week_of_the_day);

    // Add user agent
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .build()?;

    let resp = client.get(url)
        .send()?
        .json::<AniData>()?;

    let mut result = Vec::new();
    for item in resp.data {
        result.push(format!("{}", item.subject));
    }

    Ok(result)
}