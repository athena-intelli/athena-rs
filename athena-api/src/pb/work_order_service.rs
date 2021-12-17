#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartOrderRequest {
    #[prost(uint64, tag = "1")]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteOrderRequest {
    #[prost(uint64, tag = "1")]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "5")]
    pub override_route_enforcement: bool,
    #[prost(string, tag = "6")]
    pub complete_reason: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionOrderRequest {
    #[prost(uint64, tag = "1")]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "TransitionOrderState", tag = "5")]
    pub transition_state: i32,
    #[prost(bool, tag = "6")]
    pub override_route_enforcement: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrderRequest {
    #[prost(uint64, tag = "1")]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub delete_reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransitionOrderState {
    OrderClose = 0,
    OrderFinish = 1,
    OrderHold = 2,
    OrderOpen = 3,
    OrderQuarantine = 4,
    OrderRelease = 5,
    OrderShip = 6,
    OrderUndoClose = 7,
}
#[doc = r" Generated client implementations."]
pub mod work_order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WorkOrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkOrderServiceClient<tonic::transport::Channel> {
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
    impl<T> WorkOrderServiceClient<T>
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
        ) -> WorkOrderServiceClient<InterceptedService<T, F>>
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
            WorkOrderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn change_order_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/ChangeOrderPriority",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Close");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn finish(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Finish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn open(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Open");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn quarantine(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/Quarantine",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Release");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_order(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrderRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/DeleteOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_work_order(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::WorkOrder>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/SaveWorkOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn ship(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/WorkOrderService.WorkOrderService/Ship");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/UndoClose",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to record a transaction indicating that this"]
        #[doc = " WorkOrder and all of its children (any Lots, Units) have"]
        #[doc = " started at the route step specified."]
        pub async fn start_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::StartOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/StartAtRouteStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "  This method is used to record a transaction indicating that this"]
        #[doc = "  WorkOrder and all of its children (any  Lots, Units) have"]
        #[doc = "  completed the route step specified."]
        pub async fn complete_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkOrderService.WorkOrderService/CompleteAtRouteStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod work_order_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkOrderServiceServer."]
    #[async_trait]
    pub trait WorkOrderService: Send + Sync + 'static {
        async fn change_order_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn close(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn finish(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn hold(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn open(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn quarantine(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn release(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn delete_order(
            &self,
            request: tonic::Request<super::DeleteOrderRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn save_work_order(
            &self,
            request: tonic::Request<super::super::structures::WorkOrder>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn ship(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        async fn undo_close(
            &self,
            request: tonic::Request<super::TransitionOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        #[doc = " This method is used to record a transaction indicating that this"]
        #[doc = " WorkOrder and all of its children (any Lots, Units) have"]
        #[doc = " started at the route step specified."]
        async fn start_at_route_step(
            &self,
            request: tonic::Request<super::StartOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
        #[doc = "  This method is used to record a transaction indicating that this"]
        #[doc = "  WorkOrder and all of its children (any  Lots, Units) have"]
        #[doc = "  completed the route step specified."]
        async fn complete_at_route_step(
            &self,
            request: tonic::Request<super::CompleteOrderRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkOrder>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WorkOrderServiceServer<T: WorkOrderService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkOrderService> WorkOrderServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkOrderServiceServer<T>
    where
        T: WorkOrderService,
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
                "/WorkOrderService.WorkOrderService/ChangeOrderPriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeOrderPrioritySvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangeOrderPrioritySvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::structures::ChangePriorityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_order_priority(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeOrderPrioritySvc(inner);
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
                "/WorkOrderService.WorkOrderService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest> for CloseSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/Finish" => {
                    #[allow(non_camel_case_types)]
                    struct FinishSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest>
                        for FinishSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest> for HoldSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/Open" => {
                    #[allow(non_camel_case_types)]
                    struct OpenSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest> for OpenSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/Quarantine" => {
                    #[allow(non_camel_case_types)]
                    struct QuarantineSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest>
                        for QuarantineSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).quarantine(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuarantineSvc(inner);
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
                "/WorkOrderService.WorkOrderService/Release" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest>
                        for ReleaseSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/DeleteOrder" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrderSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService> tonic::server::UnaryService<super::DeleteOrderRequest>
                        for DeleteOrderSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteOrderSvc(inner);
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
                "/WorkOrderService.WorkOrderService/SaveWorkOrder" => {
                    #[allow(non_camel_case_types)]
                    struct SaveWorkOrderSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::super::structures::WorkOrder>
                        for SaveWorkOrderSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::WorkOrder>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_work_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveWorkOrderSvc(inner);
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
                "/WorkOrderService.WorkOrderService/Ship" => {
                    #[allow(non_camel_case_types)]
                    struct ShipSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest> for ShipSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ship(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShipSvc(inner);
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
                "/WorkOrderService.WorkOrderService/UndoClose" => {
                    #[allow(non_camel_case_types)]
                    struct UndoCloseSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::TransitionOrderRequest>
                        for UndoCloseSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionOrderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undo_close(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndoCloseSvc(inner);
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
                "/WorkOrderService.WorkOrderService/StartAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct StartAtRouteStepSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService> tonic::server::UnaryService<super::StartOrderRequest>
                        for StartAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartOrderRequest>,
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
                "/WorkOrderService.WorkOrderService/CompleteAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAtRouteStepSvc<T: WorkOrderService>(pub Arc<T>);
                    impl<T: WorkOrderService>
                        tonic::server::UnaryService<super::CompleteOrderRequest>
                        for CompleteAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::WorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteOrderRequest>,
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
    impl<T: WorkOrderService> Clone for WorkOrderServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WorkOrderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkOrderService> tonic::transport::NamedService for WorkOrderServiceServer<T> {
        const NAME: &'static str = "WorkOrderService.WorkOrderService";
    }
}
