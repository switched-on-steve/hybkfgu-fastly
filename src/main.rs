use fastly::{Error, Request, Response};
use serde_json::{json, Value};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let mut response = req.send("origin_0")?;

    let mut new_json: Value = response.take_body_json()?;
    new_json["new_field"] = json!("content");

    Ok(response.with_body_json(&new_json)?)
}
