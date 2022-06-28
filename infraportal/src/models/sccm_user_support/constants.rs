pub const QUERY_FIND_SCCM_PC_HARDWARE: &str = r#"
SELECT       v_GS_LOGICAL_DISK.DeviceID0 'id', 
v_R_System.Netbios_Name0 AS [name], 
v_GS_COMPUTER_SYSTEM.Model0 AS 'model', 
CONVERT(VARCHAR(26),SUM(v_GS_LOGICAL_DISK.Size0) /(COUNT(v_GS_PHYSICAL_MEMORY.ResourceID) * 1024)) AS [disk_size_gb], 
                         CONVERT(VARCHAR(26),SUM(v_GS_LOGICAL_DISK.FreeSpace0) / (COUNT(v_GS_PHYSICAL_MEMORY.ResourceID) * 1024)) AS 'total_free_space', CONVERT(VARCHAR(26), ROUND(100.0 * SUM(v_GS_LOGICAL_DISK.FreeSpace0) / SUM(v_GS_LOGICAL_DISK.Size0), 2)) AS 'total_free_space_percent', 
                         CONVERT(VARCHAR(26), ROUND(100.0 * (SUM(v_GS_LOGICAL_DISK.Size0) - SUM(v_GS_LOGICAL_DISK.FreeSpace0)) / SUM(v_GS_LOGICAL_DISK.Size0), 2)) AS [total_used_space_percent], 
                                          v_GS_PC_BIOS.SMBIOSBIOSVersion0 AS 'bios_version', 
                                          v_GS_COMPUTER_SYSTEM.Manufacturer0 AS 'manufacturer', 
                         v_GS_PC_BIOS.SerialNumber0 AS 'serial_number', 
                                          CONVERT(VARCHAR(26), v_GS_OPERATING_SYSTEM.LastBootUpTime0, 100) AS 'last_reboot_date_time', 
                                          CONVERT(VARCHAR(26), v_CH_ClientSummary.LastHW, 101) AS 'last_hardware_inventory', CONVERT(VARCHAR(26), v_CH_ClientSummary.LastOnline, 101) AS 'last_seen_online', v_R_System.Client_Version0 AS 'sccm_agent_version',
                                         CONVERT(VARCHAR(26),SUM(ISNULL(v_GS_PHYSICAL_MEMORY.Capacity0,0)) / 1024) 'memory_gb',
                                         CONVERT(VARCHAR(26),COUNT(v_GS_PHYSICAL_MEMORY.ResourceID)) 'memory_slots'
                                         
FROM            v_R_System INNER JOIN
                         v_GS_PC_BIOS ON v_R_System.ResourceID = v_GS_PC_BIOS.ResourceID INNER JOIN
                         v_GS_PHYSICAL_MEMORY ON v_R_System.ResourceID = v_GS_PHYSICAL_MEMORY.ResourceID INNER JOIN
                                         v_GS_COMPUTER_SYSTEM ON v_R_System.ResourceID = v_GS_COMPUTER_SYSTEM.ResourceID INNER JOIN
                         v_GS_LOGICAL_DISK ON v_R_System.ResourceID = v_GS_LOGICAL_DISK.ResourceID INNER JOIN
                         v_GS_OPERATING_SYSTEM ON v_R_System.ResourceID = v_GS_OPERATING_SYSTEM.ResourceID INNER JOIN
                         v_CH_ClientSummary ON v_R_System.ResourceID = v_CH_ClientSummary.ResourceID INNER JOIN
                         
                         v_GS_WORKSTATION_STATUS AS HWSCAN ON v_R_System.ResourceID = HWSCAN.ResourceID
WHERE        (v_GS_COMPUTER_SYSTEM.Model0 LIKE '%') AND (v_GS_LOGICAL_DISK.DeviceID0 LIKE '%') AND (v_GS_LOGICAL_DISK.Size0 IS NOT NULL) AND (v_GS_LOGICAL_DISK.DeviceID0 LIKE 'C:') AND 
                         (v_R_System.Netbios_Name0 = @P1)
GROUP BY v_R_System.Netbios_Name0, v_GS_COMPUTER_SYSTEM.Model0, v_GS_LOGICAL_DISK.DeviceID0, HWSCAN.LastHWScan, v_GS_PC_BIOS.SMBIOSBIOSVersion0, v_GS_COMPUTER_SYSTEM.Manufacturer0, 
                         v_GS_PC_BIOS.SerialNumber0, v_GS_OPERATING_SYSTEM.LastBootUpTime0, v_CH_ClientSummary.LastHW, v_CH_ClientSummary.LastOnline, v_R_System.Client_Version0, v_GS_PHYSICAL_MEMORY.Capacity0
