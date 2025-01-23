# Plain HTTP

This is a simple Rust HTTP library. It provides an `HttpApp` struct that acts as the main structure of the application.

### Example usage

```rust
use std::collections::HashMap;

use plain_http::*;

fn get_main(_request: HttpRequest) -> HttpAppRouteResponse {
    if let Some(body) = _request.body {
        println!("Body {}", body);
    }

    HttpAppRouteResponse {
        body: "hello".to_string(),
        content_type: "text",
        status: 200,
        headers: HashMap::new(),
    }
}

fn get_test(_request: HttpRequest) -> HttpAppRouteResponse {
    HttpAppRouteResponse::from_url("./src/assets/index.html")
}

fn main() {
    println!("Hello, world!");
    let mut app = HttpApp {
        config: HttpAppConfig {
            port: 3000,
            max_request_size_bytes: 10000,
            ..Default::default()
        },
        routes: vec![],
        default_headers: HashMap::new(),
    };

    app.add_route(HttpAppRoute {
        route: "/".to_string(),
        action: Box::new(get_main),
    });

    app.add_route(HttpAppRoute {
        route: "/test".to_string(),
        action: Box::new(get_test),
    });

    app.start();
}
```
