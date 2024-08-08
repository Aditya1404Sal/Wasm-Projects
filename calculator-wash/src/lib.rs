wit_bindgen::generate!({
    generate_all
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

struct Calculator;

impl Guest for Calculator {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap();
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() < 3 {
            let response = OutgoingResponse::new(Fields::new());
            response.set_status_code(400).unwrap();
            let response_body = response.body().unwrap();
            response_body
                .write()
                .unwrap()
                .blocking_write_and_flush(b"Invalid request format. Use /calculate/{operation}/{num1}/{num2}").unwrap();
            OutgoingBody::finish(response_body, None).expect("failed to finish response body");
            ResponseOutparam::set(response_out, Ok(response));
            return;
        }
        let operation = parts[2];
        let num1: f64 = parts[3].parse().unwrap_or(0.0);
        let num2: f64 = parts[4].parse().unwrap_or(0.0);

        let result = match operation {
            "add" => num1 + num2,
            "subtract" => num1 - num2,
            "multiply" => num1 * num2,
            "divide" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    let response = OutgoingResponse::new(Fields::new());
                    response.set_status_code(400).unwrap();
                    let response_body = response.body().unwrap();
                    response_body
                        .write()
                        .unwrap()
                        .blocking_write_and_flush(b"Division by zero error.").unwrap();
                    OutgoingBody::finish(response_body, None).expect("failed to finish response body");
                    ResponseOutparam::set(response_out, Ok(response));
                    return;
                }
            }
            _ => {
                let response = OutgoingResponse::new(Fields::new());
                response.set_status_code(400).unwrap();
                let response_body = response.body().unwrap();
                response_body
                    .write()
                    .unwrap()
                    .blocking_write_and_flush(b"No Operation Specified.").unwrap();
                OutgoingBody::finish(response_body, None).expect("failed to finish response body");
                ResponseOutparam::set(response_out, Ok(response));
                return;
            }
        };

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        let result_string = format!("Result: {}", result);
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(result_string.as_bytes()).unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        ResponseOutparam::set(response_out, Ok(response));
    }
}

export!(Calculator);
