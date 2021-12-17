use tonic::{Request, Response, Status};
use athena_api::pb::structures::*;
use athena_api::pb::object_storage_service::object_storage_service_server::ObjectStorageService;
use athena_api::pb::object_storage_service::RemoveObjectRequest;

#[derive(Default)]
pub struct ObjectStorageServiceImpl;

#[tonic::async_trait]
impl ObjectStorageService for ObjectStorageServiceImpl {
    async fn remove_equipment(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_location(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_bom(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_part(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_production_line(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_production_queue(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_route(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_route_operation(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_station(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn remove_workshop(&self, request: Request<RemoveObjectRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn change_production_line_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<ProductionLine>, Status> {
        todo!()
    }

    async fn change_equipment_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<Equipment>, Status> {
        todo!()
    }

    async fn save_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn save_bom(&self, request: Request<Bom>) -> Result<Response<Bom>, Status> {
        todo!()
    }

    async fn save_runtime_bom(&self, request: Request<RuntimeBom>) -> Result<Response<RuntimeBom>, Status> {
        todo!()
    }

    async fn save_equipment(&self, request: Request<Equipment>) -> Result<Response<Equipment>, Status> {
        todo!()
    }

    async fn save_workshop(&self, request: Request<Workshop>) -> Result<Response<Workshop>, Status> {
        todo!()
    }

    async fn save_factory(&self, request: Request<Factory>) -> Result<Response<Factory>, Status> {
        todo!()
    }

    async fn save_shift(&self, request: Request<Shift>) -> Result<Response<Shift>, Status> {
        todo!()
    }

    async fn save_work_station(&self, request: Request<WorkStation>) -> Result<Response<WorkStation>, Status> {
        todo!()
    }

    async fn save_equipment_class(&self, request: Request<EquipmentClass>) -> Result<Response<EquipmentClass>, Status> {
        todo!()
    }

    async fn save_part_class(&self, request: Request<PartClass>) -> Result<Response<PartClass>, Status> {
        todo!()
    }

    async fn save_part(&self, request: Request<Part>) -> Result<Response<Part>, Status> {
        todo!()
    }

    async fn save_location(&self, request: Request<Location>) -> Result<Response<Location>, Status> {
        todo!()
    }

    async fn save_route(&self, request: Request<Route>) -> Result<Response<Route>, Status> {
        todo!()
    }

    async fn save_route_operation(&self, request: Request<RouteOperation>) -> Result<Response<RouteOperation>, Status> {
        todo!()
    }

    async fn save_doc(&self, request: Request<Doc>) -> Result<Response<Doc>, Status> {
        todo!()
    }

    async fn save_customer(&self, request: Request<Customer>) -> Result<Response<Customer>, Status> {
        todo!()
    }

    async fn save_supplier(&self, request: Request<Supplier>) -> Result<Response<Supplier>, Status> {
        todo!()
    }
}