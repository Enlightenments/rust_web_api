use serde::{Serialize, Deserialize};
use hyper::{Response, Body, StatusCode, header};

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct JsonRes<T> { code: i8, msg: String, data: T }


impl<T:Sized + Serialize> JsonRes<T> {
    pub fn new(code: i8, msg: String, data: T) -> Result<Response<Body>, hyper::Error>{
        let res = JsonRes {
            code,
            msg,
            data
        };
        let string = serde_json::to_string(&res).unwrap();
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(string)).unwrap())
    }

}
