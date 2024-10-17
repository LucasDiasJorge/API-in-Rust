use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub age: u8,
}
