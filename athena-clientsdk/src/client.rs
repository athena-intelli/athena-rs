use athena_api::pb::object_retrieval_service::object_retrieval_service_client::ObjectRetrievalServiceClient;
use athena_api::pb::object_storage_service::object_storage_service_client::ObjectStorageServiceClient;
use athena_api::pb::package_service::package_service_client::PackageServiceClient;
use athena_api::pb::security_service::security_service_client::SecurityServiceClient;
use athena_api::pb::work_order_service::work_order_service_client::WorkOrderServiceClient;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct AthenaClient<T = Channel> {
    security_service_client: SecurityServiceClient<T>,
    obj_storage_service_clinet: ObjectStorageServiceClient<T>,
    obj_retrieval_service_clinet: ObjectRetrievalServiceClient<T>,
    workorder_service_clinet: WorkOrderServiceClient<T>,
    package_service_clinet: PackageServiceClient<T>,
}


impl AthenaClient {
    pub fn init() -> AthenaClient {
        let endpoints = ["http://[::1]:9527"]
            .iter()
            .map(|a| Channel::from_static(a));

        let channel = Channel::balance_list(endpoints);
        let security_service_client = SecurityServiceClient::new(channel.clone());
        let obj_storage_service_clinet = ObjectStorageServiceClient::new(channel.clone());
        let obj_retrieval_service_clinet = ObjectRetrievalServiceClient::new(channel.clone());
        let workorder_service_clinet = WorkOrderServiceClient::new(channel.clone());
        let package_service_clinet = PackageServiceClient::new(channel.clone());
        AthenaClient {
            security_service_client,
            obj_storage_service_clinet,
            obj_retrieval_service_clinet,
            workorder_service_clinet,
            package_service_clinet
        }
    }


    pub fn security_service_client(&self) -> &SecurityServiceClient<Channel> {
        &self.security_service_client
    }
    pub fn obj_storage_service_clinet(&self) -> &ObjectStorageServiceClient<Channel> {
        &self.obj_storage_service_clinet
    }
    pub fn obj_retrieval_service_clinet(&self) -> &ObjectRetrievalServiceClient<Channel> {
        &self.obj_retrieval_service_clinet
    }
    pub fn workorder_service_clinet(&self) -> &WorkOrderServiceClient<Channel> {
        &self.workorder_service_clinet
    }
    pub fn package_service_clinet(&self) -> &PackageServiceClient<Channel> {
        &self.package_service_clinet
    }
}