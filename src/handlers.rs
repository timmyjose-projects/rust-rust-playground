use std::fs::File;
use std::io::{Read, BufReader};
use std::error::Error;

use iron::prelude::*;
use iron::{mime, status};
use urlencoded::UrlEncodedBody;

use super::configuration;

const BASE_PATH: &'static str = "resources/html-templates/";

/// get 
pub fn execute_handler(_req: &mut Request) -> IronResult<Response> {

    if let Ok(html) = get_static_file("execute.html") {
        let mut response = Response::new();
        response.set_mut(status::Ok);

        let content_type: mime::Mime = "text/html".parse().unwrap();
        response.set_mut(content_type);
        response.set_mut(html);

        return  Ok(response);
    }

    Ok(Response::with((status::NotFound, "page not found")))
}

fn get_static_file(file: &str) -> Result<String, Box<Error>> {
    let mut reader = BufReader::new(File::open(String::from(BASE_PATH) + file)?);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

/// post
pub fn results_handler(req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_params = match req.get_ref::<UrlEncodedBody>() {
        Err(e) =>  {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error while parsing form data: {:#?}\n",
                                     e));
            return Ok(response);
        },
        Ok(map) => map,
    };

    let channel = match form_params.get("channel") {
        None  => return Ok(get_error_response("channel")),
        Some(channel) => &channel[0],
    };

    let mode = match form_params.get("mode") {
        None => return Ok(get_error_response("mode")),
        Some(mode) => &mode[0],
    };

    let edition = match form_params.get("edition") {
        None => return Ok(get_error_response("edition")),
        Some(edition) => &edition[0],
    };

    let operation = match form_params.get("operation") {
        None => return Ok(get_error_response("operation")),
        Some(operation) => &operation[0],
    };

    let code = match form_params.get("code") {
        None => return Ok(get_error_response("code")),
        Some(code) => &code[0],
    };

    let tests = false;
    let backtrace = false;
    let crate_type = if operation == "run" { "bin" } else { "lib" };

    let exec_result = get_execution_result(channel,
                                           mode,
                                           edition,
                                           crate_type,
                                           tests,
                                           code,
                                           backtrace);

    let resp_type: mime::Mime = "application/json".parse().unwrap();
    response.set_mut(exec_result.to_string());
    response.set_mut(status::Ok);
    response.set_mut(resp_type);

    Ok(response)
}


fn get_error_response(param: &str) -> Response {
    let mut response = Response::new();
    response.set_mut(format!("Error: could not retrieve form {}", param));
    response
}

fn get_execution_result(channel: &str, 
                        mode: &str, 
                        edition: &str,
                        crate_type: &str,
                        tests: bool,
                        code: &str,
                        backtrace: bool) -> serde_json::Value
{
    let settings = configuration::Settings::new().unwrap();
    let endpoint = settings.endpoint().url();

    reqwest::Client::new()
        .post(endpoint)
        .json(
            &json!({
                "channel": channel,
                "mode": mode,
                "edition": edition,
                "crateType": crate_type,
                "tests": tests,
                "code": code,
                "backtrace": backtrace
            })
        )
        .send().unwrap()
        .json().unwrap()
}
