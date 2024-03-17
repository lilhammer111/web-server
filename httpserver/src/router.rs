use std::fmt::Write;

use http::httprequest::{HttpRequest, self};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
       match req.method {
           httprequest::Method::Get => match &req.resource {
               httprequest::Resource::Path(s) => {
                   
               }
           }
       } 
    }
}
