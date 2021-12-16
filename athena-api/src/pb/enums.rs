#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModificationFlag {
    New = 0,
    Modify = 1,
    Unchanged = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserStatus {
    Enabled = 0,
    Disabled = 1,
    Locked = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BoxType {
    Normal = 0,
    Group = 1,
    Reusable = 2,
    TrackedReusable = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CycleDurationType {
    QuantityDuration = 0,
    WorkDuration = 1,
    FixedTime = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteStepType {
    StepTypeStep = 0,
    StepTypeEmbedded = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteQueueType {
    QueueTypeNormal = 0,
    QueueTypeEntry = 1,
    QueueTypeExit = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteArcNodeType {
    RouteStep = 0,
    RouteQueue = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtDefinitionType {
    AtTypeStandalone = 0,
    AtTypeDependent = 1,
    AtTypeParent = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ColumnDataType {
    TypeInteger = 0,
    TypeLong = 1,
    TypeFloat = 2,
    TypeString = 3,
    TypeDecimal = 4,
    TypeDatetime = 5,
    TypeBinary = 6,
    TypeBoolean = 7,
    TypeDouble = 8,
    TypeObject = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProductConfigurationType {
    Standard = 0,
    WorkOrder = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BomType {
    Standard = 0,
    WorkOrder = 1,
}
