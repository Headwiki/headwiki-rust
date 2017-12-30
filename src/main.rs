extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/rust", get_root, "rust");
    router.post("/rust/test", post_test, "test");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_root(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>Rust Web Server</title>
        Rust Web Server
        <form action="/test" method="post">
            <input type="text" name="input"/>
            <button type="submit">Submit</button>
        </form>
        "#);
    Ok(response)
}

fn post_test(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_input = match form_data.get("input") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form data has no 'input' parameter\n"));
            return Ok(response);
        }
        Some(data) => data
    };

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("Input: {:?}\n", unparsed_input));
    Ok(response)
}