ORDER BY 'total_free_space', 'total_free_space_percent';

    "#;

pub const QUERY_FIND_SCCM_PC_INSTALLED_SOFTWARE: &str = r#"
    SELECT DISTINCT 
    v_GS_INSTALLED_SOFTWARE.ARPDisplayName0 as 'application', 
    v_GS_INSTALLED_SOFTWARE.ProductVersion0 as 'version',
    v_R_System.Name0 as 'machine_name'
    FROM v_GS_INSTALLED_SOFTWARE
    INNER JOIN
    v_R_System ON v_GS_INSTALLED_SOFTWARE.ResourceID = v_R_System.ResourceID
    WHERE (v_R_System.Name0 = @P1)
    AND 
        (OsComponent0 = '0' 
        OR 
        v_GS_INSTALLED_SOFTWARE.ARPDisplayName0 LIKE ('Symantec%') ) 
        AND 
        v_GS_INSTALLED_SOFTWARE.ARPDisplayName0 <> '';
    "#;

pub const QUERY_FIND_SCCM_PC_LOGGED_IN_USER: &str = r#"
    SELECT DISTINCT 
    SYS.Netbios_Name0 AS 'machine_name', 
    USR.User_Name0 AS 'user_name', 
    USR.Full_User_Name0 AS 'full_name', 
    USR.department AS 'department', 
    USR.Mail0 AS 'email', 
    SYS.BuildExt AS 'os_version', 
    SYS.AD_Site_Name0 as 'site', 
    'Logged on' AS 'login_type', 
    USR.title as 'title', 
    USR.streetAddress as 'street_address'
    FROM v_R_System AS SYS 
    LEFT OUTER JOIN
    v_R_User AS USR ON USR.User_Name0 = SYS.User_Name0 
    LEFT OUTER JOIN
    v_GS_OPERATING_SYSTEM ON v_GS_OPERATING_SYSTEM.ResourceID = SYS.ResourceID
    WHERE (USR.User_Name0 NOT LIKE '%svc%') 
    AND (SYS.Netbios_Name0 NOT LIKE '%VDI%') 
    AND (SYS.Netbios_Name0 NOT LIKE '%CNF%') 
    AND (SYS.Netbios_Name0 NOT LIKE '%TEL%') 
    AND (SYS.Netbios_Name0 = @P1); 
    "#;

pub const QUERY_FIND_SCCM_PRIMARY_PC: &str = r#"
    SELECT SYS.Name0 as 'primary_machine',
    USR.User_Name0 as 'user_name'
    FROM v_UsersPrimaryMachines AS upm 
    LEFT OUTER JOIN
    v_R_User AS USR ON upm.UserResourceID = USR.ResourceID 
    LEFT OUTER JOIN
    v_R_System AS SYS ON upm.MachineID = SYS.ResourceID
    WHERE (USR.Unique_User_Name0 <> 'Null')  
    AND USR.User_Name0 = @P1;
    "#;

pub const QUERY_FIND_SCCM_USER_PROFILE: &str = r#"
    SELECT  User_Name0 AS 'user_name', 
    homeDirectory as 'home_directory',
    extensionAttribute6 as 'pst_location',
    profilePath as 'profile'
    FROM  v_R_User
    WHERE (Unique_User_Name0 <> 'Null') 
    AND (Unique_User_Name0 NOT LIKE '%2%') 
    AND (Unique_User_Name0 NOT LIKE '%8%') 
    AND (Unique_User_Name0 NOT LIKE '%svc%')  
    AND User_Name0 = @P1;
    "#;

pub const QUERY_FIND_SCCM_USER_DRIVES: &str = r#"
    SELECT 
    UserID as 'user_id',
    DriveLetter as 'drive_letter',
    DrivePath as 'drive_path'
    FROM [DriveMapping].[dbo].[V_DriveMapping]
    WHERE UserID = @P1;
    "#;

pub const QUERY_FIND_SCCM_USER_CONFERENCE_DESKTOPS: &str = r#"
    SELECT 
    [User] as 'user',
    AssignedMachine as 'assigned_machine'
    FROM [DriveMapping].[dbo].[citrixdesktop] 
    WHERE [User] = 'nb\' + @P1;
    "#;

pub const QUERY_FIND_SN_MIRROR_USER_TASK: &str = r#"
    SELECT requested_for,dv_requested_for,stage 'state',sys_id,opened_at,opened_by,number,dv_opened_by,short_description 'description'
    FROM sc_req_item
    WHERE requested_for = @P1   
    AND stage != 'completed';
    "#;
