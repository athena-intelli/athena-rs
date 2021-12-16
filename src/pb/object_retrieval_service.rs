#[doc = r" Generated client implementations."]
pub mod object_retrieval_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ObjectRetrievalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectRetrievalServiceClient<tonic::transport::Channel> {
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
    impl<T> ObjectRetrievalServiceClient<T>
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
        ) -> ObjectRetrievalServiceClient<InterceptedService<T, F>>
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
            ObjectRetrievalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_factory_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DFactory>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactoryById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_factory_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DFactory>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactoryByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_workshop_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkshop>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshopById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_workshop_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DWorkshop>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshopByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_production_line_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DProductionLine>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLineById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_production_line_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DProductionLine>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLineByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_work_station_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStationById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_work_station_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DWorkStation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStationByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_equipment_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DEquipment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipmentById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_equipment_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DEquipment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipmentByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_part_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DPart>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetPartByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_part_by_number_and_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DPartNumberAndRevision>,
        ) -> Result<tonic::Response<super::super::dto::DPart>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetPartByNumberAndRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_location_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DLocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetLocationById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_location_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DLocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetLocationByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_lot_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DLot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetLotById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_lot_by_serial_number(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectSerialNumber>,
        ) -> Result<tonic::Response<super::super::dto::DLot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetLotBySerialNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_runtime_bom_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRuntimeBom>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBomById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_route_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRoute>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_route_by_code_and_version(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DRoute>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteByCodeAndVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_operation_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRouteOperation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetOperationById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_operation_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DRouteOperation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetOperationByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_unit_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnitById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_unit_by_seria_number(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectSerialNumber>,
        ) -> Result<tonic::Response<super::super::dto::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnitBySeriaNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DUser>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUserById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_user_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectName>,
        ) -> Result<tonic::Response<super::super::dto::DUser>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUserByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_work_order_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrderById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_work_order_by_number(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectName>,
        ) -> Result<tonic::Response<super::super::dto::DWorkOrder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrderByNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_doc_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DDoc>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_doc_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DDoc>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocByCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_equipments(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DEquipment>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipments",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_work_stations(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DWorkStation>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStations",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_production_lines(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DProductionLine>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLines",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_workshops(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DWorkshop>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshops",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_factories(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DFactory>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactories",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_consume_parts(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DConsumedPart>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetConsumeParts",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_bo_ms(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DBom>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetBOMs",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_bom_items(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DBomItem>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetBOMItems",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_boxes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DBox>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetBoxes",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_lots(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DLot>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetLots",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_units(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DUnit>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnits",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_parts(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DPart>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetParts",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DRoute>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRoutes",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_route_steps(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DRouteStep>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteSteps",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_route_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DRouteOperation>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteOperations",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_runtime_bom_items(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DRuntimeBomItem>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBomItems",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_runtime_boms(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DRuntimeBom>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBoms",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_work_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::dto::DWorkOrder>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrders",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_users(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DUser>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetUsers",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_docs(
            &mut self,
            request: impl tonic::IntoRequest<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::super::dto::DDoc>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocs",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_retrieval_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectRetrievalServiceServer."]
    #[async_trait]
    pub trait ObjectRetrievalService: Send + Sync + 'static {
        async fn get_factory_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DFactory>, tonic::Status>;
        async fn get_factory_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DFactory>, tonic::Status>;
        async fn get_workshop_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkshop>, tonic::Status>;
        async fn get_workshop_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DWorkshop>, tonic::Status>;
        async fn get_production_line_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DProductionLine>, tonic::Status>;
        async fn get_production_line_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DProductionLine>, tonic::Status>;
        async fn get_work_station_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkStation>, tonic::Status>;
        async fn get_work_station_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DWorkStation>, tonic::Status>;
        async fn get_equipment_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DEquipment>, tonic::Status>;
        async fn get_equipment_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DEquipment>, tonic::Status>;
        async fn get_part_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DPart>, tonic::Status>;
        async fn get_part_by_number_and_revision(
            &self,
            request: tonic::Request<super::super::dto::DPartNumberAndRevision>,
        ) -> Result<tonic::Response<super::super::dto::DPart>, tonic::Status>;
        async fn get_location_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DLocation>, tonic::Status>;
        async fn get_location_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DLocation>, tonic::Status>;
        async fn get_lot_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DLot>, tonic::Status>;
        async fn get_lot_by_serial_number(
            &self,
            request: tonic::Request<super::super::dto::DObjectSerialNumber>,
        ) -> Result<tonic::Response<super::super::dto::DLot>, tonic::Status>;
        async fn get_runtime_bom_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRuntimeBom>, tonic::Status>;
        async fn get_route_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRoute>, tonic::Status>;
        async fn get_route_by_code_and_version(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DRoute>, tonic::Status>;
        async fn get_operation_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DRouteOperation>, tonic::Status>;
        async fn get_operation_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DRouteOperation>, tonic::Status>;
        async fn get_unit_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DUnit>, tonic::Status>;
        async fn get_unit_by_seria_number(
            &self,
            request: tonic::Request<super::super::dto::DObjectSerialNumber>,
        ) -> Result<tonic::Response<super::super::dto::DUnit>, tonic::Status>;
        async fn get_user_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DUser>, tonic::Status>;
        async fn get_user_by_name(
            &self,
            request: tonic::Request<super::super::dto::DObjectName>,
        ) -> Result<tonic::Response<super::super::dto::DUser>, tonic::Status>;
        async fn get_work_order_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DWorkOrder>, tonic::Status>;
        async fn get_work_order_by_number(
            &self,
            request: tonic::Request<super::super::dto::DObjectName>,
        ) -> Result<tonic::Response<super::super::dto::DWorkOrder>, tonic::Status>;
        async fn get_doc_by_id(
            &self,
            request: tonic::Request<super::super::dto::DObjectId>,
        ) -> Result<tonic::Response<super::super::dto::DDoc>, tonic::Status>;
        async fn get_doc_by_code(
            &self,
            request: tonic::Request<super::super::dto::DObjectCode>,
        ) -> Result<tonic::Response<super::super::dto::DDoc>, tonic::Status>;
        #[doc = "Server streaming response type for the GetEquipments method."]
        type GetEquipmentsStream: futures_core::Stream<Item = Result<super::super::dto::DEquipment, tonic::Status>>
            + Send
            + 'static;
        async fn get_equipments(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetEquipmentsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetWorkStations method."]
        type GetWorkStationsStream: futures_core::Stream<Item = Result<super::super::dto::DWorkStation, tonic::Status>>
            + Send
            + 'static;
        async fn get_work_stations(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetWorkStationsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetProductionLines method."]
        type GetProductionLinesStream: futures_core::Stream<Item = Result<super::super::dto::DProductionLine, tonic::Status>>
            + Send
            + 'static;
        async fn get_production_lines(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetProductionLinesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetWorkshops method."]
        type GetWorkshopsStream: futures_core::Stream<Item = Result<super::super::dto::DWorkshop, tonic::Status>>
            + Send
            + 'static;
        async fn get_workshops(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetWorkshopsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetFactories method."]
        type GetFactoriesStream: futures_core::Stream<Item = Result<super::super::dto::DFactory, tonic::Status>>
            + Send
            + 'static;
        async fn get_factories(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetFactoriesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetConsumeParts method."]
        type GetConsumePartsStream: futures_core::Stream<Item = Result<super::super::dto::DConsumedPart, tonic::Status>>
            + Send
            + 'static;
        async fn get_consume_parts(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetConsumePartsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBOMs method."]
        type GetBOMsStream: futures_core::Stream<Item = Result<super::super::dto::DBom, tonic::Status>>
            + Send
            + 'static;
        async fn get_bo_ms(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetBOMsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBOMItems method."]
        type GetBOMItemsStream: futures_core::Stream<Item = Result<super::super::dto::DBomItem, tonic::Status>>
            + Send
            + 'static;
        async fn get_bom_items(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetBOMItemsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetBoxes method."]
        type GetBoxesStream: futures_core::Stream<Item = Result<super::super::dto::DBox, tonic::Status>>
            + Send
            + 'static;
        async fn get_boxes(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetBoxesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetLots method."]
        type GetLotsStream: futures_core::Stream<Item = Result<super::super::dto::DLot, tonic::Status>>
            + Send
            + 'static;
        async fn get_lots(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetLotsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetUnits method."]
        type GetUnitsStream: futures_core::Stream<Item = Result<super::super::dto::DUnit, tonic::Status>>
            + Send
            + 'static;
        async fn get_units(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetUnitsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetParts method."]
        type GetPartsStream: futures_core::Stream<Item = Result<super::super::dto::DPart, tonic::Status>>
            + Send
            + 'static;
        async fn get_parts(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetPartsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetRoutes method."]
        type GetRoutesStream: futures_core::Stream<Item = Result<super::super::dto::DRoute, tonic::Status>>
            + Send
            + 'static;
        async fn get_routes(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetRoutesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetRouteSteps method."]
        type GetRouteStepsStream: futures_core::Stream<Item = Result<super::super::dto::DRouteStep, tonic::Status>>
            + Send
            + 'static;
        async fn get_route_steps(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetRouteStepsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetRouteOperations method."]
        type GetRouteOperationsStream: futures_core::Stream<Item = Result<super::super::dto::DRouteOperation, tonic::Status>>
            + Send
            + 'static;
        async fn get_route_operations(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetRouteOperationsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetRuntimeBomItems method."]
        type GetRuntimeBomItemsStream: futures_core::Stream<Item = Result<super::super::dto::DRuntimeBomItem, tonic::Status>>
            + Send
            + 'static;
        async fn get_runtime_bom_items(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetRuntimeBomItemsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetRuntimeBoms method."]
        type GetRuntimeBomsStream: futures_core::Stream<Item = Result<super::super::dto::DRuntimeBom, tonic::Status>>
            + Send
            + 'static;
        async fn get_runtime_boms(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetRuntimeBomsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetWorkOrders method."]
        type GetWorkOrdersStream: futures_core::Stream<Item = Result<super::super::dto::DWorkOrder, tonic::Status>>
            + Send
            + 'static;
        async fn get_work_orders(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetWorkOrdersStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetUsers method."]
        type GetUsersStream: futures_core::Stream<Item = Result<super::super::dto::DUser, tonic::Status>>
            + Send
            + 'static;
        async fn get_users(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetUsersStream>, tonic::Status>;
        #[doc = "Server streaming response type for the GetDocs method."]
        type GetDocsStream: futures_core::Stream<Item = Result<super::super::dto::DDoc, tonic::Status>>
            + Send
            + 'static;
        async fn get_docs(
            &self,
            request: tonic::Request<super::super::dto::DFilter>,
        ) -> Result<tonic::Response<Self::GetDocsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ObjectRetrievalServiceServer<T: ObjectRetrievalService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectRetrievalService> ObjectRetrievalServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectRetrievalServiceServer<T>
    where
        T: ObjectRetrievalService,
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactoryById" => {
                    #[allow(non_camel_case_types)]
                    struct GetFactoryByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetFactoryByIdSvc<T>
                    {
                        type Response = super::super::dto::DFactory;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_factory_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFactoryByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactoryByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetFactoryByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetFactoryByCodeSvc<T>
                    {
                        type Response = super::super::dto::DFactory;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_factory_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFactoryByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshopById" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkshopByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetWorkshopByIdSvc<T>
                    {
                        type Response = super::super::dto::DWorkshop;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workshop_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkshopByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshopByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkshopByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetWorkshopByCodeSvc<T>
                    {
                        type Response = super::super::dto::DWorkshop;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workshop_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkshopByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLineById" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductionLineByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetProductionLineByIdSvc<T>
                    {
                        type Response = super::super::dto::DProductionLine;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_production_line_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProductionLineByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLineByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductionLineByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetProductionLineByCodeSvc<T>
                    {
                        type Response = super::super::dto::DProductionLine;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_production_line_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProductionLineByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStationById" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkStationByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetWorkStationByIdSvc<T>
                    {
                        type Response = super::super::dto::DWorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_work_station_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkStationByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStationByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkStationByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetWorkStationByCodeSvc<T>
                    {
                        type Response = super::super::dto::DWorkStation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_work_station_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkStationByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipmentById" => {
                    #[allow(non_camel_case_types)]
                    struct GetEquipmentByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetEquipmentByIdSvc<T>
                    {
                        type Response = super::super::dto::DEquipment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_equipment_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEquipmentByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipmentByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetEquipmentByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetEquipmentByCodeSvc<T>
                    {
                        type Response = super::super::dto::DEquipment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_equipment_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEquipmentByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetPartByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetPartByIDSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetPartByIDSvc<T>
                    {
                        type Response = super::super::dto::DPart;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_part_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPartByIDSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetPartByNumberAndRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetPartByNumberAndRevisionSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DPartNumberAndRevision>
                        for GetPartByNumberAndRevisionSvc<T>
                    {
                        type Response = super::super::dto::DPart;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DPartNumberAndRevision>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_part_by_number_and_revision(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPartByNumberAndRevisionSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetLocationById" => {
                    #[allow(non_camel_case_types)]
                    struct GetLocationByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetLocationByIdSvc<T>
                    {
                        type Response = super::super::dto::DLocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_location_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLocationByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetLocationByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetLocationByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetLocationByCodeSvc<T>
                    {
                        type Response = super::super::dto::DLocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_location_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLocationByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetLotById" => {
                    #[allow(non_camel_case_types)]
                    struct GetLotByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetLotByIdSvc<T>
                    {
                        type Response = super::super::dto::DLot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_lot_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLotByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetLotBySerialNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetLotBySerialNumberSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectSerialNumber>
                        for GetLotBySerialNumberSvc<T>
                    {
                        type Response = super::super::dto::DLot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectSerialNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_lot_by_serial_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLotBySerialNumberSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBomById" => {
                    #[allow(non_camel_case_types)]
                    struct GetRuntimeBomByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetRuntimeBomByIdSvc<T>
                    {
                        type Response = super::super::dto::DRuntimeBom;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_runtime_bom_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRuntimeBomByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteById" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetRouteByIdSvc<T>
                    {
                        type Response = super::super::dto::DRoute;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_route_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteByCodeAndVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteByCodeAndVersionSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetRouteByCodeAndVersionSvc<T>
                    {
                        type Response = super::super::dto::DRoute;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_route_by_code_and_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteByCodeAndVersionSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetOperationById" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetOperationByIdSvc<T>
                    {
                        type Response = super::super::dto::DRouteOperation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_operation_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOperationByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetOperationByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetOperationByCodeSvc<T>
                    {
                        type Response = super::super::dto::DRouteOperation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_operation_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOperationByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnitById" => {
                    #[allow(non_camel_case_types)]
                    struct GetUnitByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetUnitByIdSvc<T>
                    {
                        type Response = super::super::dto::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_unit_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUnitByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnitBySeriaNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetUnitBySeriaNumberSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectSerialNumber>
                        for GetUnitBySeriaNumberSvc<T>
                    {
                        type Response = super::super::dto::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectSerialNumber>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_unit_by_seria_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUnitBySeriaNumberSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetUserById" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetUserByIdSvc<T>
                    {
                        type Response = super::super::dto::DUser;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetUserByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserByNameSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectName>
                        for GetUserByNameSvc<T>
                    {
                        type Response = super::super::dto::DUser;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectName>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user_by_name(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserByNameSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrderById" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkOrderByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetWorkOrderByIdSvc<T>
                    {
                        type Response = super::super::dto::DWorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_work_order_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkOrderByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrderByNumber" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkOrderByNumberSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectName>
                        for GetWorkOrderByNumberSvc<T>
                    {
                        type Response = super::super::dto::DWorkOrder;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectName>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_work_order_by_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkOrderByNumberSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocById" => {
                    #[allow(non_camel_case_types)]
                    struct GetDocByIdSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectId>
                        for GetDocByIdSvc<T>
                    {
                        type Response = super::super::dto::DDoc;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_doc_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDocByIdSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocByCode" => {
                    #[allow(non_camel_case_types)]
                    struct GetDocByCodeSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::UnaryService<super::super::dto::DObjectCode>
                        for GetDocByCodeSvc<T>
                    {
                        type Response = super::super::dto::DDoc;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DObjectCode>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_doc_by_code(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDocByCodeSvc(inner);
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
                "/ObjectRetrievalService.ObjectRetrievalService/GetEquipments" => {
                    #[allow(non_camel_case_types)]
                    struct GetEquipmentsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetEquipmentsSvc<T>
                    {
                        type Response = super::super::dto::DEquipment;
                        type ResponseStream = T::GetEquipmentsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_equipments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEquipmentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkStations" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkStationsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetWorkStationsSvc<T>
                    {
                        type Response = super::super::dto::DWorkStation;
                        type ResponseStream = T::GetWorkStationsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_work_stations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkStationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetProductionLines" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductionLinesSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetProductionLinesSvc<T>
                    {
                        type Response = super::super::dto::DProductionLine;
                        type ResponseStream = T::GetProductionLinesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_production_lines(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProductionLinesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkshops" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkshopsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetWorkshopsSvc<T>
                    {
                        type Response = super::super::dto::DWorkshop;
                        type ResponseStream = T::GetWorkshopsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workshops(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkshopsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetFactories" => {
                    #[allow(non_camel_case_types)]
                    struct GetFactoriesSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetFactoriesSvc<T>
                    {
                        type Response = super::super::dto::DFactory;
                        type ResponseStream = T::GetFactoriesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_factories(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFactoriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetConsumeParts" => {
                    #[allow(non_camel_case_types)]
                    struct GetConsumePartsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetConsumePartsSvc<T>
                    {
                        type Response = super::super::dto::DConsumedPart;
                        type ResponseStream = T::GetConsumePartsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_consume_parts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConsumePartsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetBOMs" => {
                    #[allow(non_camel_case_types)]
                    struct GetBOMsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetBOMsSvc<T>
                    {
                        type Response = super::super::dto::DBom;
                        type ResponseStream = T::GetBOMsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_bo_ms(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBOMsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetBOMItems" => {
                    #[allow(non_camel_case_types)]
                    struct GetBOMItemsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetBOMItemsSvc<T>
                    {
                        type Response = super::super::dto::DBomItem;
                        type ResponseStream = T::GetBOMItemsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_bom_items(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBOMItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetBoxes" => {
                    #[allow(non_camel_case_types)]
                    struct GetBoxesSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetBoxesSvc<T>
                    {
                        type Response = super::super::dto::DBox;
                        type ResponseStream = T::GetBoxesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_boxes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBoxesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetLots" => {
                    #[allow(non_camel_case_types)]
                    struct GetLotsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetLotsSvc<T>
                    {
                        type Response = super::super::dto::DLot;
                        type ResponseStream = T::GetLotsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_lots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetUnits" => {
                    #[allow(non_camel_case_types)]
                    struct GetUnitsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetUnitsSvc<T>
                    {
                        type Response = super::super::dto::DUnit;
                        type ResponseStream = T::GetUnitsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_units(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUnitsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetParts" => {
                    #[allow(non_camel_case_types)]
                    struct GetPartsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetPartsSvc<T>
                    {
                        type Response = super::super::dto::DPart;
                        type ResponseStream = T::GetPartsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_parts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPartsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct GetRoutesSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetRoutesSvc<T>
                    {
                        type Response = super::super::dto::DRoute;
                        type ResponseStream = T::GetRoutesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_routes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRoutesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteSteps" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteStepsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetRouteStepsSvc<T>
                    {
                        type Response = super::super::dto::DRouteStep;
                        type ResponseStream = T::GetRouteStepsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_route_steps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteStepsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetRouteOperations" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteOperationsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetRouteOperationsSvc<T>
                    {
                        type Response = super::super::dto::DRouteOperation;
                        type ResponseStream = T::GetRouteOperationsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_route_operations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBomItems" => {
                    #[allow(non_camel_case_types)]
                    struct GetRuntimeBomItemsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetRuntimeBomItemsSvc<T>
                    {
                        type Response = super::super::dto::DRuntimeBomItem;
                        type ResponseStream = T::GetRuntimeBomItemsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_runtime_bom_items(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRuntimeBomItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetRuntimeBoms" => {
                    #[allow(non_camel_case_types)]
                    struct GetRuntimeBomsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetRuntimeBomsSvc<T>
                    {
                        type Response = super::super::dto::DRuntimeBom;
                        type ResponseStream = T::GetRuntimeBomsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_runtime_boms(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRuntimeBomsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetWorkOrders" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkOrdersSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetWorkOrdersSvc<T>
                    {
                        type Response = super::super::dto::DWorkOrder;
                        type ResponseStream = T::GetWorkOrdersStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_work_orders(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkOrdersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetUsers" => {
                    #[allow(non_camel_case_types)]
                    struct GetUsersSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetUsersSvc<T>
                    {
                        type Response = super::super::dto::DUser;
                        type ResponseStream = T::GetUsersStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_users(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUsersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ObjectRetrievalService.ObjectRetrievalService/GetDocs" => {
                    #[allow(non_camel_case_types)]
                    struct GetDocsSvc<T: ObjectRetrievalService>(pub Arc<T>);
                    impl<T: ObjectRetrievalService>
                        tonic::server::ServerStreamingService<super::super::dto::DFilter>
                        for GetDocsSvc<T>
                    {
                        type Response = super::super::dto::DDoc;
                        type ResponseStream = T::GetDocsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::dto::DFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_docs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDocsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: ObjectRetrievalService> Clone for ObjectRetrievalServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectRetrievalService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectRetrievalService> tonic::transport::NamedService for ObjectRetrievalServiceServer<T> {
        const NAME: &'static str = "ObjectRetrievalService.ObjectRetrievalService";
    }
}