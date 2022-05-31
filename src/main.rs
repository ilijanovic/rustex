mod rustex;
use rustex::Response;
use serde_json::json;

fn main() {
    let bind_address = "127.0.0.1:8080";

    let mut app = rustex::App::new(bind_address);

    fn login_handler() -> Response {
        let response_message = serde_json:: json!({
            "test": "nice"
        });
        return Response {
            status: 200,
            data: response_message.to_string(),
        };
    }

    app.get("/login", login_handler);

    app.run_server();
}
