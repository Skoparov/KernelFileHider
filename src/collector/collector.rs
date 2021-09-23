extern crate protobuf;
extern crate collector;

use std::thread;
use std::net::{TcpListener, TcpStream};
use protobuf::*;
use structopt::StructOpt;
use collector::gen::*;
use collector::common::common;

#[derive(StructOpt)]
#[structopt(name = "collector")]
struct CliArgs {
    /// Port to listen
    #[structopt(short, long)]
    port: i16,
}

fn hide(path: String) -> response::Response_Result {
    println!("Hiding path: {}", path);
    return response::Response_Result::OK;
}

fn unhide(path: String) -> response::Response_Result {
    println!("Unhiding path: {}", path);
    return response::Response_Result::OK;
}

fn uninstall() -> response::Response_Result {
    println!("Uninstall");
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
    let args = CliArgs::from_args();
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

    drop(listener);
}
