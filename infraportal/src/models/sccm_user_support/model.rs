use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SccmPcHardware {
    pub id: Option<String>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub total_free_space: Option<String>,
    pub total_free_space_percent: Option<String>,
    pub total_used_space_percent: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub bios_version: Option<String>,
    pub memory_gb: Option<String>,
    pub memory_slots: Option<String>,
    pub disk_size_gb: Option<String>,
    pub last_reboot_date_time: Option<String>,
    pub last_hardware_inventory: Option<String>,
    pub last_seen_online: Option<String>,
    pub sccm_agent_version: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SccmPcInstalledSoftware {
    pub application: Option<String>,
    pub version: Option<String>,
    pub machine_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SccmPcLoggedInUser {
    pub machine_name: Option<String>,
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub department: Option<String>,
    pub email: Option<String>,
    pub os_version: Option<String>,
    pub site: Option<String>,
    pub login_type: Option<String>,
    pub title: Option<String>,
    pub street_address: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SccmUserPrimaryPc {
    pub primary_machine: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SccmUserProfile {
    pub user_name: Option<String>,
    pub home_directory: Option<String>,
    pub pst_location: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct AltirisUserDrives {
    pub user_id: String,
    pub drive_letter: String,
    pub drive_path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct AltirisUserConferenceRoomDesktops {
    pub user: Option<String>,
    pub assigned_machine: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct SnMirrorUserTasks {
    pub sys_id: String,
    pub requested_for: Option<String>,
    pub dv_requested_for: Option<String>,
    pub state: Option<String>,
    pub opened_by: Option<String>,
    pub opened_at: Option<NaiveDateTime>,
    pub number: Option<String>,
    pub description: Option<String>,
    pub dv_opened_by: Option<String>,
}
