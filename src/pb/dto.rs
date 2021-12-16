#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DMeasuredValue {
    #[prost(string, tag = "1")]
    pub uom: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub value: f64,
    #[prost(int32, tag = "3")]
    pub scale: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DObjectId {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DObjectCode {
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DObjectName {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DPartNumberAndRevision {
    #[prost(string, tag = "1")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub part_revision: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DObjectSerialNumber {
    #[prost(string, tag = "1")]
    pub serial_number: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeaInstance {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<DeaInstanceItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeaInstanceItem {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub obj_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub item_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub item_value: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub list_supported: bool,
    #[prost(uint64, tag = "6")]
    pub list_id: u64,
    #[prost(string, repeated, tag = "7")]
    pub list_value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///*
/// User Data Transfer Object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DUser {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub full_name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::enums::UserStatus", tag = "14")]
    pub status: i32,
    #[prost(uint32, tag = "15")]
    pub failed_login_attempt_count: u32,
    #[prost(uint32, tag = "16")]
    pub login_count: u32,
    #[prost(bool, tag = "17")]
    pub password_changed: bool,
    #[prost(message, optional, tag = "18")]
    pub password_expiration: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "19")]
    pub password_modifiable: bool,
    #[prost(message, optional, tag = "22")]
    pub shift: ::core::option::Option<DShift>,
    #[prost(message, optional, tag = "23")]
    pub user_expiration: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "24")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "25")]
    pub user_group_users: ::prost::alloc::vec::Vec<DUserGroupUser>,
    #[prost(message, repeated, tag = "26")]
    pub user_roles: ::prost::alloc::vec::Vec<DUserRole>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DUserGroup {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub email: ::prost::alloc::string::String,
    #[prost(uint64, tag = "13")]
    pub shift_id: u64,
    #[prost(message, repeated, tag = "14")]
    pub user_group_users: ::prost::alloc::vec::Vec<DUserGroupUser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DUserGroupUser {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub group_id: u64,
    #[prost(uint64, tag = "11")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRole {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub role_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "12")]
    pub user_roles: ::prost::alloc::vec::Vec<DUserRole>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DUserRole {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub role_id: u64,
    #[prost(uint64, tag = "11")]
    pub user_id: u64,
    #[prost(message, optional, tag = "12")]
    pub effective_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "13")]
    pub effective_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DShift {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub shift_code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(bool, tag = "15")]
    pub over_day: bool,
    #[prost(message, repeated, tag = "16")]
    pub shift_durations: ::prost::alloc::vec::Vec<DShiftDuration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DShiftDuration {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub shift_id: u64,
    #[prost(string, tag = "11")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub over_day: bool,
    #[prost(string, tag = "14")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub factory_code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub creator_id: u64,
    #[prost(message, optional, tag = "6")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "7")]
    pub last_modifier_id: u64,
    #[prost(message, optional, tag = "8")]
    pub last_modified_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "super::enums::ModificationFlag", tag = "9")]
    pub modification_flag: i32,
    #[prost(string, tag = "10")]
    pub workshop_code: ::prost::alloc::string::String,
}
///*
/// Factory Data Transfer Object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DFactory {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, repeated, tag = "10")]
    pub workshops: ::prost::alloc::vec::Vec<DWorkshop>,
    #[prost(string, tag = "11")]
    pub time_zone: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub location_id: u64,
    #[prost(uint64, tag = "13")]
    pub work_schedule_id: u64,
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
///*
/// Workshop Data Transfer Object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DWorkshop {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, repeated, tag = "10")]
    pub workshops: ::prost::alloc::vec::Vec<DWorkshop>,
    #[prost(uint64, tag = "11")]
    pub location_id: u64,
    #[prost(string, tag = "12")]
    pub time_zone: ::prost::alloc::string::String,
    #[prost(uint64, tag = "13")]
    pub work_schedule_id: u64,
    #[prost(string, tag = "14")]
    pub workshop_code: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "16")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DLocation {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub location_code: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DBom {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub bom_type: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub uom: ::prost::alloc::string::String,
    #[prost(bool, tag = "14")]
    pub bom_item_changed: bool,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DBomItem {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub bom_id: u64,
    #[prost(string, tag = "11")]
    pub consumption_type: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(double, tag = "14")]
    pub quantity: f64,
    #[prost(bool, tag = "15")]
    pub record_consumption: bool,
    #[prost(string, tag = "16")]
    pub uom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "17")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "18")]
    pub alternate_items: ::prost::alloc::vec::Vec<DAlternateBomItem>,
    #[prost(string, tag = "19")]
    pub position_number: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DAlternateBomItem {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub bom_id: u64,
    #[prost(uint64, tag = "12")]
    pub bom_item_id: u64,
    #[prost(double, tag = "13")]
    pub max_replacement_percent: f64,
    #[prost(string, tag = "14")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub part_revision: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRuntimeBom {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "11")]
    pub bom_items: ::prost::alloc::vec::Vec<DRuntimeBomItem>,
    #[prost(bool, tag = "12")]
    pub bom_items_changed: bool,
    #[prost(uint64, tag = "13")]
    pub tracked_object_id: u64,
    #[prost(string, tag = "14")]
    pub tracked_object_type: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub uom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRuntimeBomItem {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub bom_id: u64,
    #[prost(string, tag = "11")]
    pub consumption_type: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(double, tag = "14")]
    pub quantity: f64,
    #[prost(bool, tag = "15")]
    pub record_consumption: bool,
    #[prost(string, tag = "16")]
    pub uom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "17")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "18")]
    pub tracked_object_id: u64,
    #[prost(string, tag = "19")]
    pub tracked_object_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DDoc {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "9")]
    pub doc_code: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub doc_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "12")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub doc_type: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub class_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DEquipment {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub capacity: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub identity: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "12")]
    pub child_equipment_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "13")]
    pub location_id: u64,
    #[prost(string, tag = "14")]
    pub location_code: ::prost::alloc::string::String,
    #[prost(bool, tag = "15")]
    pub tool: bool,
    #[prost(uint64, tag = "16")]
    pub work_station_id: u64,
    #[prost(string, tag = "17")]
    pub work_station_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "18")]
    pub work_schedule_id: u64,
    #[prost(uint64, repeated, tag = "19")]
    pub doc_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag = "20")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "21")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, optional, tag = "22")]
    pub equipment_class: ::core::option::Option<DEquipmentClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DEquipmentClass {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub class_code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DWorkStation {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub station_code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub production_line_id: u64,
    #[prost(message, optional, tag = "13")]
    pub location: ::core::option::Option<DLocation>,
    #[prost(message, optional, tag = "14")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DTrackable {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub tracked_object_status_id: u64,
    #[prost(string, tag = "11")]
    pub operation_code: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub production_line_code: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub route_queue_name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub production_queue_name: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub route_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "18")]
    pub route_id: u64,
    #[prost(string, tag = "19")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub route_step_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "21")]
    pub route_step_id: u64,
    #[prost(
        enumeration = "d_trackable::TrackedObjectTransactionStatus",
        tag = "22"
    )]
    pub transaction_status: i32,
    #[prost(enumeration = "d_trackable::TrackedObjectTransactionState", tag = "23")]
    pub transaction_state: i32,
    #[prost(string, tag = "24")]
    pub work_station_code: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub equipment_identity: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub location_code: ::prost::alloc::string::String,
    #[prost(string, tag = "27")]
    pub location_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DTrackable`.
pub mod d_trackable {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrackedObjectTransactionStatus {
        StatusCreated = 0,
        StatusStarted = 1,
        StatusCompleted = 2,
        StatusFinished = 3,
        StatusReceived = 4,
        StatusShipped = 5,
        StatusClosed = 6,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrackedObjectTransactionState {
        StateNormal = 0,
        StateConsume = 1,
        StateDestructivesample = 2,
        StateHold = 3,
        StateQuarantine = 4,
        StatePause = 5,
        StateRevive = 6,
        StateScrap = 7,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DBox {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    /// the actual number of boxes in a Box
    #[prost(uint32, tag = "10")]
    pub box_count: u32,
    /// the number of allowed items in this box
    #[prost(uint32, tag = "11")]
    pub capacity: u32,
    #[prost(string, tag = "13")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(uint32, tag = "14")]
    pub lot_count: u32,
    #[prost(uint32, tag = "15")]
    pub current_quantity: u32,
    #[prost(enumeration = "super::enums::BoxType", tag = "16")]
    pub r#type: i32,
    #[prost(uint32, tag = "17")]
    pub unit_count: u32,
    #[prost(message, optional, tag = "18")]
    pub finished_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "19")]
    pub priority: u32,
    #[prost(message, optional, tag = "20")]
    pub shipped_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "21")]
    pub previous_node_name: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub previous_node_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "23")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, optional, tag = "24")]
    pub tracked_data: ::core::option::Option<DTrackable>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DLot {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, optional, tag = "11")]
    pub trackable: ::core::option::Option<DTrackable>,
    #[prost(string, tag = "12")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub order_number: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "17")]
    pub runtime_bom_id: u64,
    #[prost(string, tag = "18")]
    pub container_serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub container_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "20")]
    pub good: bool,
    #[prost(uint32, tag = "21")]
    pub priority: u32,
    #[prost(uint32, tag = "22")]
    pub rework_count: u32,
    #[prost(message, optional, tag = "23")]
    pub tracked_data: ::core::option::Option<DTrackable>,
    #[prost(message, optional, tag = "24")]
    pub finished_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "25")]
    pub closed_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "26")]
    pub promised_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "27")]
    pub shipped_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "28")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "29")]
    pub scrapped: bool,
    #[prost(bool, tag = "30")]
    pub consumed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DUnit {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub order_number: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "13")]
    pub container_serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub container_type: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub lot_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "16")]
    pub parent_lot_id: u64,
    #[prost(uint64, tag = "18")]
    pub parent_unit_id: u64,
    #[prost(string, tag = "19")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "21")]
    pub runtime_bom_id: u64,
    #[prost(message, optional, tag = "22")]
    pub finished_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "23")]
    pub good: bool,
    #[prost(uint64, tag = "24")]
    pub order_id: u64,
    #[prost(string, tag = "25")]
    pub previous_node_name: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub previous_node_type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "27")]
    pub priority: u32,
    #[prost(message, optional, tag = "28")]
    pub promised_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag = "29")]
    pub rework_count: u32,
    #[prost(message, optional, tag = "30")]
    pub shipped_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "31")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "32")]
    pub tracked_data: ::core::option::Option<DTrackable>,
    #[prost(bool, tag = "33")]
    pub scrapped: bool,
    #[prost(bool, tag = "34")]
    pub consumed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRoute {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub route_version: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "13")]
    pub effective_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub effective_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, repeated, tag = "16")]
    pub production_line_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "17")]
    pub default_production_line_id: u64,
    #[prost(message, repeated, tag = "18")]
    pub route_arcs: ::prost::alloc::vec::Vec<DRouteArc>,
    #[prost(message, repeated, tag = "19")]
    pub route_steps: ::prost::alloc::vec::Vec<DRouteStep>,
    #[prost(message, repeated, tag = "20")]
    pub route_queues: ::prost::alloc::vec::Vec<DRouteQueue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRouteStep {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub step_number: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub step_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "13")]
    pub cycle_duration: u32,
    #[prost(enumeration = "super::enums::CycleDurationType", tag = "14")]
    pub r#type: i32,
    #[prost(uint32, tag = "15")]
    pub enforcement: u32,
    /// true: Means this Route Step must be included.
    /// false: Means this Route Step need not be included.
    #[prost(bool, tag = "16")]
    pub failure: bool,
    #[prost(uint64, tag = "17")]
    pub operation_id: u64,
    #[prost(string, tag = "18")]
    pub operation_code: ::prost::alloc::string::String,
    #[prost(uint32, tag = "19")]
    pub pixel_x: u32,
    #[prost(uint32, tag = "20")]
    pub pixel_y: u32,
    #[prost(enumeration = "super::enums::RouteStepType", tag = "21")]
    pub step_type: i32,
    ///Primary keys of Equipments assigned to the Route Step.
    #[prost(uint64, repeated, tag = "23")]
    pub equipment_ids: ::prost::alloc::vec::Vec<u64>,
    ///Primary keys of Work stations assigned to the Route Step.
    #[prost(uint64, repeated, tag = "24")]
    pub work_station_ids: ::prost::alloc::vec::Vec<u64>,
    ///Primary keys of Production Lines associated with this Route Step.
    #[prost(uint64, repeated, tag = "25")]
    pub production_line_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "26")]
    pub embedded_route_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRouteQueue {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(bool, tag = "11")]
    pub auto_start: bool,
    #[prost(uint32, tag = "12")]
    pub queue_duration: u32,
    #[prost(uint32, tag = "13")]
    pub est_duration_to_completion: u32,
    #[prost(bool, tag = "14")]
    pub production_queue_assigned: bool,
    #[prost(double, tag = "15")]
    pub queue_capacity: f64,
    #[prost(enumeration = "super::enums::RouteQueueType", tag = "16")]
    pub r#type: i32,
    #[prost(uint32, tag = "17")]
    pub pixel_x: u32,
    #[prost(uint32, tag = "18")]
    pub pixel_y: u32,
    #[prost(uint64, repeated, tag = "19")]
    pub production_queue_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "20")]
    pub production_line_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "21")]
    pub route_id: u64,
    #[prost(string, tag = "23")]
    pub route_queue_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRouteArc {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(enumeration = "super::enums::RouteArcNodeType", tag = "10")]
    pub to_node_type: i32,
    #[prost(string, tag = "11")]
    pub to_node_name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::enums::RouteArcNodeType", tag = "12")]
    pub from_node_type: i32,
    #[prost(string, tag = "13")]
    pub from_node_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "14")]
    pub main_path: bool,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DRouteOperation {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub failure: bool,
    ///The time (in minutes) it takes to complete the Operation.
    #[prost(uint32, tag = "14")]
    pub cycle_duration: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DPart {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub bom_id: u64,
    #[prost(string, tag = "13")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub consumption_type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "16")]
    pub shelf_life: u32,
    #[prost(string, tag = "17")]
    pub uom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "18")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "19")]
    pub part_classes: ::prost::alloc::vec::Vec<DPartClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DAlternatePart {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub parent_id: u64,
    #[prost(string, tag = "11")]
    pub parent_type: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(double, tag = "15")]
    pub max_replacement_percent: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DAlternatePartGroup {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub parent_id: u64,
    #[prost(string, tag = "11")]
    pub parent_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "12")]
    pub alternate_parts: ::prost::alloc::vec::Vec<DAlternatePart>,
    #[prost(message, optional, tag = "13")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(double, tag = "14")]
    pub max_replacement_percent: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DPartClass {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DCustomer {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub contact: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "16")]
    pub status: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DSupplier {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub contact: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "15")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DWorkOrder {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub order_number: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub closed_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "12")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "13")]
    pub finished_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub promised_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub shipped_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "16")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "17")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "18")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "22")]
    pub part_id: u64,
    #[prost(uint64, tag = "23")]
    pub bom_id: u64,
    #[prost(double, tag = "24")]
    pub quatity: f64,
    #[prost(string, tag = "26")]
    pub planned_production_line_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "27")]
    pub planned_route_id: u64,
    #[prost(double, tag = "28")]
    pub precise_quantity_closed: f64,
    #[prost(double, tag = "29")]
    pub precise_quantity_finished: f64,
    #[prost(double, tag = "30")]
    pub precise_quantity_in_progress: f64,
    #[prost(double, tag = "31")]
    pub precise_quantity_shipped: f64,
    #[prost(string, tag = "32")]
    pub order_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeaDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, repeated, tag = "10")]
    pub items: ::prost::alloc::vec::Vec<DeaDefinitionItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeaDefinitionItem {
    #[prost(string, tag = "1")]
    pub attribute_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub object_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub default_value: ::prost::alloc::string::String,
    ///The maximum text length if the data type is TYPE_STRING
    #[prost(uint32, tag = "5")]
    pub text_length: u32,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub list_supported: bool,
    #[prost(uint64, tag = "8")]
    pub list_id: u64,
    #[prost(string, repeated, tag = "9")]
    pub default_list_value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DTestDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DTestInstance {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub location_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub object_id: u64,
    #[prost(string, tag = "13")]
    pub object_type: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub operation_code: ::prost::alloc::string::String,
    #[prost(uint32, tag = "15")]
    pub pass_count: u32,
    #[prost(uint32, tag = "16")]
    pub complete_count: u32,
    #[prost(string, tag = "17")]
    pub production_line_code: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "20")]
    pub test_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "21")]
    pub test_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "22")]
    pub test_passed: bool,
    #[prost(bool, tag = "23")]
    pub test_valid: bool,
    #[prost(uint64, tag = "24")]
    pub test_definition_id: u64,
    #[prost(string, tag = "25")]
    pub test_equipment_identity: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "26")]
    pub defect_repair_entries: ::prost::alloc::vec::Vec<DDefectRepairEntry>,
    #[prost(string, tag = "27")]
    pub test_user: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub work_station_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DDefectRepairEntry {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub test_instance_id: u64,
    #[prost(string, tag = "12")]
    pub defect_operation_name: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub defect_operation_code: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub defect_route_name: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub defect_route_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "16")]
    pub defect_route_id: u64,
    #[prost(string, tag = "17")]
    pub defect_route_step_number: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub defect_route_step_id: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub defect_user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub defect_code: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub defect_comment: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub repair_operation_name: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub repair_operation_code: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub repair_route_name: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub repair_route_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "26")]
    pub repair_route_id: u64,
    #[prost(string, tag = "27")]
    pub repair_route_step_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "28")]
    pub repair_route_step_id: u64,
    #[prost(string, tag = "29")]
    pub repair_user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "30")]
    pub repair_code: ::prost::alloc::string::String,
    #[prost(string, tag = "31")]
    pub repair_comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "32")]
    pub repaired: bool,
    #[prost(uint64, tag = "33")]
    pub object_id: u64,
    #[prost(string, tag = "34")]
    pub object_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductionLine {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub capacity: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub location: ::core::option::Option<DLocation>,
    #[prost(uint64, repeated, tag = "15")]
    pub production_queue_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "16")]
    pub storage_unit_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "17")]
    pub work_station_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DStorageUnit {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "9")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, optional, tag = "10")]
    pub location: ::core::option::Option<DLocation>,
    #[prost(int32, tag = "11")]
    pub maximum_storage_duration: i32,
    #[prost(int32, tag = "12")]
    pub minimum_storage_duration: i32,
    #[prost(message, optional, tag = "13")]
    pub part_class: ::core::option::Option<DPartClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DChecklistDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "11")]
    pub items: ::prost::alloc::vec::Vec<DChecklistDefinitionItem>,
    #[prost(bool, tag = "12")]
    pub checklist_items_changed: bool,
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DChecklistDefinitionItem {
    #[prost(string, tag = "1")]
    pub item_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub item_order: i32,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub max_value: f64,
    #[prost(double, tag = "5")]
    pub min_value: f64,
    #[prost(double, tag = "6")]
    pub standard_value: f64,
    #[prost(string, tag = "7")]
    pub text_value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DChecklistInstance {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(message, repeated, tag = "11")]
    pub items: ::prost::alloc::vec::Vec<DChecklistInstanceItem>,
    #[prost(bool, tag = "12")]
    pub checklist_items_changed: bool,
    #[prost(uint64, tag = "13")]
    pub checklist_define_id: u64,
    #[prost(uint64, tag = "14")]
    pub obj_id: u64,
    #[prost(string, tag = "15")]
    pub obj_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DChecklistInstanceItem {
    #[prost(string, tag = "1")]
    pub item_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub item_order: i32,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(double, tag = "4")]
    pub max_value: f64,
    #[prost(double, tag = "5")]
    pub min_value: f64,
    #[prost(double, tag = "6")]
    pub standard_value: f64,
    #[prost(string, tag = "7")]
    pub text_value: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub instance_id: u64,
    #[prost(uint64, tag = "9")]
    pub obj_id: u64,
    #[prost(string, tag = "10")]
    pub obj_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub checked: bool,
    #[prost(string, tag = "12")]
    pub comment: ::prost::alloc::string::String,
    #[prost(double, tag = "13")]
    pub actual_value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, repeated, tag = "10")]
    pub column_definitions: ::prost::alloc::vec::Vec<DatColumnDefinition>,
    #[prost(message, repeated, tag = "11")]
    pub index_definitions: ::prost::alloc::vec::Vec<DatIndexDefinition>,
    #[prost(bool, tag = "12")]
    pub static_data: bool,
    #[prost(bool, tag = "13")]
    pub column_definitions_changed: bool,
    #[prost(bool, tag = "14")]
    pub index_definitions_changed: bool,
    #[prost(string, tag = "15")]
    pub at_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "16")]
    pub at_version: u32,
    #[prost(uint64, tag = "17")]
    pub parent_id: u64,
    #[prost(enumeration = "super::enums::AtDefinitionType", tag = "18")]
    pub r#type: i32,
    #[prost(string, tag = "19")]
    pub column_prefix: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatColumnDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub at_definition_id: u64,
    #[prost(enumeration = "super::enums::ColumnDataType", tag = "11")]
    pub data_type: i32,
    #[prost(int32, tag = "12")]
    pub text_length: i32,
    #[prost(string, tag = "13")]
    pub column_name: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub default_value: ::prost::alloc::string::String,
    #[prost(bool, tag = "15")]
    pub nullable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatIndexDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub at_definition_id: u64,
    #[prost(message, repeated, tag = "11")]
    pub columns: ::prost::alloc::vec::Vec<DatColumnDefinition>,
    #[prost(string, tag = "12")]
    pub sort_direction: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub index_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "14")]
    pub unique: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatData {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(uint64, tag = "10")]
    pub at_definition_id: u64,
    #[prost(message, repeated, tag = "11")]
    pub dependent_atdatas: ::prost::alloc::vec::Vec<DatData>,
    #[prost(enumeration = "super::enums::AtDefinitionType", tag = "12")]
    pub r#type: i32,
    #[prost(uint64, tag = "13")]
    pub parent_id: u64,
    #[prost(message, repeated, tag = "14")]
    pub data_cells: ::prost::alloc::vec::Vec<DatDataCell>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatDataCell {
    #[prost(enumeration = "super::enums::ColumnDataType", tag = "1")]
    pub data_type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub blob_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DFilter {
    #[prost(int32, tag = "1")]
    pub object_type: i32,
    #[prost(int32, tag = "2")]
    pub paging_filter_row_count: i32,
    #[prost(int32, tag = "3")]
    pub paging_filter_start_row: i32,
    #[prost(message, repeated, tag = "4")]
    pub search_constraints: ::prost::alloc::vec::Vec<DFilterSearchConstraint>,
    #[prost(message, repeated, tag = "5")]
    pub sort_constraints: ::prost::alloc::vec::Vec<DFilterSortConstraint>,
    #[prost(message, repeated, tag = "6")]
    pub or_filters: ::prost::alloc::vec::Vec<DFilter>,
    #[prost(int32, tag = "7")]
    pub max_result_count: i32,
    #[prost(bool, tag = "8")]
    pub distinct_only: bool,
    #[prost(message, repeated, tag = "9")]
    pub parameters: ::prost::alloc::vec::Vec<DFilterParameter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DFilterSearchConstraint {
    #[prost(int32, tag = "1")]
    pub attribute: i32,
    #[prost(string, tag = "2")]
    pub attribute_identifier: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub comparison_operator: i32,
    #[prost(string, repeated, tag = "4")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "5")]
    pub other_attribute: i32,
    #[prost(message, repeated, tag = "6")]
    pub or_search_constraints: ::prost::alloc::vec::Vec<DFilterSearchConstraint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DFilterSortConstraint {
    #[prost(int32, tag = "1")]
    pub attribute: i32,
    #[prost(string, tag = "2")]
    pub attribute_identifier: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub sort_order: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DFilterParameter {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub default_value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePriorityRequest {
    #[prost(uint64, tag = "1")]
    pub object_id: u64,
    #[prost(uint32, tag = "2")]
    pub new_priority: u32,
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DLotReturnData {
    #[prost(message, optional, tag = "1")]
    pub lot: ::core::option::Option<DLot>,
    #[prost(message, repeated, tag = "2")]
    pub units: ::prost::alloc::vec::Vec<DUnit>,
    #[prost(message, optional, tag = "3")]
    pub order: ::core::option::Option<DWorkOrder>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DConsumedPart {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub alternate_bom_item_id: u64,
    #[prost(string, tag = "12")]
    pub consumption_status: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub consumption_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "15")]
    pub consumption_op_code: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub part_batch: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub part_invoice: ::prost::alloc::string::String,
    #[prost(uint64, tag = "18")]
    pub part_id: u64,
    #[prost(string, tag = "19")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(double, tag = "21")]
    pub quantity: f64,
    #[prost(string, tag = "22")]
    pub part_serial: ::prost::alloc::string::String,
    #[prost(uint64, tag = "23")]
    pub consumed_part_runtime_bom_id: u64,
    #[prost(uint64, tag = "24")]
    pub consumed_part_tracked_obj_id: u64,
    #[prost(string, tag = "25")]
    pub consumed_part_tracked_obj_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "26")]
    pub consumed_part_tracked_obj_status_id: u64,
    #[prost(string, tag = "27")]
    pub placement: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub production_line_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "29")]
    pub production_line_id: u64,
    #[prost(string, tag = "30")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "31")]
    pub route_id: u64,
    #[prost(string, tag = "32")]
    pub route_step_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "33")]
    pub route_step_id: u64,
    #[prost(uint64, tag = "34")]
    pub work_station_id: u64,
    #[prost(string, tag = "35")]
    pub work_station_code: ::prost::alloc::string::String,
    #[prost(uint64, tag = "36")]
    pub tracked_obj_id: u64,
    #[prost(string, tag = "37")]
    pub tracked_obj_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfiguration {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration = "super::enums::ProductConfigurationType", tag = "13")]
    pub r#type: i32,
    #[prost(message, optional, tag = "14")]
    pub effective_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub effective_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "16")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "17")]
    pub part_id: u64,
    #[prost(string, tag = "18")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "20")]
    pub order_id: u64,
    #[prost(string, tag = "21")]
    pub order_number: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRoute {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "12")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_id: u64,
    #[prost(string, tag = "14")]
    pub route_code: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub route_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "16")]
    pub effective_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "17")]
    pub effective_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "18")]
    pub default: bool,
    #[prost(bool, tag = "19")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRouteStep {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "12")]
    pub production_config_route_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_step_id: u64,
    #[prost(bool, tag = "14")]
    pub enforcement: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRouteStepBom {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "12")]
    pub production_config_route_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_step_id: u64,
    #[prost(string, tag = "14")]
    pub bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub bom_revision: ::prost::alloc::string::String,
    #[prost(uint64, tag = "16")]
    pub bom_item_id: u64,
    #[prost(string, tag = "17")]
    pub part_number: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub part_revision: ::prost::alloc::string::String,
    #[prost(double, tag = "19")]
    pub quatity: f64,
    #[prost(string, tag = "20")]
    pub uom: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub consumption_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "22")]
    pub record_consumption: bool,
    #[prost(bool, tag = "23")]
    pub enabled: bool,
    #[prost(double, tag = "24")]
    pub upper_quatity: f64,
    #[prost(double, tag = "25")]
    pub lower_quatity: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRouteStepDoc {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "12")]
    pub production_config_route_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_step_id: u64,
    #[prost(string, tag = "14")]
    pub doc_code: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub doc_version: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub doc_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "17")]
    pub enforcement: bool,
    #[prost(bool, tag = "18")]
    pub enabled: bool,
    #[prost(uint64, tag = "19")]
    pub doc_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRouteStepEquipmentClass {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "12")]
    pub production_config_route_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_step_id: u64,
    #[prost(uint64, tag = "14")]
    pub equipment_class_id: u64,
    #[prost(int32, tag = "15")]
    pub quatity: i32,
    #[prost(string, tag = "16")]
    pub uom: ::prost::alloc::string::String,
    #[prost(bool, tag = "17")]
    pub enforcement: bool,
    #[prost(bool, tag = "18")]
    pub enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DProductConfigurationRouteStepCheckList {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(uint64, tag = "11")]
    pub production_config_id: u64,
    #[prost(uint64, tag = "12")]
    pub production_config_route_id: u64,
    #[prost(uint64, tag = "13")]
    pub route_step_id: u64,
    #[prost(uint64, tag = "14")]
    pub check_list_id: u64,
    #[prost(string, tag = "15")]
    pub item_name: ::prost::alloc::string::String,
    #[prost(double, tag = "16")]
    pub max_value: f64,
    #[prost(double, tag = "17")]
    pub min_value: f64,
    #[prost(double, tag = "18")]
    pub standard_value: f64,
    #[prost(string, tag = "19")]
    pub text_value: ::prost::alloc::string::String,
    #[prost(bool, tag = "20")]
    pub enforcement: bool,
    #[prost(bool, tag = "21")]
    pub enabled: bool,
    #[prost(string, tag = "22")]
    pub datasource: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecoDelta {
    #[prost(string, tag = "1")]
    pub eco_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub original_bom_item: ::core::option::Option<DRuntimeBomItem>,
    #[prost(message, repeated, tag = "3")]
    pub removed_consumed_parts: ::prost::alloc::vec::Vec<DConsumedPart>,
    #[prost(message, optional, tag = "4")]
    pub revised_bom_item: ::core::option::Option<DRuntimeBomItem>,
    #[prost(string, tag = "5")]
    pub revised_bom_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub revised_bom_revision: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DList {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "2")]
    pub list_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "10")]
    pub list_items: ::prost::alloc::vec::Vec<DListItem>,
    #[prost(bool, tag = "11")]
    pub list_items_changed: bool,
    #[prost(bool, tag = "12")]
    pub allow_duplicates: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DListItem {
    #[prost(int32, tag = "1")]
    pub item_order: i32,
    #[prost(string, tag = "2")]
    pub item_value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DPallet {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(message, optional, tag = "10")]
    pub extended_attributes: ::core::option::Option<DeaInstance>,
    #[prost(string, tag = "11")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub container_id: u64,
    #[prost(string, tag = "13")]
    pub container_type: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag = "14")]
    pub parent: ::core::option::Option<::prost::alloc::boxed::Box<DPallet>>,
    #[prost(uint64, tag = "15")]
    pub location_id: u64,
    #[prost(string, tag = "16")]
    pub location_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeSignatureDefinition {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    #[prost(string, tag = "10")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub comment_required: bool,
    #[prost(bool, tag = "12")]
    pub send_email: bool,
    #[prost(string, tag = "13")]
    pub reason: ::prost::alloc::string::String,
    #[prost(bool, tag = "14")]
    pub performer_two_token: bool,
    #[prost(int32, tag = "15")]
    pub verifier_time_interval: i32,
    /// 0 = performer, 1 = verifier
    #[prost(int32, tag = "16")]
    pub r#type: i32,
}
