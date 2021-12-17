#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUnitResponse {
    #[prost(message, optional, tag = "1")]
    pub lot: ::core::option::Option<super::structures::Lot>,
    #[prost(message, repeated, tag = "2")]
    pub units: ::prost::alloc::vec::Vec<super::structures::Unit>,
    #[prost(message, optional, tag = "3")]
    pub order: ::core::option::Option<super::structures::WorkOrder>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeLotRequest {
    #[prost(uint64, tag = "1")]
    pub parent_lot_id: u64,
    #[prost(uint64, tag = "2")]
    pub child_lot_id: u64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeUnitRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(uint64, tag = "2")]
    pub unit_id: u64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotStartAtRouteStepRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
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
pub struct LotCompleteAtRouteStepRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
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
pub struct SplitLotRequest {
    #[prost(uint64, tag = "1")]
    pub parent_lot_id: u64,
    #[prost(string, tag = "2")]
    pub child_lot_serial_number: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub child_lot_quantity: f64,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitLotResponse {
    #[prost(message, repeated, tag = "1")]
    pub lots: ::prost::alloc::vec::Vec<super::structures::Lot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionLotRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "TransitionLotState", tag = "5")]
    pub transition_state: i32,
    #[prost(bool, tag = "6")]
    pub override_route_enforcement: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangeQuantityRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(double, tag = "2")]
    pub quantity: f64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangeRouteRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(uint64, tag = "2")]
    pub new_route_id: u64,
    #[prost(bool, tag = "3")]
    pub force_change_route: bool,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangeProductionLineRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(uint64, tag = "2")]
    pub new_production_line_id: u64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangePartRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangeBomRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LotChangeBomFromPartRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub bom: ::core::option::Option<super::structures::Bom>,
    #[prost(string, tag = "5")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOneUnitRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub unit_serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLotToQueueRequest {
    #[prost(uint64, tag = "1")]
    pub lot_id: u64,
    #[prost(string, tag = "2")]
    pub route_queue_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransitionLotState {
    LotClose = 0,
    LotFinish = 1,
    LotHold = 2,
    LotOpen = 3,
    LotQuarantine = 4,
    LotRelease = 5,
    LotShip = 6,
    LotUndoClose = 7,
    LotUndoShip = 8,
    LotUndoFinish = 9,
    LotScrap = 10,
    LotUndoScrap = 11,
    LotCancle = 12,
    LotSerialize = 13,
    LotUndoSerialize = 14,
}
#[doc = r" Generated client implementations."]
pub mod lot_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LotServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LotServiceClient<tonic::transport::Channel> {
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
    impl<T> LotServiceClient<T>
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
        ) -> LotServiceClient<InterceptedService<T, F>>
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
            LotServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Cancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Close");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn finish(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Finish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Pause");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn quarantine(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Quarantine");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Release");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn ship(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Ship");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Scrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/UndoClose");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_finish(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/UndoFinish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/UndoScrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_ship(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/UndoShip");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn serialize(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/Serialize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_serialize(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/UndoSerialize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_one_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOneUnitRequest>,
        ) -> Result<tonic::Response<super::AddUnitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/AddOneUnit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_to_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::AddLotToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/AddToQueue");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_bom(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangeBomRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/ChangeBom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_bom_from_part(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangeBomFromPartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/ChangeBomFromPart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_part(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/ChangePart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_part_only(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/ChangePartOnly");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/ChangePriority");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_production_line(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangeProductionLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/ChangeProductionLine");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_route(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/ChangeRoute");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_quantity(
            &mut self,
            request: impl tonic::IntoRequest<super::LotChangeQuantityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/ChangeQuantity");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn split_lot(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitLotRequest>,
        ) -> Result<tonic::Response<super::SplitLotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/SplitLot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn complete_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::LotCompleteAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/CompleteAtRouteStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn start_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::LotStartAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/LotService.LotService/StartAtRouteStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn merge_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/MergeUnit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn merge_lot(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeLotRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::structures::Lot>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LotService.LotService/MergeLot");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod lot_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LotServiceServer."]
    #[async_trait]
    pub trait LotService: Send + Sync + 'static {
        async fn cancel(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn close(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn finish(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn hold(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn pause(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn quarantine(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn release(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn ship(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn scrap(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn undo_close(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn undo_finish(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn undo_scrap(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn undo_ship(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn serialize(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn undo_serialize(
            &self,
            request: tonic::Request<super::TransitionLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn add_one_unit(
            &self,
            request: tonic::Request<super::AddOneUnitRequest>,
        ) -> Result<tonic::Response<super::AddUnitResponse>, tonic::Status>;
        async fn add_to_queue(
            &self,
            request: tonic::Request<super::AddLotToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_bom(
            &self,
            request: tonic::Request<super::LotChangeBomRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_bom_from_part(
            &self,
            request: tonic::Request<super::LotChangeBomFromPartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_part(
            &self,
            request: tonic::Request<super::LotChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_part_only(
            &self,
            request: tonic::Request<super::LotChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_production_line(
            &self,
            request: tonic::Request<super::LotChangeProductionLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_route(
            &self,
            request: tonic::Request<super::LotChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn change_quantity(
            &self,
            request: tonic::Request<super::LotChangeQuantityRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn split_lot(
            &self,
            request: tonic::Request<super::SplitLotRequest>,
        ) -> Result<tonic::Response<super::SplitLotResponse>, tonic::Status>;
        async fn complete_at_route_step(
            &self,
            request: tonic::Request<super::LotCompleteAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn start_at_route_step(
            &self,
            request: tonic::Request<super::LotStartAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        async fn merge_unit(
            &self,
            request: tonic::Request<super::MergeUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::Lot>, tonic::Status>;
        #[doc = "Server streaming response type for the MergeLot method."]
        type MergeLotStream: futures_core::Stream<Item = Result<super::super::structures::Lot, tonic::Status>>
            + Send
            + 'static;
        async fn merge_lot(
            &self,
            request: tonic::Request<super::MergeLotRequest>,
        ) -> Result<tonic::Response<Self::MergeLotStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LotServiceServer<T: LotService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LotService> LotServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LotServiceServer<T>
    where
        T: LotService,
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
                "/LotService.LotService/Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for CancelSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelSvc(inner);
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
                "/LotService.LotService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for CloseSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Finish" => {
                    #[allow(non_camel_case_types)]
                    struct FinishSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for FinishSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for HoldSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Pause" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for PauseSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Quarantine" => {
                    #[allow(non_camel_case_types)]
                    struct QuarantineSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for QuarantineSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Release" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for ReleaseSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Ship" => {
                    #[allow(non_camel_case_types)]
                    struct ShipSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for ShipSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/Scrap" => {
                    #[allow(non_camel_case_types)]
                    struct ScrapSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for ScrapSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/UndoClose" => {
                    #[allow(non_camel_case_types)]
                    struct UndoCloseSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for UndoCloseSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
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
                "/LotService.LotService/UndoFinish" => {
                    #[allow(non_camel_case_types)]
                    struct UndoFinishSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for UndoFinishSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undo_finish(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndoFinishSvc(inner);
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
                "/LotService.LotService/UndoScrap" => {
                    #[allow(non_camel_case_types)]
                    struct UndoScrapSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for UndoScrapSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undo_scrap(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndoScrapSvc(inner);
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
                "/LotService.LotService/UndoShip" => {
                    #[allow(non_camel_case_types)]
                    struct UndoShipSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for UndoShipSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undo_ship(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndoShipSvc(inner);
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
                "/LotService.LotService/Serialize" => {
                    #[allow(non_camel_case_types)]
                    struct SerializeSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest> for SerializeSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).serialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SerializeSvc(inner);
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
                "/LotService.LotService/UndoSerialize" => {
                    #[allow(non_camel_case_types)]
                    struct UndoSerializeSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::TransitionLotRequest>
                        for UndoSerializeSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undo_serialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndoSerializeSvc(inner);
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
                "/LotService.LotService/AddOneUnit" => {
                    #[allow(non_camel_case_types)]
                    struct AddOneUnitSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::AddOneUnitRequest> for AddOneUnitSvc<T> {
                        type Response = super::AddUnitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddOneUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_one_unit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddOneUnitSvc(inner);
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
                "/LotService.LotService/AddToQueue" => {
                    #[allow(non_camel_case_types)]
                    struct AddToQueueSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::AddLotToQueueRequest> for AddToQueueSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddLotToQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_to_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddToQueueSvc(inner);
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
                "/LotService.LotService/ChangeBom" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeBomSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::LotChangeBomRequest> for ChangeBomSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangeBomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_bom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeBomSvc(inner);
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
                "/LotService.LotService/ChangeBomFromPart" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeBomFromPartSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::UnaryService<super::LotChangeBomFromPartRequest>
                        for ChangeBomFromPartSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangeBomFromPartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_bom_from_part(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeBomFromPartSvc(inner);
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
                "/LotService.LotService/ChangePart" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePartSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::LotChangePartRequest> for ChangePartSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangePartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_part(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangePartSvc(inner);
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
                "/LotService.LotService/ChangePartOnly" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePartOnlySvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::LotChangePartRequest>
                        for ChangePartOnlySvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangePartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_part_only(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangePartOnlySvc(inner);
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
                "/LotService.LotService/ChangePriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePrioritySvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangePrioritySvc<T>
                    {
                        type Response = super::super::structures::Lot;
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
                "/LotService.LotService/ChangeProductionLine" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProductionLineSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::UnaryService<super::LotChangeProductionLineRequest>
                        for ChangeProductionLineSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangeProductionLineRequest>,
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
                "/LotService.LotService/ChangeRoute" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeRouteSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::LotChangeRouteRequest>
                        for ChangeRouteSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangeRouteRequest>,
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
                "/LotService.LotService/ChangeQuantity" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeQuantitySvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::LotChangeQuantityRequest>
                        for ChangeQuantitySvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotChangeQuantityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_quantity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeQuantitySvc(inner);
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
                "/LotService.LotService/SplitLot" => {
                    #[allow(non_camel_case_types)]
                    struct SplitLotSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::SplitLotRequest> for SplitLotSvc<T> {
                        type Response = super::SplitLotResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SplitLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).split_lot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SplitLotSvc(inner);
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
                "/LotService.LotService/CompleteAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAtRouteStepSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::UnaryService<super::LotCompleteAtRouteStepRequest>
                        for CompleteAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotCompleteAtRouteStepRequest>,
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
                "/LotService.LotService/StartAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct StartAtRouteStepSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::UnaryService<super::LotStartAtRouteStepRequest>
                        for StartAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LotStartAtRouteStepRequest>,
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
                "/LotService.LotService/MergeUnit" => {
                    #[allow(non_camel_case_types)]
                    struct MergeUnitSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService> tonic::server::UnaryService<super::MergeUnitRequest> for MergeUnitSvc<T> {
                        type Response = super::super::structures::Lot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).merge_unit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MergeUnitSvc(inner);
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
                "/LotService.LotService/MergeLot" => {
                    #[allow(non_camel_case_types)]
                    struct MergeLotSvc<T: LotService>(pub Arc<T>);
                    impl<T: LotService>
                        tonic::server::ServerStreamingService<super::MergeLotRequest>
                        for MergeLotSvc<T>
                    {
                        type Response = super::super::structures::Lot;
                        type ResponseStream = T::MergeLotStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).merge_lot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MergeLotSvc(inner);
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
    impl<T: LotService> Clone for LotServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: LotService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LotService> tonic::transport::NamedService for LotServiceServer<T> {
        const NAME: &'static str = "LotService.LotService";
    }
}
