use tonic::{Request, Response, Status};
use athena_api::pb::structures::{ChangePriorityRequest, WorkOrder};
use athena_api::pb::work_order_service::{CompleteOrderRequest, DeleteOrderRequest, StartOrderRequest, TransitionOrderRequest};
use athena_api::pb::work_order_service::work_order_service_server::WorkOrderService;

#[derive(Default)]
pub struct WorkOrderServiceImpl;

#[tonic::async_trait]
impl WorkOrderService for WorkOrderServiceImpl{
    async fn change_order_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn finish(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn hold(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn open(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn quarantine(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn release(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn delete_order(&self, request: Request<DeleteOrderRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn save_work_order(&self, request: Request<WorkOrder>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn ship(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn undo_close(&self, request: Request<TransitionOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn start_at_route_step(&self, request: Request<StartOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn complete_at_route_step(&self, request: Request<CompleteOrderRequest>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }
}