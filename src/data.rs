use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SensorValues {
    pub value_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct SensorDataValues {
    #[serde(rename = "sensordatavalues")]
    pub values: Vec<SensorValues>,
}
