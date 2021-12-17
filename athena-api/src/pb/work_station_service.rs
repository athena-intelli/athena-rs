#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToolRequest {
    #[prost(uint64, tag = "1")]
    pub work_station_id: u64,
    #[prost(uint64, repeated, tag = "2")]
    pub equipment_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "3")]
    pub remove_from_present_work_station: bool,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveToolRequest {
    #[prost(uint64, tag = "1")]
    pub work_station_id: u64,
    #[prost(uint64, repeated, tag = "2")]
    pub equipment_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod work_station_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WorkStationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkStationServiceClient<tonic::transport::Channel> {
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
    impl<T> WorkStationServiceClient<T>
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
        ) -> WorkStationServiceClient<InterceptedService<T, F>>
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
            WorkStationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn add_tools(
            &mut self,
            request: impl tonic::IntoRequest<super::AddToolRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkStationService.WorkStationService/AddTools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkStationService.WorkStationService/ChangePriority",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_tools(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveToolRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/WorkStationService.WorkStationService/RemoveTools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod work_station_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkStationServiceServer."]
    #[async_trait]
    pub trait WorkStationService: Send + Sync + 'static {
        async fn add_tools(
            &self,
            request: tonic::Request<super::AddToolRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status>;
        async fn change_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status>;
        async fn remove_tools(
            &self,
            request: tonic::Request<super::RemoveToolRequest>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WorkStationServiceServer<T: WorkStationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkStationService> WorkStationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkStationServiceServer<T>
    where
        T: WorkStationService,
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
                "/WorkStationService.WorkStationService/AddTools" => {
                    #[allow(non_camel_case_types)]
                    struct AddToolsSvc<T: WorkStationService>(pub Arc<T>);
                    impl<T: WorkStationService> tonic::server::UnaryService<super::AddToolRequest> for AddToolsSvc<T> {
                        type Response = super::super::structures::WorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddToolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_tools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddToolsSvc(inner);
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
                "/WorkStationService.WorkStationService/ChangePriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePrioritySvc<T: WorkStationService>(pub Arc<T>);
                    impl<T: WorkStationService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangePrioritySvc<T>
                    {
                        type Response = super::super::structures::WorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::structures::ChangePriorityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_priority(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangePrioritySvc(inner);
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
                "/WorkStationService.WorkStationService/RemoveTools" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveToolsSvc<T: WorkStationService>(pub Arc<T>);
                    impl<T: WorkStationService>
                        tonic::server::UnaryService<super::RemoveToolRequest>
                        for RemoveToolsSvc<T>
                    {
                        type Response = super::super::structures::WorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveToolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_tools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveToolsSvc(inner);
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
    impl<T: WorkStationService> Clone for WorkStationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WorkStationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkStationService> tonic::transport::NamedService for WorkStationServiceServer<T> {
        const NAME: &'static str = "WorkStationService.WorkStationService";
    }
}
