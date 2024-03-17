use http::httprequest::HttpRequest;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpRequest {
         
    }
}
