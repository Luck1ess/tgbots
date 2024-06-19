use std::env;
use teloxide::{
    dispatching::dialogue::{
        serializer::Json,
        ErasedStorage,
        SqliteStorage,
        Storage,
        // UpdateHandler,
    },
    prelude::*,
    types::ParseMode,
    utils::command::BotCommands,
};

mod models;
use models::weatherapi::{WeatherJSON, WeatherJSONLocation, LocationJson};

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;


const URL: &str = "http://api.weatherapi.com/v1/";
fn get_url_token() -> String{
    return env::var("URL_TOKEN").unwrap();
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    ReceiveLocation,
    GetWeather(WeatherJSONLocation),
}

#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    ShowLocation,
    ResetLocation,
    GetWeather,
    Help,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    let storage: MyStorage = SqliteStorage::open("db.sqlite", Json).await.unwrap().erase();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, ErasedStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveLocation].endpoint(receive_location))
            .branch(
                dptree::case![State::GetWeather(location)]
                    .branch(dptree::entry().filter_command::<Command>().endpoint(get_weather))
                    .branch(dptree::endpoint(invalid_command)),
            )   
    )
    .dependencies(dptree::deps![storage])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! Send me your location").await?;
    dialogue.update(State::ReceiveLocation).await?;
    Ok(())
}

async fn receive_location(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult{
    match msg.location(){
        Some(location) => {
            let client = reqwest::Client::new();
            let (lat, lon) = (location.latitude, location.longitude);
            let resp = client.get(format!("{}{}", URL, "timezone.json"))
                .query(&[("key", get_url_token().as_str()),  ("q", &format!("{},{}", lat, lon))])
                .send()
                .await?
                .json::<LocationJson>()
                .await?;
            let location = resp.location;
            let message_rows = vec!(
                format!("<b> Hey Hey Hey! </b>", ),
                format!("<b> We detected your location! </b>", ),
                format!("<b> Here you are! </b>", ),
                format!("<i> City: {} </i>", location.name,),
                format!("<i> Region: {} </i>", location.region,),
                format!("<i> Country: {} </i>", location.country,),
                format!("<i> Latitude: {}, Longitude: {} </i>", location.lat, location.lon,),
                format!("<i> Your timezone: {} </i>", location.tz_id,),
            );
            let message = message_rows.join("\n");
            bot.parse_mode(ParseMode::Html).send_message(msg.chat.id, message).await?;
            dialogue.update(State::GetWeather(location)).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Oops, lets start again").await?;
            dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}

async fn get_weather(
    bot: Bot, 
    location: WeatherJSONLocation,
    dialogue: MyDialogue,
    msg: Message,
    cmd: Command,
) -> HandlerResult {
    match cmd {
        Command::GetWeather => {
            let client = reqwest::Client::new();
            let (lat, lon) = (location.lat, location.lon);
            let resp = client.get(format!("{}{}", URL, "current.json"))
                .query(&[("key".to_owned(), get_url_token()), ("q".to_owned(), format!("{},{}", lat, lon))])
                .send()
                .await?
                .json::<WeatherJSON>()
                .await?;
            let answer_rows = vec![
                format!("<b> Hi friend fom {}, {}, {}!!! </b>", location.name, location.region, location.country),
                format!("<i> Current temperature: {}° Feels like {}° </i>", resp.current.temp_c, resp.current.feelslike_c,),
                format!("<i> Seems to be {} </i>", resp.current.condition.text,),
                format!("<i> Wind in {} direction {}km/h </i>", resp.current.wind_dir, resp.current.wind_kph,),
                format!("<i> UV index {}, be careful</i>", resp.current.uv,),
                format!("<b> UV index description! </b>"),
                format!("<i> 0 to 2 - You can safely enjoy being outside!</i>"),
                format!("<i> 3 to 7 - Seek shade during midday hours! Slip on a shirt, slop on sunscreen and slap on hat! </i>"),
                format!("<i> 8 and above - Avoid being outside during midday hours! Make sure you seek shade! Shirt, sunscreen and hat are a must! </i>"),
            ];
            let answer = answer_rows.join("\n");
            bot.parse_mode(ParseMode::Html).send_message(msg.chat.id, answer).await?;
        }
        Command::ShowLocation => {
            let message_rows = vec!(
                    format!("<b> Here you are! </b>", ),
                    format!("<i> City: {} </i>", location.name,),
                    format!("<i> Region: {} </i>", location.region,),
                    format!("<i> Country: {} </i>", location.country,),
                    format!("<i> Latitude: {}, Longitude: {} </i>", location.lat, location.lon,),
                    format!("<i> Your timezone: {} </i>", location.tz_id),
            );
            let message = message_rows.join("\n");
            bot.parse_mode(ParseMode::Html).send_message(msg.chat.id, message).await?;
        }
        Command::ResetLocation => {
            bot.send_message(msg.chat.id, "Send me your new location please").await?;
            dialogue.update(State::ReceiveLocation).await?;
            
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
    }
    Ok(())
}

async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
