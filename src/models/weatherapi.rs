use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherJSONLocation {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherJSONCurrentCondition {
    pub text: String,
    icon: String,
    code: i32
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherJSONCurrent {
    last_updated_epoch: i64,
    last_updated: String,
    pub temp_c: f32,
    temp_f: f32,
    is_day: i16,
    pub condition: WeatherJSONCurrentCondition,
    wind_mph: f32,
    pub wind_kph: f32,
    wind_degree: i16,
    pub wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: i16,
    cloud: i16,
    pub feelslike_c: f32,
    feelslike_f: f32,
    windchill_c: f32,
    windchill_f: f32,
    heatindex_c: f32,
    heatindex_f: f32,
    dewpoint_c: f32,
    dewpoint_f: f32,
    vis_km: f32,
    vis_miles: f32,
    pub uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherJSON {
    pub location: WeatherJSONLocation,
    pub current: WeatherJSONCurrent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationJson {
    pub location: WeatherJSONLocation,
}
