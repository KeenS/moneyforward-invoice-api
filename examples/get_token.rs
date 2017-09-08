//! Envs:
//! "MF_INVOICE_CLIENT_ID" -- client id
//! "MF_INVOICE_CLIENT_SECRET" -- client secret


extern crate env_logger;
extern crate native_tls;
extern crate oauth2;
extern crate url;

use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
use oauth2::{Token, Config};
use url::Url;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

static SERVER: &str = "localhost:3000";

fn accept_tls<P: AsRef<Path>>(identity_file: P, password: &str) -> TlsStream<TcpStream> {
    let mut file = File::open(identity_file).unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Pkcs12::from_der(&pkcs12, password).unwrap();

    let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();

    let listener = TcpListener::bind(SERVER).unwrap();

    let (stream, _remote) = listener.accept().unwrap();
    let stream = acceptor.accept(stream).unwrap();
    stream
}

fn handle_client(stream: TlsStream<TcpStream>) -> String {
    let code;
    let state;
    let mut reader = BufReader::new(stream);
    {

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

        let code_pair = url.query_pairs()
            .find(|pair| {
                let &(ref key, _) = pair;
                key == "code"
            })
            .unwrap();

        let (_, value) = code_pair;
        code = value.into_owned();

        let state_pair = url.query_pairs()
            .find(|pair| {
                let &(ref key, _) = pair;
                key == "state"
            })
            .unwrap();

        let (_, value) = state_pair;
        state = value.into_owned();
    }
    let mut stream = reader.into_inner();

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();
    println!("MF returned the following code:\n{}\n", code);
    println!("MF returned the following state:\n{}\n", state);
    code
}

fn oauth<P: AsRef<Path>>(identity_file: P, password: &str) -> Token {
    let client_id = env::var("MF_INVOICE_CLIENT_ID").unwrap();
    let client_secret = env::var("MF_INVOICE_CLIENT_SECRET").unwrap();
    let config = Config::new(
        client_id,
        client_secret,
        "https://invoice.moneyforward.com/oauth/authorize",
        "https://invoice.moneyforward.com/oauth/token",
    ).add_scope("write")
        .set_state("1234")
        .set_redirect_url(format!("https://{}/cb", SERVER));

    let authorize_url = config.authorize_url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    let stream = accept_tls(identity_file, password);
    let code = handle_client(stream);

    // Exchange the code with a token.
    let token = config.exchange_code(code);

    println!("MF returned the following token:\n{:?}\n", token);
    token.unwrap()
}

fn main() {
    env_logger::init().unwrap();
    let identity_file = env::args().nth(1).unwrap();
    let password = env::args().nth(2).unwrap();
    oauth(identity_file, &password);
}
