## Simple example

Example with closure

``
mod rustex;
use rustex::Response;

fn main() {
    let bind_address = "127.0.0.1:8080";

    let mut app = rustex::App::new(bind_address);

    app.get("/hello", || -> Response {
        let text = String::from("I work");
        Response {
            status: 200,
            data: text,
        }
    });

    app.run_server();

}
``


Example with named function:


``
mod rustex;
use rustex::Response;

fn main() {
    let bind_address = "127.0.0.1:8080";

    let mut app = rustex::App::new(bind_address);

    app.get("/hello", test_function);

    app.run_server();
}

fn test_function() -> Response {
    let text = String::from("I work");
    Response {
        status: 200,
        data: text,
    }
}
``