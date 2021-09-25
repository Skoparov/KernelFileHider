extern crate protobuf;
extern crate collector;

use std::net::{TcpStream};
use std::path::PathBuf;
use protobuf::Message;
use structopt::StructOpt;
use collector::gen::*;
use collector::common::*;

const COMMAND_VALS: &[&str] = &["hide", "unhide", "uninstall"];

#[derive(StructOpt)]
#[structopt(name = "server_emulator")]
struct CliArgs {
   /// Collector address
   #[structopt(short, long, default_value = "localhost")]
   address: String,

   /// Collector port
   #[structopt(short, long)]
   port: i16,

   /// Command to execute
   #[structopt(short, long, possible_values(COMMAND_VALS))]
   command: String,

   /// Path to hide/unhide
   #[structopt(short, long, parse(from_os_str), required_if("command", "hide"), required_if("command", "unhide"))]
   target_path: Option<PathBuf>,
}

fn get_command(command: &String) -> commands::Command_Type{
   match command.as_ref() {
      "hide" => commands::Command_Type::HIDE,
      "unhide" => commands::Command_Type::UNHIDE,
      "uninstall" => commands::Command_Type::UNINSTALL,
      _ => panic!("Invalid command")
   }
}

fn send_command(
   mut stream: &TcpStream,
   command_type: commands::Command_Type,
   path: Option<PathBuf>)
{
   println!("Send command: {:?}, path: {:?}", command_type, path);

   let mut command = commands::Command::new();
   command.set_command_type(command_type);

   if path.is_some() {
      command.set_path(path.unwrap().into_os_string().into_string().unwrap());
   }

   common::send_message(&mut stream, &command);
}

fn main() {
   let args = CliArgs::from_args();
   let connection_str = format!("{}:{}", args.address, args.port);
   println!("Connecting to {}...", connection_str);

   match TcpStream::connect(connection_str) {
      Ok(mut stream) => {
         send_command(&mut stream, get_command(&args.command), args.target_path);
         let response = response::Response::parse_from_reader(&mut stream).unwrap();
         println!("Response: {:?}", response.get_result());
      },
      Err(e) => {
         eprintln!("Failed to connect: {}", e);
      }
   }
}
