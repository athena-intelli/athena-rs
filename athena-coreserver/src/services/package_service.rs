use tonic::{Request, Response, Status};
use athena_api::pb::package_service::*;
use athena_api::pb::package_service::package_service_server::PackageService;
use athena_api::pb::structures::DContainer;

#[derive(Default)]
pub(crate) struct PackageServiceImpl;

#[tonic::async_trait]
impl PackageService for PackageServiceImpl {
    async fn add_container_to_container(&self, request: Request<HandleContainerRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn add_lot_to_container(&self, request: Request<HandleLotRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn add_container_to_queue(&self, request: Request<AddContainerToQueueRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn add_unit_to_container(&self, request: Request<HandleUnitRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn change_production_line(&self, request: Request<ChangePLineRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn change_route(&self, request: Request<ContainerChangeRouteRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn complete_at_route_step(&self, request: Request<ContainerCompleteRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn start_at_route_step(&self, request: Request<ContainerStartRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn finish(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn hold(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn open(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn pause(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn release(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn scrap(&self, request: Request<ContainerTransactionRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn remove_all(&self, request: Request<ClearContainerRequest>) -> Result<Response<DContainer>, Status> {
        todo!()
    }

    async fn remove_container(&self, request: Request<HandleContainerRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_lot(&self, request: Request<HandleLotRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_unit(&self, request: Request<HandleUnitRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn save_container(&self, request: Request<DContainer>) -> Result<Response<DContainer>, Status> {
        todo!()
    }
}