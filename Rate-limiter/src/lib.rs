wit_bindgen::generate!({generate_all});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

use wasmcloud::bus::lattice;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let rate_limit = 30;
        let path_with_query = request
            .path_with_query()
            .expect("failed to get path with query");

        let mut object_name = path_with_query.clone();

        let mut link_name = "default";

        if let Some((path, query)) = path_with_query.split_once('?') {
            object_name = path.to_string();

            let query_params = query
                .split('&')
                .filter_map(|v| v.split_once('='))
                .collect::<Vec<(&str, &str)>>();

            if let Some((_, configured_link_name)) = query_params
                .iter()
                .find(|(k, _v)| k.to_lowercase() == "link_name")
            {
                link_name = configured_link_name;
            }
        }

        lattice::set_link_name(
            link_name,
            vec![
                wasmcloud::bus::lattice::CallTargetInterface::new("wasi", "keyvalue", "store"),
                wasmcloud::bus::lattice::CallTargetInterface::new("wasi", "keyvalue", "atomics"),
            ],
        );

        let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
        let count = wasi::keyvalue::atomics::increment(&bucket, &object_name, 1)
            .expect("failed to increment count");

        let response = OutgoingResponse::new(Fields::new());
        let response_body = response.body().unwrap();
        if count > rate_limit {
            response.set_status_code(500).unwrap();
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(format!("Rate Limit exceeded\n").as_bytes())
            .unwrap();
        }else {
            response.set_status_code(200).unwrap();
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(format!("Counter {object_name}: {count}\n").as_bytes())
            .unwrap();
        }
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        ResponseOutparam::set(response_out, Ok(response));
    }
}

export!(HttpServer);
