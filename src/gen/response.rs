// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    result: ::std::option::Option<Response_Result>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    // required .Response.Result result = 1;


    pub fn get_result(&self) -> Response_Result {
        self.result.unwrap_or(Response_Result::OK)
    }
    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Response_Result) {
        self.result = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&v))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Response_Result>>(
                "result",
                |m: &Response| { &m.result },
                |m: &mut Response| { &mut m.result },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Response>(
                "Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Response {
        static instance: ::protobuf::rt::LazyV2<Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Response::new)
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.result = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Response_Result {
    OK = 0,
    ERROR = 1,
}

impl ::protobuf::ProtobufEnum for Response_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_Result> {
        match value {
            0 => ::std::option::Option::Some(Response_Result::OK),
            1 => ::std::option::Option::Some(Response_Result::ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_Result] = &[
            Response_Result::OK,
            Response_Result::ERROR,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Response_Result>("Response.Result", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Response_Result {
}

impl ::std::default::Default for Response_Result {
    fn default() -> Self {
        Response_Result::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for Response_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eresponse.proto\"Q\n\x08Response\x12(\n\x06result\x18\x01\x20\x02(\
    \x0e2\x10.Response.ResultR\x06result\"\x1b\n\x06Result\x12\x06\n\x02OK\
    \x10\0\x12\t\n\x05ERROR\x10\x01\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}