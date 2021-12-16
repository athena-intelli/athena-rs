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
        ) -> Result<tonic::Response<super::super::structures::DProductionLine>, tonic::Status>
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
        ) -> Result<tonic::Response<super::super::structures::DEquipment>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DUser>,
        ) -> Result<tonic::Response<super::super::structures::DUser>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DBom>,
        ) -> Result<tonic::Response<super::super::structures::DBom>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DRuntimeBom>,
        ) -> Result<tonic::Response<super::super::structures::DRuntimeBom>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DEquipment>,
        ) -> Result<tonic::Response<super::super::structures::DEquipment>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DWorkshop>,
        ) -> Result<tonic::Response<super::super::structures::DWorkshop>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DFactory>,
        ) -> Result<tonic::Response<super::super::structures::DFactory>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DShift>,
        ) -> Result<tonic::Response<super::super::structures::DShift>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DWorkStation>,
        ) -> Result<tonic::Response<super::super::structures::DWorkStation>, tonic::Status>
        {
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
            request: impl tonic::IntoRequest<super::super::structures::DEquipmentClass>,
        ) -> Result<tonic::Response<super::super::structures::DEquipmentClass>, tonic::Status>
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
            request: impl tonic::IntoRequest<super::super::structures::DPartClass>,
        ) -> Result<tonic::Response<super::super::structures::DPartClass>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DPart>,
        ) -> Result<tonic::Response<super::super::structures::DPart>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DLocation>,
        ) -> Result<tonic::Response<super::super::structures::DLocation>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DRoute>,
        ) -> Result<tonic::Response<super::super::structures::DRoute>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DRouteOperation>,
        ) -> Result<tonic::Response<super::super::structures::DRouteOperation>, tonic::Status>
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
        pub async fn save_ea_definition(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::DeaDefinition>,
        ) -> Result<tonic::Response<super::super::structures::DeaDefinition>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectStorageService.ObjectStorageService/SaveEADefinition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_doc(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::DDoc>,
        ) -> Result<tonic::Response<super::super::structures::DDoc>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DCustomer>,
        ) -> Result<tonic::Response<super::super::structures::DCustomer>, tonic::Status> {
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
            request: impl tonic::IntoRequest<super::super::structures::DSupplier>,
        ) -> Result<tonic::Response<super::super::structures::DSupplier>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::structures::DProductionLine>, tonic::Status>;
        async fn change_equipment_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::DEquipment>, tonic::Status>;
        async fn save_user(
            &self,
            request: tonic::Request<super::super::structures::DUser>,
        ) -> Result<tonic::Response<super::super::structures::DUser>, tonic::Status>;
        async fn save_bom(
            &self,
            request: tonic::Request<super::super::structures::DBom>,
        ) -> Result<tonic::Response<super::super::structures::DBom>, tonic::Status>;
        async fn save_runtime_bom(
            &self,
            request: tonic::Request<super::super::structures::DRuntimeBom>,
        ) -> Result<tonic::Response<super::super::structures::DRuntimeBom>, tonic::Status>;
        async fn save_equipment(
            &self,
            request: tonic::Request<super::super::structures::DEquipment>,
        ) -> Result<tonic::Response<super::super::structures::DEquipment>, tonic::Status>;
        async fn save_workshop(
            &self,
            request: tonic::Request<super::super::structures::DWorkshop>,
        ) -> Result<tonic::Response<super::super::structures::DWorkshop>, tonic::Status>;
        async fn save_factory(
            &self,
            request: tonic::Request<super::super::structures::DFactory>,
        ) -> Result<tonic::Response<super::super::structures::DFactory>, tonic::Status>;
        async fn save_shift(
            &self,
            request: tonic::Request<super::super::structures::DShift>,
        ) -> Result<tonic::Response<super::super::structures::DShift>, tonic::Status>;
        async fn save_work_station(
            &self,
            request: tonic::Request<super::super::structures::DWorkStation>,
        ) -> Result<tonic::Response<super::super::structures::DWorkStation>, tonic::Status>;
        async fn save_equipment_class(
            &self,
            request: tonic::Request<super::super::structures::DEquipmentClass>,
        ) -> Result<tonic::Response<super::super::structures::DEquipmentClass>, tonic::Status>;
        async fn save_part_class(
            &self,
            request: tonic::Request<super::super::structures::DPartClass>,
        ) -> Result<tonic::Response<super::super::structures::DPartClass>, tonic::Status>;
        async fn save_part(
            &self,
            request: tonic::Request<super::super::structures::DPart>,
        ) -> Result<tonic::Response<super::super::structures::DPart>, tonic::Status>;
        async fn save_location(
            &self,
            request: tonic::Request<super::super::structures::DLocation>,
        ) -> Result<tonic::Response<super::super::structures::DLocation>, tonic::Status>;
        async fn save_route(
            &self,
            request: tonic::Request<super::super::structures::DRoute>,
        ) -> Result<tonic::Response<super::super::structures::DRoute>, tonic::Status>;
        async fn save_route_operation(
            &self,
            request: tonic::Request<super::super::structures::DRouteOperation>,
        ) -> Result<tonic::Response<super::super::structures::DRouteOperation>, tonic::Status>;
        async fn save_ea_definition(
            &self,
            request: tonic::Request<super::super::structures::DeaDefinition>,
        ) -> Result<tonic::Response<super::super::structures::DeaDefinition>, tonic::Status>;
        async fn save_doc(
            &self,
            request: tonic::Request<super::super::structures::DDoc>,
        ) -> Result<tonic::Response<super::super::structures::DDoc>, tonic::Status>;
        async fn save_customer(
            &self,
            request: tonic::Request<super::super::structures::DCustomer>,
        ) -> Result<tonic::Response<super::super::structures::DCustomer>, tonic::Status>;
        async fn save_supplier(
            &self,
            request: tonic::Request<super::super::structures::DSupplier>,
        ) -> Result<tonic::Response<super::super::structures::DSupplier>, tonic::Status>;
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
                        type Response = super::super::structures::DProductionLine;
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
                        type Response = super::super::structures::DEquipment;
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
                        tonic::server::UnaryService<super::super::structures::DUser>
                        for SaveUserSvc<T>
                    {
                        type Response = super::super::structures::DUser;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DUser>,
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
                        tonic::server::UnaryService<super::super::structures::DBom>
                        for SaveBomSvc<T>
                    {
                        type Response = super::super::structures::DBom;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DBom>,
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
                        tonic::server::UnaryService<super::super::structures::DRuntimeBom>
                        for SaveRuntimeBomSvc<T>
                    {
                        type Response = super::super::structures::DRuntimeBom;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DRuntimeBom>,
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
                        tonic::server::UnaryService<super::super::structures::DEquipment>
                        for SaveEquipmentSvc<T>
                    {
                        type Response = super::super::structures::DEquipment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DEquipment>,
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
                        tonic::server::UnaryService<super::super::structures::DWorkshop>
                        for SaveWorkshopSvc<T>
                    {
                        type Response = super::super::structures::DWorkshop;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DWorkshop>,
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
                        tonic::server::UnaryService<super::super::structures::DFactory>
                        for SaveFactorySvc<T>
                    {
                        type Response = super::super::structures::DFactory;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DFactory>,
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
                        tonic::server::UnaryService<super::super::structures::DShift>
                        for SaveShiftSvc<T>
                    {
                        type Response = super::super::structures::DShift;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DShift>,
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
                        tonic::server::UnaryService<super::super::structures::DWorkStation>
                        for SaveWorkStationSvc<T>
                    {
                        type Response = super::super::structures::DWorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DWorkStation>,
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
                        tonic::server::UnaryService<super::super::structures::DEquipmentClass>
                        for SaveEquipmentClassSvc<T>
                    {
                        type Response = super::super::structures::DEquipmentClass;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DEquipmentClass>,
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
                        tonic::server::UnaryService<super::super::structures::DPartClass>
                        for SavePartClassSvc<T>
                    {
                        type Response = super::super::structures::DPartClass;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DPartClass>,
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
                        tonic::server::UnaryService<super::super::structures::DPart>
                        for SavePartSvc<T>
                    {
                        type Response = super::super::structures::DPart;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DPart>,
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
                        tonic::server::UnaryService<super::super::structures::DLocation>
                        for SaveLocationSvc<T>
                    {
                        type Response = super::super::structures::DLocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DLocation>,
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
                        tonic::server::UnaryService<super::super::structures::DRoute>
                        for SaveRouteSvc<T>
                    {
                        type Response = super::super::structures::DRoute;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DRoute>,
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
                        tonic::server::UnaryService<super::super::structures::DRouteOperation>
                        for SaveRouteOperationSvc<T>
                    {
                        type Response = super::super::structures::DRouteOperation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DRouteOperation>,
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
                "/ObjectStorageService.ObjectStorageService/SaveEADefinition" => {
                    #[allow(non_camel_case_types)]
                    struct SaveEADefinitionSvc<T: ObjectStorageService>(pub Arc<T>);
                    impl<T: ObjectStorageService>
                        tonic::server::UnaryService<super::super::structures::DeaDefinition>
                        for SaveEADefinitionSvc<T>
                    {
                        type Response = super::super::structures::DeaDefinition;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DeaDefinition>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_ea_definition(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveEADefinitionSvc(inner);
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
                        tonic::server::UnaryService<super::super::structures::DDoc>
                        for SaveDocSvc<T>
                    {
                        type Response = super::super::structures::DDoc;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DDoc>,
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
                        tonic::server::UnaryService<super::super::structures::DCustomer>
                        for SaveCustomerSvc<T>
                    {
                        type Response = super::super::structures::DCustomer;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DCustomer>,
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
                        tonic::server::UnaryService<super::super::structures::DSupplier>
                        for SaveSupplierSvc<T>
                    {
                        type Response = super::super::structures::DSupplier;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DSupplier>,
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
