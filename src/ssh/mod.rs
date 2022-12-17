use crate::types::ServerConf;
use ssh2::Session;
use std::net::TcpStream;

pub fn connect_server_via_ssh(args: &ServerConf) -> Session {
    let tcp = TcpStream::connect(&args.ip).expect("connection failed");
    let mut sess = Session::new().expect("session failed");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("handshake failed");
    sess.userauth_password(&args.username, &args.password)
        .expect("userauthgone");

    return sess;
}