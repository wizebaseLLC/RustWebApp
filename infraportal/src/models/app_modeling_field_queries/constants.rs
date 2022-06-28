pub const QUERY_FIND_ALL_SSO_PARTNERS: &str = r#"
    SELECT 
        sys_id,
        u_id,
        u_partner,
        u_partnerapp,
        u_partnerpid,
        u_partnersys_id,
        u_partnerurl1,
        u_partnerurl2,
        u_partnerurl3,
        u_partnerurlrelaystate1,
        u_partnerurlrelaystate2,
        u_partnerurlrelaystate3
    FROM u_sso_partners
    ORDER BY u_partnerapp
        ;"#;

//Tim has it set where whenever someone submits a request for a new project, and it is fully approved, it'll automatically populate to that table
pub const QUERY_FIND_MATCHING_VSTS_PROJECTS: &str = r#"
SELECT TOP 20
        sys_id,
        u_description,
        u_name,
        u_url
    FROM u_vsts_project
    WHERE u_name like '%' + @P1 + '%'
    ORDER BY u_name
        ;"#;

pub const QUERY_FIND_MATCHING_USERS: &str = r#"
    SELECT TOP 20
        sys_id,
        u_display_name,
        user_name,
        employee_number
    FROM sys_user
    WHERE 
    (
        user_name like '%' + @P1 + '%'
    OR  name like '%' + @P1 + '%'
    OR  u_display_name like '%' + @P1 + '%'
    )
    AND active = 'true'
    AND employee_number <> ''
    ORDER BY u_display_name
        ;"#;

pub const QUERY_FIND_MATCHING_DFS_LINKS: &str = r#"
    SELECT TOP 20
        sys_id,
        u_path
    FROM u_dfs_links
    WHERE   
        u_active = 'true'
    AND u_status = 'Online' 
    AND u_path like '%nb.com%' 
    AND (
        u_path like '%' + @P1 + '%'
    OR  u_target like '%' + @P1 + '%'
    )
    ORDER BY u_path
        ;"#;

pub const QUERY_FIND_MATCHING_VIPS: &str = r#"
    SELECT TOP 20
        sys_id,
        u_name as 'u_vip_name'
    FROM u_f5_vip_infoblox
    WHERE u_name like '%' + @P1 + '%'
    ORDER BY u_name
        ;"#;

pub const QUERY_FIND_MATCHING_VENDORS: &str = r#"
    SELECT TOP 20
        sys_id,
        name
    FROM core_company
    WHERE street <> '' 
    AND name <> ''  
    AND name like '%' + @P1 + '%' 
    ORDER BY name
        ;"#;

pub const QUERY_FIND_ALL_SIX_R: &str = r#"
    SELECT
        sys_id,
        u_why
    FROM u_cloud_strategy_why
    ORDER BY u_why
        ;"#;

pub const QUERY_FIND_MATCHING_SERVICE_ACCOUNTS: &str = r#"
    SELECT TOP 20
        sys_id,
        u_user_name
    FROM u_service_accounts
    WHERE u_user_name <> '' 
    AND u_user_name like '%' + @P1 + '%' 
    ORDER BY u_user_name
        ;"#;

pub const QUERY_FIND_MATCHING_SERVERS: &str = r#"
    SELECT TOP 75
        sys_id,
        name,
        sys_class_name
    FROM cmdb_ci
    WHERE (
        sys_class_name = 'cmdb_ci_voice_hardware' 
        OR sys_class_name LIKE '%server%'
        OR sys_class_name = 'cmdb_ci_ip_firewall'
		OR sys_class_name = 'cmdb_ci_ip_router'
		OR sys_class_name = 'cmdb_ci_ip_switch'
        OR sys_class_name = 'cmdb_ci_lb_netscaler'
        OR sys_class_name = 'cmdb_ci_citrix_netscaler_sdx'
         )
    AND name <> '' 
    AND sys_class_name NOT LIKE '%web_server%'
    AND sys_class_name NOT IN 
    (
    'cmdb_ci_app_server_websphere',
    'cmdb_ci_app_server_jboss',
    'cmdb_ci_app_server_tomcat',
    'cmdb_ci_appl_license_server',
    'cmdb_ci_app_server_weblogic'
    ) 
    AND u_active = 'true'  
    AND name LIKE '%' + @P1 + '%'
    ORDER by name
        ;"#;

pub const QUERY_FIND_MATCHING_DATABASES: &str = r#"
    SELECT TOP 50
        sys_id,
        name
    FROM cmdb_ci_database
    WHERE name <> '' 
    AND u_active = 'true' 
    AND u_is_decom = 'false' 
    AND name LIKE '%' + @P1 + '%'
    ORDER BY name
        ;"#;

