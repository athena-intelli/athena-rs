use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use athena_api::pb::structures::*;
use athena_api::pb::object_retrieval_service::object_retrieval_service_server::ObjectRetrievalService;

struct ObjectRetrievalServiceImpl;

#[tonic::async_trait]
impl ObjectRetrievalService for ObjectRetrievalServiceImpl {
    async fn get_factory_by_id(&self, request: Request<DObjectId>) -> Result<Response<DFactory>, Status> {
        todo!()
    }

    async fn get_factory_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DFactory>, Status> {
        todo!()
    }

    async fn get_workshop_by_id(&self, request: Request<DObjectId>) -> Result<Response<DWorkshop>, Status> {
        todo!()
    }

    async fn get_workshop_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DWorkshop>, Status> {
        todo!()
    }

    async fn get_production_line_by_id(&self, request: Request<DObjectId>) -> Result<Response<DProductionLine>, Status> {
        todo!()
    }

    async fn get_production_line_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DProductionLine>, Status> {
        todo!()
    }

    async fn get_work_station_by_id(&self, request: Request<DObjectId>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }

    async fn get_work_station_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DWorkStation>, Status> {
        todo!()
    }

    async fn get_equipment_by_id(&self, request: Request<DObjectId>) -> Result<Response<DEquipment>, Status> {
        todo!()
    }

    async fn get_equipment_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DEquipment>, Status> {
        todo!()
    }

    async fn get_part_by_id(&self, request: Request<DObjectId>) -> Result<Response<DPart>, Status> {
        todo!()
    }

    async fn get_part_by_number_and_revision(&self, request: Request<DPartNumberAndRevision>) -> Result<Response<DPart>, Status> {
        todo!()
    }

    async fn get_location_by_id(&self, request: Request<DObjectId>) -> Result<Response<DLocation>, Status> {
        todo!()
    }

    async fn get_location_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DLocation>, Status> {
        todo!()
    }

    async fn get_lot_by_id(&self, request: Request<DObjectId>) -> Result<Response<DLot>, Status> {
        todo!()
    }

    async fn get_lot_by_serial_number(&self, request: Request<DObjectSerialNumber>) -> Result<Response<DLot>, Status> {
        todo!()
    }

    async fn get_runtime_bom_by_id(&self, request: Request<DObjectId>) -> Result<Response<DRuntimeBom>, Status> {
        todo!()
    }

    async fn get_route_by_id(&self, request: Request<DObjectId>) -> Result<Response<DRoute>, Status> {
        todo!()
    }

    async fn get_route_by_code_and_version(&self, request: Request<DObjectCode>) -> Result<Response<DRoute>, Status> {
        todo!()
    }

    async fn get_operation_by_id(&self, request: Request<DObjectId>) -> Result<Response<DRouteOperation>, Status> {
        todo!()
    }

    async fn get_operation_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DRouteOperation>, Status> {
        todo!()
    }

    async fn get_unit_by_id(&self, request: Request<DObjectId>) -> Result<Response<DUnit>, Status> {
        todo!()
    }

    async fn get_unit_by_seria_number(&self, request: Request<DObjectSerialNumber>) -> Result<Response<DUnit>, Status> {
        todo!()
    }

    async fn get_user_by_id(&self, request: Request<DObjectId>) -> Result<Response<DUser>, Status> {
        todo!()
    }

    async fn get_user_by_name(&self, request: Request<DObjectName>) -> Result<Response<DUser>, Status> {
        todo!()
    }

    async fn get_work_order_by_id(&self, request: Request<DObjectId>) -> Result<Response<DWorkOrder>, Status> {
        todo!()
    }

    async fn get_work_order_by_number(&self, request: Request<DObjectName>) -> Result<Response<DWorkOrder>, Status> {
        todo!()
    }

    async fn get_doc_by_id(&self, request: Request<DObjectId>) -> Result<Response<DDoc>, Status> {
        todo!()
    }

    async fn get_doc_by_code(&self, request: Request<DObjectCode>) -> Result<Response<DDoc>, Status> {
        todo!()
    }

    type GetEquipmentsStream = ReceiverStream<Result<DEquipment, Status>>;

    async fn get_equipments(&self, request: Request<DFilter>) -> Result<Response<Self::GetEquipmentsStream>, Status> {
        todo!()
    }

    type GetWorkStationsStream = ReceiverStream<Result<DWorkStation, Status>>;

