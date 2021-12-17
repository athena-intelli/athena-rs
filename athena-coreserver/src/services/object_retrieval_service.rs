use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use athena_api::pb::structures::*;
use athena_api::pb::object_retrieval_service::object_retrieval_service_server::ObjectRetrievalService;
use athena_api::pb::object_retrieval_service::PartNumberAndRevisionRequest;

#[derive(Default)]
pub struct ObjectRetrievalServiceImpl;

#[tonic::async_trait]
impl ObjectRetrievalService for ObjectRetrievalServiceImpl {
    async fn get_factory_by_id(&self, request: Request<ObjectId>) -> Result<Response<Factory>, Status> {
        todo!()
    }

    async fn get_factory_by_code(&self, request: Request<ObjectCode>) -> Result<Response<Factory>, Status> {
        todo!()
    }

    async fn get_workshop_by_id(&self, request: Request<ObjectId>) -> Result<Response<Workshop>, Status> {
        todo!()
    }

    async fn get_workshop_by_code(&self, request: Request<ObjectCode>) -> Result<Response<Workshop>, Status> {
        todo!()
    }

    async fn get_production_line_by_id(&self, request: Request<ObjectId>) -> Result<Response<ProductionLine>, Status> {
        todo!()
    }

    async fn get_production_line_by_code(&self, request: Request<ObjectCode>) -> Result<Response<ProductionLine>, Status> {
        todo!()
    }

    async fn get_work_station_by_id(&self, request: Request<ObjectId>) -> Result<Response<WorkStation>, Status> {
        todo!()
    }

    async fn get_work_station_by_code(&self, request: Request<ObjectCode>) -> Result<Response<WorkStation>, Status> {
        todo!()
    }

    async fn get_equipment_by_id(&self, request: Request<ObjectId>) -> Result<Response<Equipment>, Status> {
        todo!()
    }

    async fn get_equipment_by_code(&self, request: Request<ObjectCode>) -> Result<Response<Equipment>, Status> {
        todo!()
    }

    async fn get_part_by_id(&self, request: Request<ObjectId>) -> Result<Response<Part>, Status> {
        todo!()
    }

    async fn get_part_by_number_and_revision(&self, request: Request<PartNumberAndRevisionRequest>) -> Result<Response<Part>, Status> {
        todo!()
    }

    async fn get_location_by_id(&self, request: Request<ObjectId>) -> Result<Response<Location>, Status> {
        todo!()
    }

    async fn get_location_by_code(&self, request: Request<ObjectCode>) -> Result<Response<Location>, Status> {
        todo!()
    }

