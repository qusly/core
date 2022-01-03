use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;

fn main() {
    let tcp = TcpStream::connect("").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("", "").unwrap();

    println!("{}", sess.authenticated());

    let mut channel = sess.channel_session().unwrap();

    let sftp = sess.sftp().unwrap();

    let xd = PathBuf::from("/home/");
    let files = sftp.readdir(xd.as_path()).unwrap();

    println!("{}", 1)

    // channel.exec("ls").unwrap();
    // let mut s = String::new();
    // channel.read_to_string(&mut s).unwrap();
    // println!("{}", s);
    // channel.wait_close();
    // println!("{}", channel.exit_status().unwrap());
}
