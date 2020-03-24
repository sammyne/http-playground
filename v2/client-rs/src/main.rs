use std::net::TcpStream;

use http_req::{tls, uri::Uri};
use http_req::request::{RequestBuilder, HttpVersion};

fn main() {
    let addr: Uri = "https://localhost:8000".parse().expect("invalid uri");

    //Connect to remote host
    let stream = {
        // port is required
        let host_port = format!(
            "{}:{}",
            addr.host().expect("missing host"),
            addr.port().unwrap_or(addr.corr_port()),
        );
        match TcpStream::connect(&host_port) {
            Ok(v) => v,
            Err(err) => panic!("fail to connect: {:?}", err),
        }
    };
    info!("connected to {}", addr);

    //Open secure connection over TlsStream, because of `addr` (https)
    let mut stream = {
        const ROOT_CA_PATH = "../pki/ca.cert";
        let c = match tls::Config::add_root_cert_file_pem(ROOT_CA_PATH) {
            Ok(v) => v,
            Err(err) => panic!("failed to add root cert: {:?}", err),
        };

        match c.connect(addr.host().unwrap_or(""), stream) {
            Ok(v) => v,
            Err(err) => panic!("failed to connect server: {:?}", err),
        }
    };

    //Container for response's body
    let mut writer = Vec::new();

    //Add header `Connection: Close`
    let response = {
        let response = RequestBuilder::new(&addr)
            .version(HttpVersion::Http20)
            .header("Connection", "Close")
            .send(&mut stream, &mut writer);
        match response {
            Ok(v) => v,
            Err(err) => panic!("failed to do request: {:?}", err),
        }
    };

    let status_code = Into::<u16>::into(response.status_code()) as u32;
    if !response.status_code().is_success() {
        panic!("bad status: {}", status_code);
    }

    println!("response: {}", std::str::from_utf8(&writer).expect("non-utf8 response"));
}
