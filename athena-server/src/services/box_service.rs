use tonic::{Request, Response, Status};
use athena_api::pb::box_service::box_service_server::BoxService;
use athena_api::pb::box_service::*;
use athena_api::pb::structures::DBox;

struct BoxServiceImpl;

#[tonic::async_trait]
impl BoxService for BoxServiceImpl {
    async fn add_box_to_box(&self, request: Request<HandleBoxRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn add_lot_to_box(&self, request: Request<HandleLotRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn add_box_to_queue(&self, request: Request<AddBoxToQueueRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn add_unit_to_box(&self, request: Request<HandleUnitRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn change_production_line(&self, request: Request<ChangePLineRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn change_route(&self, request: Request<BoxChangeRouteRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn complete_at_route_step(&self, request: Request<BoxCompleteRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn start_at_route_step(&self, request: Request<BoxStartRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn finish(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn hold(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn open(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn pause(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn release(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn scrap(&self, request: Request<BoxTransactionRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn remove_all(&self, request: Request<ClearBoxRequest>) -> Result<Response<DBox>, Status> {
        todo!()
    }

    async fn remove_box(&self, request: Request<HandleBoxRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_lot(&self, request: Request<HandleLotRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_unit(&self, request: Request<HandleUnitRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn save_box(&self, request: Request<DBox>) -> Result<Response<DBox>, Status> {
        todo!()
    }
}