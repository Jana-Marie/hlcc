// hlcc-service  -  hlcc http api
// Copyright (C) 2022 Jana Marie Hemsing

use tide::{Request, StatusCode, Response};
use std::borrow::Cow;
use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut args = env::args();
    args.next();
    let socks: Vec<String> = args.collect();

    let mut app = tide::new();
    app.at("/").get(compute);
    app.listen(socks).await?;
    Ok(())
}

async fn compute(req: Request<()>) -> tide::Result {
    let mut input = None;

    for param in req.url().query_pairs() {
        if let (Cow::Borrowed("q"), value) = param {
            input = Some(value);
        }
    }

    let mut res = match input {
        None => return Ok(
            Response::builder(StatusCode::BadRequest)
            .content_type("text/plain")
            .body("Missing query parameter\n")
            .build()
        ),
        Some(input) =>
            match hlcc_parser::compute(&input) {
                None => return Ok(
                    Response::builder(StatusCode::BadRequest)
                    .content_type("text/plain")
                    .body("Parser Error\n")
                    .build()
                ),
                Some(res) => res
            }
    };

    res.push('\n');

    Ok(
        Response::builder(StatusCode::Ok)
        .content_type("text/plain;charset=UTF-8")
        .header("Cache-Control", "public, max-age=86400")
        .body(res)
        .build()
    )
}
