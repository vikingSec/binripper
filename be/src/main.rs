
use std::io;
 
use poem::{
    listener::TcpListener, 
    Result, Route, Server, web::Path, EndpointExt,
     IntoResponse, Error
};
use poem_openapi::{
    
    payload::{Json, PlainText},
   Object, OpenApi, OpenApiService, types::ParseError,

};
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
struct Api;

mod x86_64_parser;
use crate::x86_64_parser::x86_64_parser::parser;

#[derive(Object)]
struct BasicObj {
    code: Option<i32>,
    message: String
}

#[OpenApi]
impl Api {

    #[oai(path="/test", method="get")]
    pub async fn testfunc(&self) -> Json<BasicObj>{
        Json(BasicObj {
            code: Some(200),
            message: "Hello!".to_string()
        })
    }

    // #[oai(path="/parse", method="post")]
    // pub async fn parsefunc(&self, ?) -> ? {
    //     //TODO: Implement a basic function that takes a binary file as input and parses it using whatever parses we have available.
    //     // return a JSON file with basic meta information an a link to disassemble it
    // }
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let allendpoints = (Api);
    let api_service =
        OpenApiService::new(allendpoints, "Hello World", "1.0").server("http://127.0.0.1:3111/api/");

    let ui = api_service.swagger_ui();
    let router = Route::new().nest("/api", api_service).nest("/", ui);

    
    Server::new(TcpListener::bind("127.0.0.1:3111"))
        .run(router)
        .await;
    println!("Reached");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::x86_64_parser::*;

    #[test]
    fn basictest(){
        assert!(1==1);
    }
    #[test]
    fn x86_64ImportTest(){
        let mut p = x86_64_parser::parser::new();
        assert!(p.checkparse() == "works");
        let loaded = p.loadfile(format!("{}{}", env!("CARGO_MANIFEST_DIR").to_string(), "/testfiles/binripper.exe".to_string()).to_string());
        assert!(loaded.is_ok());
        let badload = p.loadfile("doesnotexist".to_string());
        assert!(!badload.is_ok());
    }

    #[test]
    fn x86_64MagicByteTest(){
        let mut p = x86_64_parser::parser::new();
        assert!(p.checkparse() == "works");
        let loaded = p.loadfile(format!("{}{}", env!("CARGO_MANIFEST_DIR").to_string(), "/testfiles/binripper.exe".to_string()).to_string());
        assert!(loaded.is_ok());
        let magic = p.magicbytes();
        assert!(format!("{:x}{:x}", magic[0], magic[1]) == "4d5a");
    }
}