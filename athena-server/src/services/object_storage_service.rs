use tonic::{Request, Response, Status};
use athena_api::pb::structures::*;
use athena_api::pb::object_storage_service::object_storage_service_server::ObjectStorageService;
use athena_api::pb::object_storage_service::RemoveObjectRequest;

struct ObjectStorageServiceImpl;

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

    async fn change_production_line_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<DProductionLine>, Status> {
        todo!()
    }

    async fn change_equipment_priority(&self, request: Request<ChangePriorityRequest>) -> Result<Response<DEquipment>, Status> {
        todo!()
    }

    async fn save_user(&self, request: Request<DUser>) -> Result<Response<DUser>, Status> {
        todo!()
    }

    async fn save_bom(&self, request: Request<DBom>) -> Result<Response<DBom>, Status> {
        todo!()
    }

    async fn save_runtime_bom(&self, request: Request<DRuntimeBom>) -> Result<Response<DRuntimeBom>, Status> {
        todo!()
    }

    async fn save_equipment(&self, request: Request<DEquipment>) -> Result<Response<DEquipment>, Status> {
        todo!()
    }

    async fn save_workshop(&self, request: Request<DWorkshop>) -> Result<Response<DWorkshop>, Status> {
        todo!()
    }

    async fn save_factory(&self, request: Request<DFactory>) -> Result<Response<DFactory>, Status> {
        todo!()
    }

    async fn save_shift(&self, request: Request<DShift>) -> Result<Response<DShift>, Status> {
        todo!()
    }

    async fn save_work_station(&self, request: Request<DWorkStation>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }

    async fn save_equipment_class(&self, request: Request<DEquipmentClass>) -> Result<Response<DEquipmentClass>, Status> {
        todo!()
    }

    async fn save_part_class(&self, request: Request<DPartClass>) -> Result<Response<DPartClass>, Status> {
        todo!()
    }

    async fn save_part(&self, request: Request<DPart>) -> Result<Response<DPart>, Status> {
        todo!()
    }

    async fn save_location(&self, request: Request<DLocation>) -> Result<Response<DLocation>, Status> {
        todo!()
    }

    async fn save_route(&self, request: Request<DRoute>) -> Result<Response<DRoute>, Status> {
        todo!()
    }

    async fn save_route_operation(&self, request: Request<DRouteOperation>) -> Result<Response<DRouteOperation>, Status> {
        todo!()
    }

    async fn save_ea_definition(&self, request: Request<DeaDefinition>) -> Result<Response<DeaDefinition>, Status> {
        todo!()
    }

    async fn save_doc(&self, request: Request<DDoc>) -> Result<Response<DDoc>, Status> {
        todo!()
    }

    async fn save_customer(&self, request: Request<DCustomer>) -> Result<Response<DCustomer>, Status> {
        todo!()
    }

    async fn save_supplier(&self, request: Request<DSupplier>) -> Result<Response<DSupplier>, Status> {
        todo!()
    }
}