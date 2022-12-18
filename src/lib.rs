use typeshare::typeshare;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct SimpleStruct {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
pub enum Command {
    List,
    Detail(u32),
}
