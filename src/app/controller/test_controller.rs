
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};

pub async fn main(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    JsonRes::new(0,"success".to_string(),"")
}