pub const QUERY_FIND_DISTRIBUTION_LIST: &str = r#"
SELECT TOP 50 
      sys_id,
	  u_display_name + ' - (' + email + ')' as name
  FROM [sn_mirror].[dbo].[sys_user_group]
  WHERE active = 'true'
  AND email <> '' 
  AND (name like '%' + @P1 + '%' OR u_display_name like '%' + @P1 + '%')
  ORDER BY name
    ;"#;

pub const QUERY_FIND_MATCHING_APPS: &str = r#"
    SELECT TOP 20
        sys_id,
        name
    FROM cmdb_ci_service
    WHERE name like '%' + @P1 + '%'
    AND u_app_ci_class = 'application'
    AND u_active = 'true'
    ORDER by name
        ;"#;

pub const QUERY_FIND_MATCHING_APP_INSTANCES: &str = r#"
SELECT TOP 20
    sys_id,
    name
FROM cmdb_ci_service
WHERE name like '%' + @P1 + '%'
AND u_app_ci_class = 'instance'
AND u_active = 'true'
ORDER by name
    ;"#;

pub const QUERY_FIND_MATCHING_CMDB_MODELS: &str = r#"
    SELECT TOP 20
        sys_id,
        display_name
    FROM cmdb_model
    WHERE sys_class_name = 'cmdb_application_product_model' 
    AND  u_retired = 'false' 
    AND name LIKE '%' + @P1 + '%'
    ORDER BY display_name
        ;"#;

pub const QUERY_FIND_MATCHING_ACTIVE_DIRECTORY_GROUPS: &str = r#"
    SELECT TOP 20
        sys_id,
        name
    FROM sys_user_group
    WHERE name <> '' 
    AND active = 'true' 
    AND (
        name LIKE '%' + @P1 + '%'
        OR sys_id LIKE '%' + @P1 + '%'
        )
    AND type  = '2dbd3ddd15aa1100d7697519dd3e5d3b'
    AND u_members <> ''
    ORDER BY name
        ;"#;

pub const QUERY_FIND_PM_PROJECT: &str = r#"
    SELECT TOP 20
        sys_id,
        short_description + ' (' + number + ')' as short_description
    FROM pm_project
    WHERE short_description like '%' + @P1 + '%'
    OR
    number like '%' + @P1 + '%'
    ORDER BY short_description
        ;"#;

pub const QUERY_FIND_MATCHING_APP_ADMIN: &str = r#"
    SELECT TOP 20
        sys_id,
        u_name
    FROM u_blueappadmin
    WHERE u_name like '%' + @P1 + '%'
    ORDER BY u_name
        ;"#;

pub const QUERY_FIND_SYS_CHOICE_LIST: &str = r#"
    SELECT 
        sys_id,
        label,
        value,
        element,
        name,
        hint
    FROM sys_choice
    WHERE inactive = 'false'
    AND name = @P1
    ORDER BY label
        ;"#;

pub const QUERY_FIND_SYS_DICTIONARY: &str = r#"
    SELECT 
        column_label,
        comments,
        element
    FROM sys_dictionary
    WHERE name = @P1
    ORDER BY column_label
        ;"#;

pub const QUERY_MATCHING_CMDB_CI_CERTIFICATES: &str = r#"
    SELECT 
         sys_id 
        ,issuer
	    ,dv_issuer
        ,subject_alternative_name
        ,dv_subject_alternative_name
        ,u_application_ownership
        ,dv_u_application_ownership
        ,state
        ,valid_from
        ,valid_to
        ,subject_common_name
        ,sys_updated_on
        ,subject_organization
        ,is_self_signed
        ,issuer_common_name
        ,fingerprint
        ,signature_algorithm
        ,version
    FROM cmdb_ci_certificate
    WHERE dv_u_application_ownership = @P1 + ' _ Prod'
    OR
	dv_u_application_ownership =  @P1 + ' _ Dev'
    OR
    dv_u_application_ownership =  @P1 + ' _ Stage'
    OR
    dv_u_application_ownership =  @P1 + ' _ Qa'
    ORDER BY valid_to 
        ;"#;

pub const QUERY_BLUE_RELATIONS: &str = r#"
    SELECT Id as id, ServiceId as service_id, AppName as app_name, HostId as host_id, Env as env, PhysicalLocation as physical_location
    FROM [BlueDB].[dbo].[SNOwnershipReport]
    WHERE AppName = @P1;
"#;

pub fn query_many_servers(keys: Vec<String>) -> String {
    format!(
        r#"
    SELECT sys_id, name
    FROM cmdb_ci_hardware
    WHERE  u_active = 'true'  
    AND name IN ({0})
    ORDER by name;
    "#,
        keys.join(",")
    )
}
