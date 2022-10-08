use anyhow::Result;
use hyper::{Body, Method, Request, Response, StatusCode};
use opencv::{core::Vector, prelude::*};
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::parameter::Parameters;

pub async fn handler(
    req: Request<Body>,
    img_path: Arc<String>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/image") => {
            let params_map: HashMap<String, String> = req
                .uri()
                .query()
                .map(|query| {
                    url::form_urlencoded::parse(query.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new);
            let params_result: Result<Parameters> = params_map.try_into();
            match params_result {
                Ok(params) => {
                    let img =
                        opencv::imgcodecs::imread(&img_path, opencv::imgcodecs::IMREAD_COLOR)?;
                    let mut processed_img = Mat::default();
                    img.convert_to(&mut processed_img, -1, params.contrast, params.brightness)?;
                    let mut source = Vector::new();
                    opencv::imgcodecs::imencode(
                        ".jpg",
                        &processed_img,
                        &mut source,
                        &Vector::new(),
                    )?;
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "image/jpeg")
                        .body(source.to_vec().into())
                        .unwrap())
                }
                Err(e) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(e.to_string().into())
                    .unwrap()),
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap()),
    }
}