    async fn get_work_stations(&self, request: Request<DFilter>) -> Result<Response<Self::GetWorkStationsStream>, Status> {
        todo!()
    }

    type GetProductionLinesStream = ReceiverStream<Result<DProductionLine, Status>>;

    async fn get_production_lines(&self, request: Request<DFilter>) -> Result<Response<Self::GetProductionLinesStream>, Status> {
        todo!()
    }

    type GetWorkshopsStream = ReceiverStream<Result<DWorkshop, Status>>;

    async fn get_workshops(&self, request: Request<DFilter>) -> Result<Response<Self::GetWorkshopsStream>, Status> {
        todo!()
    }

    type GetFactoriesStream = ReceiverStream<Result<DFactory, Status>>;

    async fn get_factories(&self, request: Request<DFilter>) -> Result<Response<Self::GetFactoriesStream>, Status> {
        todo!()
    }

    type GetConsumePartsStream = ReceiverStream<Result<DConsumedPart, Status>>;

    async fn get_consume_parts(&self, request: Request<DFilter>) -> Result<Response<Self::GetConsumePartsStream>, Status> {
        todo!()
    }

    type GetBOMsStream = ReceiverStream<Result<DBom, Status>>;

    async fn get_bo_ms(&self, request: Request<DFilter>) -> Result<Response<Self::GetBOMsStream>, Status> {
        todo!()
    }

    type GetBOMItemsStream = ReceiverStream<Result<DBomItem, Status>>;

    async fn get_bom_items(&self, request: Request<DFilter>) -> Result<Response<Self::GetBOMItemsStream>, Status> {
        todo!()
    }

    type GetBoxesStream = ReceiverStream<Result<DBox, Status>>;

    async fn get_boxes(&self, request: Request<DFilter>) -> Result<Response<Self::GetBoxesStream>, Status> {
        todo!()
    }

    type GetLotsStream = ReceiverStream<Result<DLot, Status>>;

    async fn get_lots(&self, request: Request<DFilter>) -> Result<Response<Self::GetLotsStream>, Status> {
        todo!()
    }

    type GetUnitsStream = ReceiverStream<Result<DUnit, Status>>;

    async fn get_units(&self, request: Request<DFilter>) -> Result<Response<Self::GetUnitsStream>, Status> {
        todo!()
    }

    type GetPartsStream = ReceiverStream<Result<DPart, Status>>;

    async fn get_parts(&self, request: Request<DFilter>) -> Result<Response<Self::GetPartsStream>, Status> {
        todo!()
    }

    type GetRoutesStream = ReceiverStream<Result<DRoute, Status>>;

    async fn get_routes(&self, request: Request<DFilter>) -> Result<Response<Self::GetRoutesStream>, Status> {
        todo!()
    }

    type GetRouteStepsStream = ReceiverStream<Result<DRouteStep, Status>>;

    async fn get_route_steps(&self, request: Request<DFilter>) -> Result<Response<Self::GetRouteStepsStream>, Status> {
        todo!()
    }

    type GetRouteOperationsStream = ReceiverStream<Result<DRouteOperation, Status>>;

    async fn get_route_operations(&self, request: Request<DFilter>) -> Result<Response<Self::GetRouteOperationsStream>, Status> {
        todo!()
    }

    type GetRuntimeBomItemsStream = ReceiverStream<Result<DRuntimeBomItem, Status>>;

    async fn get_runtime_bom_items(&self, request: Request<DFilter>) -> Result<Response<Self::GetRuntimeBomItemsStream>, Status> {
        todo!()
    }

    type GetRuntimeBomsStream = ReceiverStream<Result<DRuntimeBom, Status>>;

    async fn get_runtime_boms(&self, request: Request<DFilter>) -> Result<Response<Self::GetRuntimeBomsStream>, Status> {
        todo!()
    }

    type GetWorkOrdersStream = ReceiverStream<Result<DWorkOrder, Status>>;

    async fn get_work_orders(&self, request: Request<DFilter>) -> Result<Response<Self::GetWorkOrdersStream>, Status> {
        todo!()
    }

    type GetUsersStream = ReceiverStream<Result<DUser, Status>>;

    async fn get_users(&self, request: Request<DFilter>) -> Result<Response<Self::GetUsersStream>, Status> {
        todo!()
    }

    type GetDocsStream = ReceiverStream<Result<DDoc, Status>>;

    async fn get_docs(&self, request: Request<DFilter>) -> Result<Response<Self::GetDocsStream>, Status> {
        todo!()
    }
}