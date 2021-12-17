use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use athena_api::pb::structures::{ChangePriorityRequest, Lot};
use athena_api::pb::lot_service::lot_service_server::LotService;
use athena_api::pb::lot_service::*;

#[derive(Default)]
pub struct LotSerivceImpl;

#[tonic::async_trait]
impl LotService for LotSerivceImpl {
    async fn cancel(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn finish(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn hold(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn pause(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn quarantine(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn release(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn ship(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn scrap(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn undo_close(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn undo_finish(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn undo_scrap(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn undo_ship(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn serialize(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn undo_serialize(&self, request: Request<TransitionLotRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn add_one_unit(&self, request: Request<AddOneUnitRequest>) -> Result<Response<AddUnitResponse>, Status> {
        todo!()
    }

    async fn add_to_queue(&self, request: Request<AddLotToQueueRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_bom(&self, request: Request<LotChangeBomRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_bom_from_part(&self, request: Request<LotChangeBomFromPartRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_part(&self, request: Request<LotChangePartRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_part_only(&self, request: Request<LotChangePartRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_production_line(&self, request: Request<LotChangeProductionLineRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_route(&self, request: Request<LotChangeRouteRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn change_quantity(&self, request: Request<LotChangeQuantityRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn split_lot(&self, request: Request<SplitLotRequest>) -> Result<Response<SplitLotResponse>, Status> {
        todo!()
    }

    async fn complete_at_route_step(&self, request: Request<LotCompleteAtRouteStepRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn start_at_route_step(&self, request: Request<LotStartAtRouteStepRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn merge_unit(&self, request: Request<MergeUnitRequest>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    type MergeLotStream = ReceiverStream<Result<Lot, Status>>;

    async fn merge_lot(&self, request: Request<MergeLotRequest>) -> Result<Response<Self::MergeLotStream>, Status> {
        todo!()
    }
}