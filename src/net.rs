pub mod http;
use self::http::HttpResponse;
pub struct HttpClient {

}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {

        }
    }
    pub async fn get(&self, uri: &str) -> &str {
        "result"
    }
    pub async fn send(&self, uri: &str) -> HttpResponse {
        HttpResponse{}
    }
}
#[derive(Default)]
pub struct Fetch {

}
impl Fetch {
    pub fn new() -> Self {
        Fetch::default()
    }
    pub async fn get(&self) -> HttpResponse {
        HttpResponse::default()
    }
    pub async fn post(&self) -> HttpResponse {
        HttpResponse::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Fetch;
    #[test]
    fn get() {
        async_std::task::block_on(async {
            let fetch = Fetch::new();
            let rs = fetch.get().await;
            assert_eq!(1, 1);
         }); 
    }
}

