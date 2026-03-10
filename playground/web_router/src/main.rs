#[derive(Debug)]
enum HttpMethod {
    Get,
    Post(String),
    Delete,
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    url: String,
    handled: bool,
}

trait Logger {
    fn log_info(&self);
}

impl Logger for HttpRequest {
    fn log_info(&self) {
        println!("[LOG] Incoming request to: {}", self.url);
    }
}

impl HttpRequest {
    fn new(method: HttpMethod, url: String, handled: bool) -> HttpRequest {
        HttpRequest {
            method,
            url,
            handled,
        }
    }

    fn mark_as_handled(&mut self) {
        self.handled = true;
    }

    fn route(&self) {
        match &self.method {
            HttpMethod::Get => {
                println!("Routing GET request for {}", self.url);
            }
            HttpMethod::Post(body) => {
                println!("Routing POST request. Storing data: {}", body);
            }
            HttpMethod::Delete => {
                println!("WARNING: Deleting resource at {}", self.url);
            }
        }
    }

}
fn main() {
    let mut my_request = HttpRequest::new(HttpMethod::Get, String::from("/api/login"), false);
    println!("{}", my_request.handled);
    my_request.mark_as_handled();
    println!("{}", my_request.handled);
    my_request.route();
}
