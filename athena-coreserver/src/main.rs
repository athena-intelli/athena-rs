use tonic::transport::Server;
use athena_api::pb::lot_service::lot_service_server::LotServiceServer;
use athena_api::pb::object_retrieval_service::object_retrieval_service_server::ObjectRetrievalServiceServer;
use athena_api::pb::object_storage_service::object_storage_service_server::ObjectStorageServiceServer;
use athena_api::pb::package_service::package_service_server::PackageServiceServer;
use athena_api::pb::security_service::security_service_server::SecurityServiceServer;
use athena_api::pb::unit_service::unit_service_server::UnitServiceServer;
use athena_api::pb::work_order_service::work_order_service_server::WorkOrderServiceServer;
use athena_api::pb::work_station_service::work_station_service_server::WorkStationServiceServer;
use crate::services::package_service::{PackageServiceImpl};
use crate::services::lot_service::LotSerivceImpl;
use crate::services::object_retrieval_service::ObjectRetrievalServiceImpl;
use crate::services::object_storage_service::ObjectStorageServiceImpl;
use crate::services::security::SecurityServiceImpl;
use crate::services::unit_service::UnitServiceImpl;
use crate::services::work_order_service::WorkOrderServiceImpl;
use crate::services::work_station_service::WorkStationServiceImpl;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:9527".parse().unwrap();
    println!("Athena Server listening on {}", addr);
    let package_service = PackageServiceServer::new(PackageServiceImpl::default());
    let lot_service = LotServiceServer::new(LotSerivceImpl::default());
    let obj_retri_service = ObjectRetrievalServiceServer::new(ObjectRetrievalServiceImpl::default());
    let obj_storage_service = ObjectStorageServiceServer::new(ObjectStorageServiceImpl::default());
    let unit_service = UnitServiceServer::new(UnitServiceImpl::default());
    let order_service = WorkOrderServiceServer::new(WorkOrderServiceImpl::default());
    let station_service = WorkStationServiceServer::new(WorkStationServiceImpl::default());
    let security_service = SecurityServiceServer::new(SecurityServiceImpl::default());
    Server::builder()
        .add_service(security_service)
        .add_service(package_service)
        .add_service(lot_service)
        .add_service(obj_retri_service)
        .add_service(obj_storage_service)
        .add_service(unit_service)
        .add_service(order_service)
        .add_service(station_service)
        .serve(addr)
        .await?;

    Ok(())
}
