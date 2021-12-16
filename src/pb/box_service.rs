#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearBoxRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(message, optional, tag = "2")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleUnitRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(uint64, tag = "2")]
    pub unit_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxTransactionRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
    #[prost(enumeration = "box_transaction_request::BoxTransaction", tag = "6")]
    pub transaction: i32,
}
/// Nested message and enum types in `BoxTransactionRequest`.
pub mod box_transaction_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BoxTransaction {
        Close = 0,
        Finish = 1,
        Pause = 2,
        Release = 3,
        Restart = 4,
        Open = 5,
        Hold = 6,
        Ship = 7,
        UndoClose = 8,
        UndoFinish = 9,
        UndoScrap = 10,
        UndoShip = 11,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleBoxRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(uint64, tag = "2")]
    pub box_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleLotRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(uint64, tag = "2")]
    pub lot_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBoxToQueueRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub route_queue_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePLineRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub production_line_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub production_line_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxChangeRouteRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub force_change_route: bool,
    #[prost(uint64, tag = "6")]
    pub route_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxCompleteRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
    #[prost(uint64, tag = "6")]
    pub route_step_id: u64,
    #[prost(string, tag = "7")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub route_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxStartRequest {
    #[prost(uint64, tag = "1")]
    pub box_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
    #[prost(uint64, tag = "6")]
    pub route_step_id: u64,
    #[prost(string, tag = "7")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub route_id: u64,
}
#[doc = r" Generated client implementations."]
pub mod box_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BoxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BoxServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BoxServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BoxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BoxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "This method is used to add a box to a box"]
        pub async fn add_box_to_box(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleBoxRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/AddBoxToBox");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to add a lot to a box"]
        pub async fn add_lot_to_box(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleLotRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/AddLotToBox");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "  of its contents (any contained boxes, lots and units) has been moved to"]
        #[doc = "  the queue specified."]
        pub async fn add_box_to_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::AddBoxToQueueRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/AddBoxToQueue");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to add a unit to a box"]
        pub async fn add_unit_to_box(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/AddUnitToBox");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to change the production line of a box and all of its"]
        #[doc = "  contents (any contained boxes, lots and units)."]
        pub async fn change_production_line(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePLineRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/BoxService.BoxService/ChangeProductionLine");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to change the route of a box and all of its contents"]
        #[doc = " (any contained boxes, lots and units)."]
        pub async fn change_route(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/ChangeRoute");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have closed."]
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Close");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have completed the"]
        #[doc = " route step specified."]
        pub async fn complete_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxCompleteRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/BoxService.BoxService/CompleteAtRouteStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have started the"]
        #[doc = " route step specified."]
        pub async fn start_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxStartRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/BoxService.BoxService/StartAtRouteStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "   of its contents (any contained boxes, lots and units) have finished."]
        pub async fn finish(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Finish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "     of its contents (any contained boxes, lots and units) have been placed on"]
        #[doc = "     hold. When an object is on hold no other transactions may be performed on"]
        #[doc = "     it, until it is released"]
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to record a transaction indicating that a box and its"]
        #[doc = "     * containers (any container boxes and carriers) have opened. The open"]
        #[doc = "     * method is equivalent to the undoClose method, except that it propagates"]
        #[doc = "     * upward to the Box's containers instead of downward to the Box's contents."]
        pub async fn open(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Open");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have stopped"]
        #[doc = " processing. When an object is paused no other transactions may be"]
        #[doc = " performed on it, until it is restarted."]
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Pause");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have been released"]
        #[doc = " from hold or quarantine."]
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Release");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) has been scrapped."]
        pub async fn scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/Scrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method removes everything from a box."]
        pub async fn remove_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearBoxRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/RemoveAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a box from a box."]
        pub async fn remove_box(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleBoxRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/RemoveBox");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a lot from a box."]
        pub async fn remove_lot(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleLotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/RemoveLot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a unit from a box."]
        pub async fn remove_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/RemoveUnit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to save or update a box."]
        pub async fn save_box(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DBox>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxService.BoxService/SaveBox");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod box_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BoxServiceServer."]
    #[async_trait]
    pub trait BoxService: Send + Sync + 'static {
        #[doc = "This method is used to add a box to a box"]
        async fn add_box_to_box(
            &self,
            request: tonic::Request<super::HandleBoxRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to add a lot to a box"]
        async fn add_lot_to_box(
            &self,
            request: tonic::Request<super::HandleLotRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "  of its contents (any contained boxes, lots and units) has been moved to"]
        #[doc = "  the queue specified."]
        async fn add_box_to_queue(
            &self,
            request: tonic::Request<super::AddBoxToQueueRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to add a unit to a box"]
        async fn add_unit_to_box(
            &self,
            request: tonic::Request<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to change the production line of a box and all of its"]
        #[doc = "  contents (any contained boxes, lots and units)."]
        async fn change_production_line(
            &self,
            request: tonic::Request<super::ChangePLineRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to change the route of a box and all of its contents"]
        #[doc = " (any contained boxes, lots and units)."]
        async fn change_route(
            &self,
            request: tonic::Request<super::BoxChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have closed."]
        async fn close(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have completed the"]
        #[doc = " route step specified."]
        async fn complete_at_route_step(
            &self,
            request: tonic::Request<super::BoxCompleteRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have started the"]
        #[doc = " route step specified."]
        async fn start_at_route_step(
            &self,
            request: tonic::Request<super::BoxStartRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "   of its contents (any contained boxes, lots and units) have finished."]
        async fn finish(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = "     of its contents (any contained boxes, lots and units) have been placed on"]
        #[doc = "     hold. When an object is on hold no other transactions may be performed on"]
        #[doc = "     it, until it is released"]
        async fn hold(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to record a transaction indicating that a box and its"]
        #[doc = "     * containers (any container boxes and carriers) have opened. The open"]
        #[doc = "     * method is equivalent to the undoClose method, except that it propagates"]
        #[doc = "     * upward to the Box's containers instead of downward to the Box's contents."]
        async fn open(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have stopped"]
        #[doc = " processing. When an object is paused no other transactions may be"]
        #[doc = " performed on it, until it is restarted."]
        async fn pause(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) have been released"]
        #[doc = " from hold or quarantine."]
        async fn release(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a box and all"]
        #[doc = " of its contents (any contained boxes, lots and units) has been scrapped."]
        async fn scrap(
            &self,
            request: tonic::Request<super::BoxTransactionRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method removes everything from a box."]
        async fn remove_all(
            &self,
            request: tonic::Request<super::ClearBoxRequest>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
        #[doc = "This method is used to remove a box from a box."]
        async fn remove_box(
            &self,
            request: tonic::Request<super::HandleBoxRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "This method is used to remove a lot from a box."]
        async fn remove_lot(
            &self,
            request: tonic::Request<super::HandleLotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "This method is used to remove a unit from a box."]
        async fn remove_unit(
            &self,
            request: tonic::Request<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to save or update a box."]
        async fn save_box(
            &self,
            request: tonic::Request<super::super::dto::DBox>,
        ) -> Result<tonic::Response<super::super::dto::DBox>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BoxServiceServer<T: BoxService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BoxService> BoxServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BoxServiceServer<T>
    where
        T: BoxService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/BoxService.BoxService/AddBoxToBox" => {
                    #[allow(non_camel_case_types)]
                    struct AddBoxToBoxSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleBoxRequest> for AddBoxToBoxSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleBoxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_box_to_box(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddBoxToBoxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/AddLotToBox" => {
                    #[allow(non_camel_case_types)]
                    struct AddLotToBoxSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleLotRequest> for AddLotToBoxSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_lot_to_box(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLotToBoxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/AddBoxToQueue" => {
                    #[allow(non_camel_case_types)]
                    struct AddBoxToQueueSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::AddBoxToQueueRequest>
                        for AddBoxToQueueSvc<T>
                    {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddBoxToQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_box_to_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddBoxToQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/AddUnitToBox" => {
                    #[allow(non_camel_case_types)]
                    struct AddUnitToBoxSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleUnitRequest> for AddUnitToBoxSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_unit_to_box(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUnitToBoxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/ChangeProductionLine" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProductionLineSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::ChangePLineRequest>
                        for ChangeProductionLineSvc<T>
                    {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangePLineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_production_line(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeProductionLineSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/ChangeRoute" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeRouteSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxChangeRouteRequest>
                        for ChangeRouteSvc<T>
                    {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxChangeRouteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeRouteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for CloseSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/CompleteAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAtRouteStepSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxCompleteRequest>
                        for CompleteAtRouteStepSvc<T>
                    {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxCompleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).complete_at_route_step(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteAtRouteStepSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/StartAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct StartAtRouteStepSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxStartRequest> for StartAtRouteStepSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxStartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_at_route_step(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartAtRouteStepSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Finish" => {
                    #[allow(non_camel_case_types)]
                    struct FinishSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for FinishSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finish(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for HoldSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).hold(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HoldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Open" => {
                    #[allow(non_camel_case_types)]
                    struct OpenSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for OpenSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Pause" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for PauseSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pause(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PauseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Release" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for ReleaseSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).release(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReleaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/Scrap" => {
                    #[allow(non_camel_case_types)]
                    struct ScrapSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::BoxTransactionRequest> for ScrapSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BoxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scrap(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScrapSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/RemoveAll" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveAllSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::ClearBoxRequest> for RemoveAllSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearBoxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/RemoveBox" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveBoxSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleBoxRequest> for RemoveBoxSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleBoxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_box(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveBoxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/RemoveLot" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLotSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleLotRequest> for RemoveLotSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_lot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveLotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/RemoveUnit" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUnitSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::HandleUnitRequest> for RemoveUnitSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_unit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveUnitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/BoxService.BoxService/SaveBox" => {
                    #[allow(non_camel_case_types)]
                    struct SaveBoxSvc<T: BoxService>(pub Arc<T>);
                    impl<T: BoxService> tonic::server::UnaryService<super::super::dto::DBox> for SaveBoxSvc<T> {
                        type Response = super::super::dto::DBox;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DBox>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_box(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveBoxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BoxService> Clone for BoxServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BoxService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BoxService> tonic::transport::NamedService for BoxServiceServer<T> {
        const NAME: &'static str = "BoxService.BoxService";
    }
}
