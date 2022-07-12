use tonic::{Request, Response, Status};
use athena_api::pb::structures::{ChangePriorityRequest, DWorkStation};
use athena_api::pb::work_station_service::{AddToolRequest, RemoveToolRequest};
use athena_api::pb::work_station_service::work_station_service_server::WorkStationService;

#[derive(Default)]
pub struct WorkStationServiceImpl;

#[tonic::async_trait]
impl WorkStationService for WorkStationServiceImpl {
    async fn add_tools(&self, request: Request<AddToolRequest>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }

    async fn change_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }

    async fn remove_tools(&self, request: Request<RemoveToolRequest>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }
}