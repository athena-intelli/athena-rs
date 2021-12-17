use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use athena_api::pb::structures::{ChangePriorityRequest, Unit};
use athena_api::pb::unit_service::*;
use athena_api::pb::unit_service::unit_service_server::UnitService;

#[derive(Default)]
pub struct UnitServiceImpl;

#[tonic::async_trait]
impl UnitService for UnitServiceImpl {
    async fn add_to_queue(&self, request: Request<AddUnitToQueueRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn save_unit(&self, request: Request<Unit>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn cancel(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn finish(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn hold(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn pause(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn quarantine(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn release(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn ship(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn scrap(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn undo_close(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn undo_finish(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn undo_scrap(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn undo_ship(&self, request: Request<TransitionUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_bom(&self, request: Request<UnitChangeBomRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_bom_from_part(&self, request: Request<UnitChangeBomFromPartRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_lot(&self, request: Request<ChangeLotRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_part(&self, request: Request<UnitChangePartRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_part_only(&self, request: Request<UnitChangePartRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_production_line(&self, request: Request<ChangeProductionLineRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_route(&self, request: Request<UnitChangeRouteRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn change_serial_number(&self, request: Request<ChangeSerialNumberRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn complete_at_route_step(&self, request: Request<UnitCompleteAtRouteStepRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn start_at_route_step(&self, request: Request<UnitStartAtRouteStepRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn restart(&self, request: Request<RestartUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn create_stand_alone_unit(&self, request: Request<CreateStandAloneUnitRequest>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    type CreateStandAloneUnitsStream = ReceiverStream<Result<Unit, Status>>;

    async fn create_stand_alone_units(&self, request: Request<CreateStandAloneUnitRequest>) -> Result<Response<Self::CreateStandAloneUnitsStream>, Status> {
        todo!()
    }
}