    async fn get_lot_by_id(&self, request: Request<ObjectId>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn get_lot_by_serial_number(&self, request: Request<ObjectSerialNumber>) -> Result<Response<Lot>, Status> {
        todo!()
    }

    async fn get_runtime_bom_by_id(&self, request: Request<ObjectId>) -> Result<Response<RuntimeBom>, Status> {
        todo!()
    }

    async fn get_route_by_id(&self, request: Request<ObjectId>) -> Result<Response<Route>, Status> {
        todo!()
    }

    async fn get_route_by_code_and_version(&self, request: Request<ObjectCode>) -> Result<Response<Route>, Status> {
        todo!()
    }

    async fn get_operation_by_id(&self, request: Request<ObjectId>) -> Result<Response<RouteOperation>, Status> {
        todo!()
    }

    async fn get_operation_by_code(&self, request: Request<ObjectCode>) -> Result<Response<RouteOperation>, Status> {
        todo!()
    }

    async fn get_unit_by_id(&self, request: Request<ObjectId>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn get_unit_by_seria_number(&self, request: Request<ObjectSerialNumber>) -> Result<Response<Unit>, Status> {
        todo!()
    }

    async fn get_user_by_id(&self, request: Request<ObjectId>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn get_user_by_name(&self, request: Request<ObjectName>) -> Result<Response<User>, Status> {
        todo!()
    }

    async fn get_work_order_by_id(&self, request: Request<ObjectId>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn get_work_order_by_number(&self, request: Request<ObjectName>) -> Result<Response<WorkOrder>, Status> {
        todo!()
    }

    async fn get_doc_by_id(&self, request: Request<ObjectId>) -> Result<Response<Doc>, Status> {
        todo!()
    }

    async fn get_doc_by_code(&self, request: Request<ObjectCode>) -> Result<Response<Doc>, Status> {
        todo!()
    }

    type GetEquipmentsStream = ReceiverStream<Result<Equipment, Status>>;

    async fn get_equipments(&self, request: Request<Filter>) -> Result<Response<Self::GetEquipmentsStream>, Status> {
        todo!()
    }

    type GetWorkStationsStream = ReceiverStream<Result<WorkStation, Status>>;

    async fn get_work_stations(&self, request: Request<Filter>) -> Result<Response<Self::GetWorkStationsStream>, Status> {
        todo!()
    }

    type GetProductionLinesStream = ReceiverStream<Result<ProductionLine, Status>>;

    async fn get_production_lines(&self, request: Request<Filter>) -> Result<Response<Self::GetProductionLinesStream>, Status> {
        todo!()
    }

    type GetWorkshopsStream = ReceiverStream<Result<Workshop, Status>>;

    async fn get_workshops(&self, request: Request<Filter>) -> Result<Response<Self::GetWorkshopsStream>, Status> {
        todo!()
    }

    type GetFactoriesStream = ReceiverStream<Result<Factory, Status>>;

    async fn get_factories(&self, request: Request<Filter>) -> Result<Response<Self::GetFactoriesStream>, Status> {
        todo!()
    }

    type GetConsumePartsStream = ReceiverStream<Result<ConsumedPart, Status>>;

    async fn get_consume_parts(&self, request: Request<Filter>) -> Result<Response<Self::GetConsumePartsStream>, Status> {
        todo!()
    }

    type GetBOMsStream = ReceiverStream<Result<Bom, Status>>;

    async fn get_bo_ms(&self, request: Request<Filter>) -> Result<Response<Self::GetBOMsStream>, Status> {
        todo!()
    }

    type GetBOMItemsStream = ReceiverStream<Result<BomItem, Status>>;

    async fn get_bom_items(&self, request: Request<Filter>) -> Result<Response<Self::GetBOMItemsStream>, Status> {
        todo!()
    }

    type GetContainersStream = ReceiverStream<Result<Container, Status>>;

    async fn get_containers(&self, request: Request<Filter>) -> Result<Response<Self::GetContainersStream>, Status> {
        todo!()
    }


    type GetLotsStream = ReceiverStream<Result<Lot, Status>>;

    async fn get_lots(&self, request: Request<Filter>) -> Result<Response<Self::GetLotsStream>, Status> {
        todo!()
    }

    type GetUnitsStream = ReceiverStream<Result<Unit, Status>>;

    async fn get_units(&self, request: Request<Filter>) -> Result<Response<Self::GetUnitsStream>, Status> {
        todo!()
    }

    type GetPartsStream = ReceiverStream<Result<Part, Status>>;

    async fn get_parts(&self, request: Request<Filter>) -> Result<Response<Self::GetPartsStream>, Status> {
        todo!()
    }

    type GetRoutesStream = ReceiverStream<Result<Route, Status>>;

    async fn get_routes(&self, request: Request<Filter>) -> Result<Response<Self::GetRoutesStream>, Status> {
        todo!()
    }

    type GetRouteStepsStream = ReceiverStream<Result<RouteStep, Status>>;

    async fn get_route_steps(&self, request: Request<Filter>) -> Result<Response<Self::GetRouteStepsStream>, Status> {
        todo!()
    }

    type GetRouteOperationsStream = ReceiverStream<Result<RouteOperation, Status>>;

    async fn get_route_operations(&self, request: Request<Filter>) -> Result<Response<Self::GetRouteOperationsStream>, Status> {
        todo!()
    }

    type GetRuntimeBomItemsStream = ReceiverStream<Result<RuntimeBomItem, Status>>;

    async fn get_runtime_bom_items(&self, request: Request<Filter>) -> Result<Response<Self::GetRuntimeBomItemsStream>, Status> {
        todo!()
    }

    type GetRuntimeBomsStream = ReceiverStream<Result<RuntimeBom, Status>>;

    async fn get_runtime_boms(&self, request: Request<Filter>) -> Result<Response<Self::GetRuntimeBomsStream>, Status> {
        todo!()
    }

    type GetWorkOrdersStream = ReceiverStream<Result<WorkOrder, Status>>;

    async fn get_work_orders(&self, request: Request<Filter>) -> Result<Response<Self::GetWorkOrdersStream>, Status> {
        todo!()
    }

    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn get_users(&self, request: Request<Filter>) -> Result<Response<Self::GetUsersStream>, Status> {
        todo!()
    }

    type GetDocsStream = ReceiverStream<Result<Doc, Status>>;

    async fn get_docs(&self, request: Request<Filter>) -> Result<Response<Self::GetDocsStream>, Status> {
        todo!()
    }
}