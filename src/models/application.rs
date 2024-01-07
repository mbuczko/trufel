use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Application {
    #[serde(rename = "application_id")]
    id: Uuid,
    name: String,
    description: Option<String>,
    url: String,
    icon: String,
    visible: bool,
    order: u16
}
