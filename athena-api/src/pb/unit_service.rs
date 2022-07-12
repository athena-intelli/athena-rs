#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStandAloneUnitRequest {
    #[prost(string, tag = "1")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub route_id: u64,
    #[prost(int32, tag = "7")]
    pub priority: i32,
    #[prost(string, tag = "8")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "10")]
    pub quantity: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartUnitRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitStartAtRouteStepRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
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
pub struct UnitCompleteAtRouteStepRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
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
pub struct ChangeSerialNumberRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(string, tag = "2")]
    pub new_serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitChangeRouteRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
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
pub struct ChangeProductionLineRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(uint64, tag = "2")]
    pub new_production_line_id: u64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitChangePartRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
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
pub struct ChangeLotRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(uint64, tag = "2")]
    pub new_lot_id: u64,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitChangeBomRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
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
pub struct UnitChangeBomFromPartRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(string, tag = "2")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub bom: ::core::option::Option<super::structures::DBom>,
    #[prost(string, tag = "5")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionUnitRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "TransitionUnitState", tag = "5")]
    pub transition_state: i32,
    #[prost(bool, tag = "6")]
    pub override_route_enforcement: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUnitToQueueRequest {
    #[prost(uint64, tag = "1")]
    pub unit_id: u64,
    #[prost(string, tag = "2")]
    pub route_queue_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransitionUnitState {
    UnitClose = 0,
    UnitFinish = 1,
    UnitHold = 2,
    UnitOpen = 3,
    UnitQuarantine = 4,
    UnitRelease = 5,
    UnitShip = 6,
    UnitUndoClose = 7,
    UnitUndoShip = 8,
    UnitUndoFinish = 9,
    UnitScrap = 10,
    UnitUndoScrap = 11,
    UnitCancle = 12,
}
#[doc = r" Generated client implementations."]
pub mod unit_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UnitServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UnitServiceClient<tonic::transport::Channel> {
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
    impl<T> UnitServiceClient<T>
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
        ) -> UnitServiceClient<InterceptedService<T, F>>
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
            UnitServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn add_to_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUnitToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/AddToQueue");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::DUnit>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/SaveUnit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Cancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Close");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn finish(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Finish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Pause");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn quarantine(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Quarantine");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Release");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn ship(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Ship");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Scrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_close(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/UndoClose");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_finish(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/UndoFinish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_scrap(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/UndoScrap");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undo_ship(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/UndoShip");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_bom(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitChangeBomRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangeBom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_bom_from_part(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitChangeBomFromPartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangeBomFromPart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_lot(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangeLot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_part(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangePart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_part_only(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangePartOnly");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangePriority");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_production_line(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeProductionLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/UnitService.UnitService/ChangeProductionLine",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_route(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangeRoute");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn change_serial_number(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeSerialNumberRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/UnitService.UnitService/ChangeSerialNumber");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn complete_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitCompleteAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/UnitService.UnitService/CompleteAtRouteStep",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn start_at_route_step(
            &mut self,
            request: impl tonic::IntoRequest<super::UnitStartAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/UnitService.UnitService/StartAtRouteStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restart(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UnitService.UnitService/Restart");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_stand_alone_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStandAloneUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/UnitService.UnitService/CreateStandAloneUnit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_stand_alone_units(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStandAloneUnitRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::structures::DUnit>>,
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
                "/UnitService.UnitService/CreateStandAloneUnits",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod unit_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UnitServiceServer."]
    #[async_trait]
    pub trait UnitService: Send + Sync + 'static {
        async fn add_to_queue(
            &self,
            request: tonic::Request<super::AddUnitToQueueRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn save_unit(
            &self,
            request: tonic::Request<super::super::structures::DUnit>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn cancel(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn close(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn finish(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn hold(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn pause(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn quarantine(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn release(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn ship(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn scrap(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn undo_close(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn undo_finish(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn undo_scrap(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn undo_ship(
            &self,
            request: tonic::Request<super::TransitionUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_bom(
            &self,
            request: tonic::Request<super::UnitChangeBomRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_bom_from_part(
            &self,
            request: tonic::Request<super::UnitChangeBomFromPartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_lot(
            &self,
            request: tonic::Request<super::ChangeLotRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_part(
            &self,
            request: tonic::Request<super::UnitChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_part_only(
            &self,
            request: tonic::Request<super::UnitChangePartRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_priority(
            &self,
            request: tonic::Request<super::super::structures::ChangePriorityRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_production_line(
            &self,
            request: tonic::Request<super::ChangeProductionLineRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_route(
            &self,
            request: tonic::Request<super::UnitChangeRouteRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn change_serial_number(
            &self,
            request: tonic::Request<super::ChangeSerialNumberRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn complete_at_route_step(
            &self,
            request: tonic::Request<super::UnitCompleteAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn start_at_route_step(
            &self,
            request: tonic::Request<super::UnitStartAtRouteStepRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn restart(
            &self,
            request: tonic::Request<super::RestartUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        async fn create_stand_alone_unit(
            &self,
            request: tonic::Request<super::CreateStandAloneUnitRequest>,
        ) -> Result<tonic::Response<super::super::structures::DUnit>, tonic::Status>;
        #[doc = "Server streaming response type for the CreateStandAloneUnits method."]
        type CreateStandAloneUnitsStream: futures_core::Stream<Item = Result<super::super::structures::DUnit, tonic::Status>>
            + Send
            + 'static;
        async fn create_stand_alone_units(
            &self,
            request: tonic::Request<super::CreateStandAloneUnitRequest>,
        ) -> Result<tonic::Response<Self::CreateStandAloneUnitsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UnitServiceServer<T: UnitService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UnitService> UnitServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UnitServiceServer<T>
    where
        T: UnitService,
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
                "/UnitService.UnitService/AddToQueue" => {
                    #[allow(non_camel_case_types)]
                    struct AddToQueueSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::AddUnitToQueueRequest>
                        for AddToQueueSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUnitToQueueRequest>,
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
                "/UnitService.UnitService/SaveUnit" => {
                    #[allow(non_camel_case_types)]
                    struct SaveUnitSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::super::structures::DUnit>
                        for SaveUnitSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::structures::DUnit>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_unit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveUnitSvc(inner);
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
                "/UnitService.UnitService/Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for CancelSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for CloseSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Finish" => {
                    #[allow(non_camel_case_types)]
                    struct FinishSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for FinishSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for HoldSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Pause" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for PauseSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Quarantine" => {
                    #[allow(non_camel_case_types)]
                    struct QuarantineSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest>
                        for QuarantineSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Release" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for ReleaseSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Ship" => {
                    #[allow(non_camel_case_types)]
                    struct ShipSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for ShipSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/Scrap" => {
                    #[allow(non_camel_case_types)]
                    struct ScrapSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for ScrapSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/UndoClose" => {
                    #[allow(non_camel_case_types)]
                    struct UndoCloseSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for UndoCloseSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/UndoFinish" => {
                    #[allow(non_camel_case_types)]
                    struct UndoFinishSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest>
                        for UndoFinishSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/UndoScrap" => {
                    #[allow(non_camel_case_types)]
                    struct UndoScrapSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for UndoScrapSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/UndoShip" => {
                    #[allow(non_camel_case_types)]
                    struct UndoShipSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::TransitionUnitRequest> for UndoShipSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionUnitRequest>,
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
                "/UnitService.UnitService/ChangeBom" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeBomSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::UnitChangeBomRequest> for ChangeBomSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitChangeBomRequest>,
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
                "/UnitService.UnitService/ChangeBomFromPart" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeBomFromPartSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::UnitChangeBomFromPartRequest>
                        for ChangeBomFromPartSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitChangeBomFromPartRequest>,
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
                "/UnitService.UnitService/ChangeLot" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeLotSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::ChangeLotRequest> for ChangeLotSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeLotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_lot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeLotSvc(inner);
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
                "/UnitService.UnitService/ChangePart" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePartSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::UnitChangePartRequest>
                        for ChangePartSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitChangePartRequest>,
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
                "/UnitService.UnitService/ChangePartOnly" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePartOnlySvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::UnitChangePartRequest>
                        for ChangePartOnlySvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitChangePartRequest>,
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
                "/UnitService.UnitService/ChangePriority" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePrioritySvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::super::structures::ChangePriorityRequest>
                        for ChangePrioritySvc<T>
                    {
                        type Response = super::super::structures::DUnit;
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
                "/UnitService.UnitService/ChangeProductionLine" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProductionLineSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::ChangeProductionLineRequest>
                        for ChangeProductionLineSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeProductionLineRequest>,
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
                "/UnitService.UnitService/ChangeRoute" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeRouteSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::UnitChangeRouteRequest>
                        for ChangeRouteSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitChangeRouteRequest>,
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
                "/UnitService.UnitService/ChangeSerialNumber" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeSerialNumberSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::ChangeSerialNumberRequest>
                        for ChangeSerialNumberSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeSerialNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_serial_number(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeSerialNumberSvc(inner);
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
                "/UnitService.UnitService/CompleteAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAtRouteStepSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::UnitCompleteAtRouteStepRequest>
                        for CompleteAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitCompleteAtRouteStepRequest>,
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
                "/UnitService.UnitService/StartAtRouteStep" => {
                    #[allow(non_camel_case_types)]
                    struct StartAtRouteStepSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::UnitStartAtRouteStepRequest>
                        for StartAtRouteStepSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnitStartAtRouteStepRequest>,
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
                "/UnitService.UnitService/Restart" => {
                    #[allow(non_camel_case_types)]
                    struct RestartSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService> tonic::server::UnaryService<super::RestartUnitRequest> for RestartSvc<T> {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestartUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).restart(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestartSvc(inner);
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
                "/UnitService.UnitService/CreateStandAloneUnit" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStandAloneUnitSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::UnaryService<super::CreateStandAloneUnitRequest>
                        for CreateStandAloneUnitSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStandAloneUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_stand_alone_unit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateStandAloneUnitSvc(inner);
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
                "/UnitService.UnitService/CreateStandAloneUnits" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStandAloneUnitsSvc<T: UnitService>(pub Arc<T>);
                    impl<T: UnitService>
                        tonic::server::ServerStreamingService<super::CreateStandAloneUnitRequest>
                        for CreateStandAloneUnitsSvc<T>
                    {
                        type Response = super::super::structures::DUnit;
                        type ResponseStream = T::CreateStandAloneUnitsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStandAloneUnitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_stand_alone_units(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateStandAloneUnitsSvc(inner);
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
    impl<T: UnitService> Clone for UnitServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UnitService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UnitService> tonic::transport::NamedService for UnitServiceServer<T> {
        const NAME: &'static str = "UnitService.UnitService";
    }
}
