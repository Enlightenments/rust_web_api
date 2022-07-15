
use crate::app::lib::json_res::JsonRes;
use crate::app::lib::param;
use hyper::{Request, Body, Response};

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //判断 params中是否有name   param.get() 必须抛出错误  建议预先处理错误
    let is_name = params.contains_key("name");
    info!("{:?}",is_name);
    let is_pwd = params.contains_key("pwd");
    info!("{:?}",is_pwd);
    if is_name && is_pwd  {
        let name = params.get("name").unwrap();
        let pwd = params.get("pwd").unwrap();
        JsonRes::new(0,"success".to_string(),(name,pwd))
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }

}