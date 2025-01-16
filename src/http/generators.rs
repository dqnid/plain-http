use super::*;

// pub fn generate_response(petition: HttpRequest) -> ProcessedResponse {}

fn format_response_headers(headers: Headers, content_len: usize, content_type: &str) -> String {
    let mut response_headers = String::new();
    response_headers.push_str(format!("Content-Length: {}", content_len).as_str());
    response_headers.push_str(format!("\nContent-Type: {}", content_type).as_str());
    for (key, value) in headers {
        response_headers.push_str(format!("\n{key}: {value}").as_str());
    }
    return response_headers;
}

pub fn format_response(raw_response: HttpAppRouteResponse) -> ProcessedResponse {
    ProcessedResponse {
        data: format!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
            raw_response.status,
            format_response_headers(
                raw_response.headers,
                raw_response.body.len(),
                raw_response.content_type
            ),
            raw_response.body
        ),
        status: raw_response.status,
    }
}
