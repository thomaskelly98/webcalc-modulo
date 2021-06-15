#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate serde;

use rocket_contrib::json::JsonValue;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;


// Allow CORS
pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Content-Type", "application/json"));
    }
}

fn modulo(x_int: i32,  y_int: i32) -> i32 {
    let ans = x_int % y_int;

    return ans
}

fn is_string_numeric(str: &String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

#[get("/?<x>&<y>")]
fn index(x: String, y: String) -> JsonValue {
    if x.trim().is_empty() || y.trim().is_empty() {
        let response = json!({ "error": "One or both parameters are missing" });
        return response
    }

    if !is_string_numeric(&x) {
        let response = json!({ "error": "Param x is not an integer" });
        return response
    }
    if !is_string_numeric(&y) {
        let response = json!({ "error": "Param y is not an integer" });
        return response
    }
    
    let x_int = x.parse::<i32>().unwrap();
    let y_int = y.parse::<i32>().unwrap();
    
    if y_int > 0 || y_int < 0 {
        let ans = modulo(x_int, y_int);

        let response = json!({ "error": "", "x": x_int, "y": y_int, "answer": ans });
        return response
    }
    else if y_int == 0 {

        let response = json!({ "error": "Cannot divide by 0" });
        return response
    }

    let response = json!({"error": "One or both parameters are invalid"});
    return response
}

#[get("/")]
fn index_no_params() -> JsonValue {
    let response = json!({"error": "One or both parameters are missing"});
    return response
}

fn rocket() -> rocket::Rocket {
    // Attach CORS header to rocket
    rocket::ignite().attach(CORS()).mount("/", routes![index, index_no_params])
}

fn main() {
    rocket().launch();
}


// Tests
#[cfg(test)]
mod test {
    use super::rocket;
    use super::modulo;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn local_modulo_func() {
        let x = 12;
        let y = 5;
        let ans = modulo(x, y);
        assert_eq!(ans, 2);
    }

    #[test]
    fn index_no_params() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"One or both parameters are missing\"}").into()));
    }

    #[test]
    fn index_no_x() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?y=5").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"One or both parameters are missing\"}").into()));
    }

    #[test]
    fn index_x_no_val() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=&y=5").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"One or both parameters are missing\"}").into()));
    }

    #[test]
    fn index_no_y() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=12").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"One or both parameters are missing\"}").into()));
    }

    #[test]
    fn index_y_no_val() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=12&y=").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"One or both parameters are missing\"}").into()));
    }

    #[test]
    fn index_y_is_0() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=12&y=0").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"Cannot divide by 0\"}").into()));
    }

    #[test]
    fn index_x_is_not_int() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=the&y=45").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"Param x is not an integer\"}").into()));
    }

    #[test]
    fn index_y_is__not_int() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=45&y=the").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"error\":\"Param y is not an integer\"}").into()));
    }


    #[test]
    fn index_valid_params() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/?x=12&y=5").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(("{\"answer\":2,\"error\":\"\",\"x\":12,\"y\":5}").into()));
    }
}
