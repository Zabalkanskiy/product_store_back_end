

use lazy_static::lazy_static;
use futures::{future, Future};
//futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn_ok;
use regex::Regex;


const APP_SECRET : & str = "myappsecret";
const USERNAME : & str = "admin";
const PASSWORD : & str = "secret";


lazy_static! {
    static ref LOGIN_PATH: Regex = Regex::new("^/login/?$").unwrap();
    static ref PRODUCT_PATH: Regex = Regex::new("^/products/?$").unwrap();
    static ref ORDER_PATH: Regex = Regex::new("^/orders/?$").unwrap();
}

fn microservice_handler(req: Request<Body>)
    -> impl Future<Item=Response<Body>, Error=Error>
{
   let response =  {
        let method = req.method();
        let path = req.uri().path();
        
        if LOGIN_PATH.is_match(path){
            if method == &Method::POST {
                //псевдокод чтения тела запроса нужно сделать через serde??
               if req.body.name == USERNAME && req.body.password == PASSWORD {
                   //код  js который необходимо реализовать на расте 
                   //библиотека const jwt = require("jsonwebtoken");
                     //  let token = jwt.sign({ data: USERNAME, expiresIn: "1h" }, APP_SECRET);
                    //res.json({ success: true, token: token });
               }
            } else{
                response_with_code(StatusCode::METHOD_NOT_ALLOWED)
            }
        } else if PRODUCT_PATH.is_match(path){
            unimplemented!()
        }else if ORDER_PATH.is_match(path){
            unimplemented!() 
        }else  {
            response_with_code(StatusCode::NOT_FOUND)
        }
   };
   response
}

fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}


fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(move | req| service_fn_ok( microservice_handler(req)));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

