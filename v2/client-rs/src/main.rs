use std::net::TcpStream;
use std::path::Path;

use http_req::request::{HttpVersion, RequestBuilder};
use http_req::{tls, uri::Uri};

fn main() {
    let addr: Uri = "https://localhost:8000/".parse().expect("invalid uri");

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
    println!("connected to {}", addr);

    //Open secure connection over TlsStream, because of `addr` (https)
    let mut stream = {
        const ROOT_CA_PATH: &'static str = "../pki/ca.cert";
        let root_ca_path = Path::new(ROOT_CA_PATH);

        let mut c = tls::Config::default();

        match c.add_root_cert_file_pem(root_ca_path) {
            Ok(_) => {}
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
            //.version(HttpVersion::Http20)
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

    println!(
        "response: {}",
        std::str::from_utf8(&writer).expect("non-utf8 response")
    );
}
