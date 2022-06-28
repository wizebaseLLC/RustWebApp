pub const QUERY_FIND_USER: &str = r#"
    SELECT Top 1   
        name,
        sys_id,
        employee_number,
        user_name,
        email,
        dv_department,
        u_cbt_,
        dv_u_business_head_name,
        u_business_head,
        dv_location,
        title,
        u_display_name,
        phone,
        mobile_phone
    FROM  sys_user
    WHERE user_name = @P1 
    OR sys_id = @P1
    ;"#;

pub const QUERY_FIND_USER_MANY: &str = r#"
    SELECT    
        name,
        sys_id,
        employee_number,
        user_name,
        email,
        dv_department,
        u_cbt_,
        dv_u_business_head_name,
        u_business_head,
        dv_location,
        title,
        u_display_name,
        phone,
        mobile_phone
    FROM sys_user
    WHERE active = 'true'
;"#;

pub const QUERY_IS_ADMIN: &str = r#"
    SELECT
        name, 
        u_members
    FROM sys_user_group
    WHERE u_members LIKE '%' + @P1 + '%' 
    and 
    (
        name = 'ITSM_AllModelingApps' 
        OR name = 'ITSM_TechOwners' 
        OR name = 'NB-Change-Management' 
        OR name = 'Knowbe4_Admins'
        ) 
    ;"#;

pub const QUERY_IS_MANAGER: &str = r#"
    SELECT distinct
        manager,
        dv_manager
    FROM sys_user
    WHERE manager = @P1 
    ORDER BY dv_manager
    ;"#;

pub const QUERY_IS_SUPPORT: &str = r#"
    SELECT
        name, 
        u_members
    FROM sys_user_group
    WHERE u_members LIKE '%' + @P1 + '%' 
    and 
    (
        name = 'Desktop Support NY' 
        OR name = 'Desktop Engineering' 
        OR name = 'desktop_admins' 
        OR name = 'desktop_LocalAdmin'
        OR name = 'Desktop Support International' 
        OR name = 'Desktop_support' 
        OR name = 'TOC-Operations'
        OR name = 'ITSM' 
        OR name = 'Mobile Engineering' 
        OR name = 'HelpDesk'
        OR name = 'InfoSec Support' 
        OR name = 'AppSupportDMS' 
        OR name = 'dmsUsers'
        OR name = 'Networking'
        ) 
    ;"#;

pub const QUERY_FIND_MANAGERS_FROM_BUSINESS_HEAD: &str = r#"
    SELECT 
        name,
        sys_id,
        employee_number,
        user_name,
        email,
        dv_department,
        u_cbt_,
        dv_u_business_head_name,
        u_business_head,
        dv_location,
        title,
        u_display_name,
        phone,
        mobile_phone
    FROM sys_user
    WHERE dv_manager = @P1 
    AND dv_u_business_head_name = @P1
    ;"#;

pub const QUERY_FIND_MANAGERS_FROM_BUSINESS_HEAD_IF_GEORGE_WALKER: &str = r#"
    SELECT 
        name,
        sys_id,
        employee_number,
        user_name,
        email,
        dv_department,
        u_cbt_,
        dv_u_business_head_name,
        u_business_head,
        dv_location,
        title,
        u_display_name,
        phone,
        mobile_phone
    FROM sys_user
    WHERE dv_manager = @P1 
    ;"#;
