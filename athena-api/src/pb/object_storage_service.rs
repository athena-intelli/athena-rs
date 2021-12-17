#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveObjectRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod object_storage_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The ObjectStorage definition."]
    #[derive(Debug, Clone)]
    pub struct ObjectStorageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectStorageServiceClient<tonic::transport::Channel> {
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
    impl<T> ObjectStorageServiceClient<T>
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
        ) -> ObjectStorageServiceClient<InterceptedService<T, F>>
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
            ObjectStorageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn remove_equipment(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveEquipment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_location(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_bom(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveBOM",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_part(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemovePart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_production_line(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveProductionLine",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_production_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveProductionQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_route(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_route_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveRouteOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_station(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveStation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_workshop(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/RemoveWorkshop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_production_line_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::ProductionLine>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/ChangeProductionLinePriority",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_equipment_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Equipment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/ChangeEquipmentPriority",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::User>,
        ) -> Result<tonic::Response<super::super::structures::User>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_bom(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Bom>,
        ) -> Result<tonic::Response<super::super::structures::Bom>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveBom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_runtime_bom(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::RuntimeBom>,
        ) -> Result<tonic::Response<super::super::structures::RuntimeBom>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveRuntimeBom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_equipment(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Equipment>,
        ) -> Result<tonic::Response<super::super::structures::Equipment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveEquipment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_workshop(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Workshop>,
        ) -> Result<tonic::Response<super::super::structures::Workshop>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveWorkshop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_factory(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Factory>,
        ) -> Result<tonic::Response<super::super::structures::Factory>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveFactory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_shift(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Shift>,
        ) -> Result<tonic::Response<super::super::structures::Shift>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveShift",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_work_station(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::WorkStation>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveWorkStation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_equipment_class(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::EquipmentClass>,
        ) -> Result<tonic::Response<super::super::structures::EquipmentClass>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveEquipmentClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_part_class(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::PartClass>,
        ) -> Result<tonic::Response<super::super::structures::PartClass>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SavePartClass",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_part(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Part>,
        ) -> Result<tonic::Response<super::super::structures::Part>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SavePart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_location(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Location>,
        ) -> Result<tonic::Response<super::super::structures::Location>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_route(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Route>,
        ) -> Result<tonic::Response<super::super::structures::Route>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveRoute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_route_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::RouteOperation>,
        ) -> Result<tonic::Response<super::super::structures::RouteOperation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveRouteOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_doc(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Doc>,
        ) -> Result<tonic::Response<super::super::structures::Doc>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveDoc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Customer>,
        ) -> Result<tonic::Response<super::super::structures::Customer>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_supplier(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::Supplier>,
        ) -> Result<tonic::Response<super::super::structures::Supplier>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveSupplier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_storage_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectStorageServiceServer."]
    #[async_trait]
    pub trait ObjectStorageService: Send + Sync + 'static {
        async fn remove_equipment(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_location(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_bom(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_part(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_production_line(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_production_queue(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_route(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_route_operation(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_station(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn remove_workshop(
            &self,
            request: tonic::Request<super::RemoveObjectRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn change_production_line_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::ProductionLine>, tonic::Status>;
        async fn change_equipment_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Equipment>, tonic::Status>;
        async fn save_user(
            &self,
            request: tonic::Request<super::super::structures::User>,
        ) -> Result<tonic::Response<super::super::structures::User>, tonic::Status>;
        async fn save_bom(
            &self,
            request: tonic::Request<super::super::structures::Bom>,
        ) -> Result<tonic::Response<super::super::structures::Bom>, tonic::Status>;
        async fn save_runtime_bom(
            &self,
            request: tonic::Request<super::super::structures::RuntimeBom>,
        ) -> Result<tonic::Response<super::super::structures::RuntimeBom>, tonic::Status>;
        async fn save_equipment(
            &self,
            request: tonic::Request<super::super::structures::Equipment>,
        ) -> Result<tonic::Response<super::super::structures::Equipment>, tonic::Status>;
        async fn save_workshop(
            &self,
            request: tonic::Request<super::super::structures::Workshop>,
        ) -> Result<tonic::Response<super::super::structures::Workshop>, tonic::Status>;
        async fn save_factory(
            &self,
            request: tonic::Request<super::super::structures::Factory>,
        ) -> Result<tonic::Response<super::super::structures::Factory>, tonic::Status>;
        async fn save_shift(
            &self,
            request: tonic::Request<super::super::structures::Shift>,
        ) -> Result<tonic::Response<super::super::structures::Shift>, tonic::Status>;
        async fn save_work_station(
            &self,
            request: tonic::Request<super::super::structures::WorkStation>,
        ) -> Result<tonic::Response<super::super::structures::WorkStation>, tonic::Status>;
        async fn save_equipment_class(
            &self,
            request: tonic::Request<super::super::structures::EquipmentClass>,
        ) -> Result<tonic::Response<super::super::structures::EquipmentClass>, tonic::Status>;
        async fn save_part_class(
            &self,
            request: tonic::Request<super::super::structures::PartClass>,
        ) -> Result<tonic::Response<super::super::structures::PartClass>, tonic::Status>;
        async fn save_part(
            &self,
            request: tonic::Request<super::super::structures::Part>,
        ) -> Result<tonic::Response<super::super::structures::Part>, tonic::Status>;
        async fn save_location(
            &self,
            request: tonic::Request<super::super::structures::Location>,
        ) -> Result<tonic::Response<super::super::structures::Location>, tonic::Status>;
        async fn save_route(
            &self,
            request: tonic::Request<super::super::structures::Route>,
        ) -> Result<tonic::Response<super::super::structures::Route>, tonic::Status>;
        async fn save_route_operation(
            &self,
            request: tonic::Request<super::super::structures::RouteOperation>,
        ) -> Result<tonic::Response<super::super::structures::RouteOperation>, tonic::Status>;
        async fn save_doc(
            &self,
            request: tonic::Request<super::super::structures::Doc>,
        ) -> Result<tonic::Response<super::super::structures::Doc>, tonic::Status>;
        async fn save_customer(
            &self,
            request: tonic::Request<super::super::structures::Customer>,
        ) -> Result<tonic::Response<super::super::structures::Customer>, tonic::Status>;
        async fn save_supplier(
            &self,
            request: tonic::Request<super::super::structures::Supplier>,
        ) -> Result<tonic::Response<super::super::structures::Supplier>, tonic::Status>;
    }
    #[doc = " The ObjectStorage definition."]
    #[derive(Debug)]
    pub struct ObjectStorageServiceServer<T: ObjectStorageService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectStorageService> ObjectStorageServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectStorageServiceServer<T>
    where
        T: ObjectStorageService,
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
                "/ObjectStorageService.ObjectStorageService/RemoveEquipment" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveEquipmentSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveEquipmentSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_equipment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveEquipmentSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveLocation" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLocationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveLocationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveLocationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveBOM" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveBOMSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveBOMSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_bom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveBOMSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemovePart" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePartSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemovePartSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_part(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePartSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveProductionLine" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveProductionLineSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveProductionLineSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_production_line(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveProductionLineSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveProductionQueue" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveProductionQueueSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveProductionQueueSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).remove_production_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveProductionQueueSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveRoute" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveRouteSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveRouteSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveRouteSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveRouteOperation" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveRouteOperationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveRouteOperationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_route_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveRouteOperationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveStation" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveStationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveStationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_station(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveStationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/RemoveWorkshop" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveWorkshopSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::RemoveObjectRequest>
                        for RemoveWorkshopSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_workshop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveWorkshopSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/ChangeProductionLinePriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProductionLinePrioritySvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangeProductionLinePrioritySvc<T>
                    {
                        type Response = super::super::structures::ProductionLine;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::structures::ChangePriorityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_production_line_priority(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeProductionLinePrioritySvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/ChangeEquipmentPriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeEquipmentPrioritySvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangeEquipmentPrioritySvc<T>
                    {
                        type Response = super::super::structures::Equipment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::structures::ChangePriorityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).change_equipment_priority(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeEquipmentPrioritySvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveUser" => {
                    #[allow(non_camel_case_types)]
                    struct SaveUserSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::User>
                        for SaveUserSvc<T>
                    {
                        type Response = super::super::structures::User;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::User>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveUserSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveBom" => {
                    #[allow(non_camel_case_types)]
                    struct SaveBomSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Bom>
                        for SaveBomSvc<T>
                    {
                        type Response = super::super::structures::Bom;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Bom>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_bom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveBomSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveRuntimeBom" => {
                    #[allow(non_camel_case_types)]
                    struct SaveRuntimeBomSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::RuntimeBom>
                        for SaveRuntimeBomSvc<T>
                    {
                        type Response = super::super::structures::RuntimeBom;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::RuntimeBom>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_runtime_bom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveRuntimeBomSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveEquipment" => {
                    #[allow(non_camel_case_types)]
                    struct SaveEquipmentSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Equipment>
                        for SaveEquipmentSvc<T>
                    {
                        type Response = super::super::structures::Equipment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Equipment>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_equipment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveEquipmentSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveWorkshop" => {
                    #[allow(non_camel_case_types)]
                    struct SaveWorkshopSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Workshop>
                        for SaveWorkshopSvc<T>
                    {
                        type Response = super::super::structures::Workshop;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Workshop>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_workshop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveWorkshopSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveFactory" => {
                    #[allow(non_camel_case_types)]
                    struct SaveFactorySvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Factory>
                        for SaveFactorySvc<T>
                    {
                        type Response = super::super::structures::Factory;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Factory>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_factory(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveFactorySvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveShift" => {
                    #[allow(non_camel_case_types)]
                    struct SaveShiftSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Shift>
                        for SaveShiftSvc<T>
                    {
                        type Response = super::super::structures::Shift;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Shift>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_shift(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveShiftSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveWorkStation" => {
                    #[allow(non_camel_case_types)]
                    struct SaveWorkStationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::WorkStation>
                        for SaveWorkStationSvc<T>
                    {
                        type Response = super::super::structures::WorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::WorkStation>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_work_station(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveWorkStationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveEquipmentClass" => {
                    #[allow(non_camel_case_types)]
                    struct SaveEquipmentClassSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::EquipmentClass>
                        for SaveEquipmentClassSvc<T>
                    {
                        type Response = super::super::structures::EquipmentClass;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::EquipmentClass>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_equipment_class(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveEquipmentClassSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SavePartClass" => {
                    #[allow(non_camel_case_types)]
                    struct SavePartClassSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::PartClass>
                        for SavePartClassSvc<T>
                    {
                        type Response = super::super::structures::PartClass;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::PartClass>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_part_class(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SavePartClassSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SavePart" => {
                    #[allow(non_camel_case_types)]
                    struct SavePartSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Part>
                        for SavePartSvc<T>
                    {
                        type Response = super::super::structures::Part;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Part>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_part(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SavePartSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveLocation" => {
                    #[allow(non_camel_case_types)]
                    struct SaveLocationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Location>
                        for SaveLocationSvc<T>
                    {
                        type Response = super::super::structures::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Location>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveLocationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveRoute" => {
                    #[allow(non_camel_case_types)]
                    struct SaveRouteSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Route>
                        for SaveRouteSvc<T>
                    {
                        type Response = super::super::structures::Route;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Route>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveRouteSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveRouteOperation" => {
                    #[allow(non_camel_case_types)]
                    struct SaveRouteOperationSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::RouteOperation>
                        for SaveRouteOperationSvc<T>
                    {
                        type Response = super::super::structures::RouteOperation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::RouteOperation>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_route_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveRouteOperationSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveDoc" => {
                    #[allow(non_camel_case_types)]
                    struct SaveDocSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Doc>
                        for SaveDocSvc<T>
                    {
                        type Response = super::super::structures::Doc;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Doc>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_doc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveDocSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct SaveCustomerSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Customer>
                        for SaveCustomerSvc<T>
                    {
                        type Response = super::super::structures::Customer;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Customer>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_customer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveCustomerSvc(inner);
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
                "/ObjectStorageService.ObjectStorageService/SaveSupplier" => {
                    #[allow(non_camel_case_types)]
                    struct SaveSupplierSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::Supplier>
                        for SaveSupplierSvc<T>
                    {
                        type Response = super::super::structures::Supplier;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::Supplier>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_supplier(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveSupplierSvc(inner);
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
    impl<T: ObjectStorageService> Clone for ObjectStorageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectStorageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectStorageService> tonic::transport::NamedService for ObjectStorageServiceServer<T> {
        const NAME: &'static str = "ObjectStorageService.ObjectStorageService";
    }
}
