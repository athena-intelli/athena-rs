#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearContainerRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(message, optional, tag = "2")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleUnitRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(uint64, tag = "2")]
    pub unit_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerTransactionRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
    #[prost(
        enumeration = "container_transaction_request::ContainerTransaction",
        tag = "6"
    )]
    pub transaction: i32,
}
/// Nested message and enum types in `ContainerTransactionRequest`.
pub mod container_transaction_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContainerTransaction {
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
pub struct HandleContainerRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(uint64, tag = "2")]
    pub parent_container_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleLotRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
    #[prost(uint64, tag = "2")]
    pub lot_id: u64,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddContainerToQueueRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
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
    pub container_id: u64,
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
pub struct ContainerChangeRouteRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
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
pub struct ContainerCompleteRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
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
pub struct ContainerStartRequest {
    #[prost(uint64, tag = "1")]
    pub container_id: u64,
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
pub mod package_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PackageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PackageServiceClient<tonic::transport::Channel> {
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
    impl<T> PackageServiceClient<T>
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
        ) -> PackageServiceClient<InterceptedService<T, F>>
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
            PackageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "This method is used to add a container to a container"]
        pub async fn add_container_to_container(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleContainerRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/AddContainerToContainer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to add a lot to a container"]
        pub async fn add_lot_to_container(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/AddLotToContainer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "  of its contents (any contained containers, lots and units) has been moved to"]
        #[doc = "  the queue specified."]
        pub async fn add_container_to_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::AddContainerToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/AddContainerToQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to add a unit to a container"]
        pub async fn add_unit_to_container(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/AddUnitToContainer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to change the production line of a container and all of its"]
        #[doc = "  contents (any contained containers, lots and units)."]
        pub async fn change_production_line(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/ChangeProductionLine",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to change the route of a container and all of its contents"]
        #[doc = " (any contained containers, lots and units)."]
        pub async fn change_route(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/ChangeRoute");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have closed."]
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PackageService.PackageService/Close");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have completed the"]
        #[doc = " route step specified."]
        pub async fn complete_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerCompleteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/CompleteAtRouteStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have started the"]
        #[doc = " route step specified."]
        pub async fn start_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerStartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/StartAtRouteStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "   of its contents (any contained containers, lots and units) have finished."]
        pub async fn finish(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/Finish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "     of its contents (any contained containers, lots and units) have been placed on"]
        #[doc = "     hold. When an object is on hold no other transactions may be performed on"]
        #[doc = "     it, until it is released"]
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PackageService.PackageService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to record a transaction indicating that a container and its"]
        #[doc = "     * containers (any container containers) have opened. The open"]
        #[doc = "     * method is equivalent to the undoClose method, except that it propagates"]
        #[doc = "     * upward to the Container's containers instead of downward to the Container's contents."]
        pub async fn open(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PackageService.PackageService/Open");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have stopped"]
        #[doc = " processing. When an object is paused no other transactions may be"]
        #[doc = " performed on it, until it is restarted."]
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PackageService.PackageService/Pause");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have been released"]
        #[doc = " from hold or quarantine."]
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/Release");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) has been scrapped."]
        pub async fn scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PackageService.PackageService/Scrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method removes everything from a container."]
        pub async fn remove_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearContainerRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/RemoveAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a container from a container."]
        pub async fn remove_container(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleContainerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/RemoveContainer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a lot from a container."]
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
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/RemoveLot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "This method is used to remove a unit from a container."]
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
            let path =
                http::uri::PathAndQuery::from_static("/PackageService.PackageService/RemoveUnit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "*"]
        #[doc = " This method is used to save or update a container."]
        pub async fn save_container(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Container>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PackageService.PackageService/SaveContainer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod package_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PackageServiceServer."]
    #[async_trait]
    pub trait PackageService: Send + Sync + 'static {
        #[doc = "This method is used to add a container to a container"]
        async fn add_container_to_container(
            &self,
            request: tonic::Request<super::HandleContainerRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to add a lot to a container"]
        async fn add_lot_to_container(
            &self,
            request: tonic::Request<super::HandleLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "  of its contents (any contained containers, lots and units) has been moved to"]
        #[doc = "  the queue specified."]
        async fn add_container_to_queue(
            &self,
            request: tonic::Request<super::AddContainerToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to add a unit to a container"]
        async fn add_unit_to_container(
            &self,
            request: tonic::Request<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to change the production line of a container and all of its"]
        #[doc = "  contents (any contained containers, lots and units)."]
        async fn change_production_line(
            &self,
            request: tonic::Request<super::ChangePLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to change the route of a container and all of its contents"]
        #[doc = " (any contained containers, lots and units)."]
        async fn change_route(
            &self,
            request: tonic::Request<super::ContainerChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have closed."]
        async fn close(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have completed the"]
        #[doc = " route step specified."]
        async fn complete_at_route_step(
            &self,
            request: tonic::Request<super::ContainerCompleteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have started the"]
        #[doc = " route step specified."]
        async fn start_at_route_step(
            &self,
            request: tonic::Request<super::ContainerStartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "   of its contents (any contained containers, lots and units) have finished."]
        async fn finish(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = "     of its contents (any contained containers, lots and units) have been placed on"]
        #[doc = "     hold. When an object is on hold no other transactions may be performed on"]
        #[doc = "     it, until it is released"]
        async fn hold(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to record a transaction indicating that a container and its"]
        #[doc = "     * containers (any container containers) have opened. The open"]
        #[doc = "     * method is equivalent to the undoClose method, except that it propagates"]
        #[doc = "     * upward to the Container's containers instead of downward to the Container's contents."]
        async fn open(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have stopped"]
        #[doc = " processing. When an object is paused no other transactions may be"]
        #[doc = " performed on it, until it is restarted."]
        async fn pause(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) have been released"]
        #[doc = " from hold or quarantine."]
        async fn release(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to record a transaction indicating that a container and all"]
        #[doc = " of its contents (any contained containers, lots and units) has been scrapped."]
        async fn scrap(
            &self,
            request: tonic::Request<super::ContainerTransactionRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method removes everything from a container."]
        async fn remove_all(
            &self,
            request: tonic::Request<super::ClearContainerRequest>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
        #[doc = "This method is used to remove a container from a container."]
        async fn remove_container(
            &self,
            request: tonic::Request<super::HandleContainerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "This method is used to remove a lot from a container."]
        async fn remove_lot(
            &self,
            request: tonic::Request<super::HandleLotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "This method is used to remove a unit from a container."]
        async fn remove_unit(
            &self,
            request: tonic::Request<super::HandleUnitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "*"]
        #[doc = " This method is used to save or update a container."]
        async fn save_container(
            &self,
            request: tonic::Request<super::super::structures::Container>,
        ) -> Result<tonic::Response<super::super::structures::Container>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PackageServiceServer<T: PackageService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PackageService> PackageServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PackageServiceServer<T>
    where
        T: PackageService,
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
                "/PackageService.PackageService/AddContainerToContainer" => {
                    #[allow(non_camel_case_types)]
                    struct AddContainerToContainerSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::HandleContainerRequest>
                        for AddContainerToContainerSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleContainerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).add_container_to_container(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddContainerToContainerSvc(inner);
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
                "/PackageService.PackageService/AddLotToContainer" => {
                    #[allow(non_camel_case_types)]
                    struct AddLotToContainerSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService> tonic::server::UnaryService<super::HandleLotRequest>
                        for AddLotToContainerSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_lot_to_container(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLotToContainerSvc(inner);
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
                "/PackageService.PackageService/AddContainerToQueue" => {
                    #[allow(non_camel_case_types)]
                    struct AddContainerToQueueSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::AddContainerToQueueRequest>
                        for AddContainerToQueueSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddContainerToQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_container_to_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddContainerToQueueSvc(inner);
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
                "/PackageService.PackageService/AddUnitToContainer" => {
                    #[allow(non_camel_case_types)]
                    struct AddUnitToContainerSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService> tonic::server::UnaryService<super::HandleUnitRequest>
                        for AddUnitToContainerSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_unit_to_container(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUnitToContainerSvc(inner);
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
                "/PackageService.PackageService/ChangeProductionLine" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProductionLineSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService> tonic::server::UnaryService<super::ChangePLineRequest>
                        for ChangeProductionLineSvc<T>
                    {
                        type Response = super::super::structures::Container;
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
                "/PackageService.PackageService/ChangeRoute" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeRouteSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerChangeRouteRequest>
                        for ChangeRouteSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerChangeRouteRequest>,
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
                "/PackageService.PackageService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for CloseSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/CompleteAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAtRouteStepSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerCompleteRequest>
                        for CompleteAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerCompleteRequest>,
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
                "/PackageService.PackageService/StartAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct StartAtRouteStepSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerStartRequest>
                        for StartAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerStartRequest>,
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
                "/PackageService.PackageService/Finish" => {
                    #[allow(non_camel_case_types)]
                    struct FinishSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for FinishSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for HoldSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/Open" => {
                    #[allow(non_camel_case_types)]
                    struct OpenSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for OpenSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/Pause" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for PauseSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/Release" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for ReleaseSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/Scrap" => {
                    #[allow(non_camel_case_types)]
                    struct ScrapSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ContainerTransactionRequest>
                        for ScrapSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContainerTransactionRequest>,
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
                "/PackageService.PackageService/RemoveAll" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveAllSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::ClearContainerRequest>
                        for RemoveAllSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearContainerRequest>,
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
                "/PackageService.PackageService/RemoveContainer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveContainerSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::HandleContainerRequest>
                        for RemoveContainerSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleContainerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_container(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveContainerSvc(inner);
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
                "/PackageService.PackageService/RemoveLot" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLotSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService> tonic::server::UnaryService<super::HandleLotRequest> for RemoveLotSvc<T> {
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
                "/PackageService.PackageService/RemoveUnit" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUnitSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService> tonic::server::UnaryService<super::HandleUnitRequest> for RemoveUnitSvc<T> {
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
                "/PackageService.PackageService/SaveContainer" => {
                    #[allow(non_camel_case_types)]
                    struct SaveContainerSvc<T: PackageService>(pub Arc<T>);
                    impl<T: PackageService>
                        tonic::server::UnaryService<super::super::structures::Container>
                        for SaveContainerSvc<T>
                    {
                        type Response = super::super::structures::Container;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Container>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_container(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveContainerSvc(inner);
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
    impl<T: PackageService> Clone for PackageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PackageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PackageService> tonic::transport::NamedService for PackageServiceServer<T> {
        const NAME: &'static str = "PackageService.PackageService";
    }
}
