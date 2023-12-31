#![feature(async_fn_in_trait)]

use tonic::{transport::Server, Request, Response, Status};

use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};
use ::clap::{Parser};
use regex::{Regex};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct Echo {}

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        // // Regex for Firefox
        let re = Regex::new(r"(\s|^)Firefox\s?/\s?([0-9][0-9]|[7-9][0-9]|\d{3,})(\.|\s|$)").expect("Regex attempt failed.");
        let user_agent = request.into_inner().message;
        let decision = if re.is_match(&user_agent) {"Accepting the Request."} else {"Blocking the Request."};

        let reply = EchoResponse {
            message: format!("{}", decision), 
            user_agent: format!("{}", user_agent), 
        };

        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = Echo::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::net::{AddrParseError, IpAddr};

    #[tokio::test]
    async fn test_bad_address() {    
        let addr: Result<IpAddr, AddrParseError> = "9.9".parse();
        let result = addr.is_ok();
        assert_eq!(result, false);
    }
    //TODO: Add good address test
    //TODO: Add bad server connection test
    //TODO: Add good server connection test
}