use super::HandlerResult;

use iron::mime::Mime;
use iron::mime::SubLevel;
use iron::mime::TopLevel;
use iron::status;
use iron::IronResult;
use iron::Response;
use serde::Serialize;
use serde_json;

pub fn handle_empty<Res, F>(callback: F) -> IronResult<Response>
where
    Res: Serialize,
    F: FnOnce() -> HandlerResult<Res>,
{
    let response = match callback() {
        Ok(response) => ErrorResponse::success(response),
        Err(err) => ErrorResponse::error(&format!("{}", err)),
    };

    struct_to_response(&response)
}

fn struct_to_response<Res>(value: &Res) -> IronResult<Response>
where
    Res: Serialize,
{
    match serde_json::to_string(value) {
        Ok(body) => {
            let content_type = Mime(TopLevel::Application, SubLevel::Json, vec![]);

            Ok(Response::with((status::Ok, content_type, body)))
        }
        Err(_) => Ok(Response::with((status::InternalServerError,))),
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ErrorResponse<T>
where
    T: Serialize,
{
    fn success(result: T) -> ErrorResponse<T> {
        ErrorResponse {
            success: true,
            result: Some(result),
            message: None,
        }
    }

    fn error(message: &str) -> ErrorResponse<T> {
        ErrorResponse {
            success: false,
            result: None,
            message: Some(message.into()),
        }
    }
}
