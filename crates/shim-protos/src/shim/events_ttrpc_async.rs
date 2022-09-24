// This file is generated by ttrpc-compiler 0.6.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct EventsClient {
    client: ::ttrpc::r#async::Client,
}

impl EventsClient {
    pub fn new(client: ::ttrpc::r#async::Client) -> Self {
        EventsClient {
            client: client,
        }
    }

    pub async fn forward(&self, ctx: ttrpc::context::Context, req: &super::events::ForwardRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.services.events.ttrpc.v1.Events", "Forward", cres);
    }
}

struct ForwardMethod {
    service: Arc<Box<dyn Events + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ForwardMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<::ttrpc::Response> {
        ::ttrpc::async_request_handler!(self, ctx, req, events, ForwardRequest, forward);
    }
}

#[async_trait]
pub trait Events: Sync {
    async fn forward(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _: super::events::ForwardRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.services.events.ttrpc.v1.Events/Forward is not supported".to_string())))
    }
}

pub fn create_events(service: Arc<Box<dyn Events + Send + Sync>>) -> HashMap<String, ::ttrpc::r#async::Service> {
    let mut ret = HashMap::new();
    let mut methods = HashMap::new();
    let streams = HashMap::new();

    methods.insert("Forward".to_string(),
                    Box::new(ForwardMethod{service: service.clone()}) as Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    ret.insert("containerd.services.events.ttrpc.v1.Events".to_string(), ::ttrpc::r#async::Service{ methods, streams });
    ret
}
