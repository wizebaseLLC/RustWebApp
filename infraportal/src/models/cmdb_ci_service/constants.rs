pub const ALL_APPS_URL: &str = "https://neubergertest.service-now.com/api/now/table/cmdb_ci_service?sysparm_display_value=all&sysparm_fields=name%2Csys_id%2Cu_active%2Cu_is_gpdr_affirmed%2Cu_is_signedoff%2Cu_is_signedoff_cbt%2Cu_app_lifecycle%2Cu_appowner_pri";

pub const QUERY_FIND_ALL_CMDB_CI_SERVICE: &str = r#"
SELECT 
sys_id,
u_ie11_explanation,
u_ie11_dependent,
u_asset_type,
dv_asset,
asset,
u_app_replacement_name,
u_app_replacement_date,
dv_u_app_replacement_project,
u_app_replacement_project,
asset_tag,
u_azure_aws_hosted,
assigned_to,
u_vstsproject,
dv_u_vstsproject,
dv_assigned_to,
assignment_group,
dv_assignment_group,
company,
dv_company,
cost_center
dv_cost_center,
department,
dv_department,
justification,
location,
dv_location,
managed_by,
dv_managed_by,
manufacturer,
dv_manufacturer,
model_id,
dv_model_id,
name,
owned_by,
dv_owned_by,
supported_by,
dv_supported_by,
support_group,
dv_support_group,
sys_class_name,
sys_class_path,
sys_created_by,
sys_updated_by,
sys_updated_on,
vendor,
dv_vendor,
category,
comments,
operational_status,
short_description,
u_active,
u_additional_it_approvers,
dv_u_additional_it_approvers,
u_cbt_owner,
dv_u_cbt_owner,
u_tech_owner,
dv_u_tech_owner,
busines_criticality,
user_group,
dv_user_group,
u_6_rs_plan,
u_ad_groups,
dv_u_ad_groups,
u_alt_app_name,
u_appowner_asd_pri,
dv_u_appowner_asd_pri,
u_appowner_asd_sec,
dv_u_appowner_asd_sec,
u_appowner_biz_pri,
dv_u_appowner_biz_pri,
u_appowner_biz_sec,
dv_u_appowner_biz_sec,
u_appowner_pri,
dv_u_appowner_pri,
u_asd_manager,
dv_u_asd_manager,
u_tertiary_asd_owner,
dv_u_tertiary_asd_owner,
u_appowner_sec,
dv_u_appowner_sec,
u_app_access_evidence_loc,
u_app_access_evidence_loc_path,
u_app_access_reviewed_by,
dv_u_app_access_reviewed_by,
u_app_admin,
dv_u_app_admin,
u_app_ci_class,
u_app_lifecycle,
u_app_model,
u_asd_supported_by,
u_biz_functions,
u_business_tier,
u_cloud_strategy_plan_benefits,
u_cloud_strategy_why,
dv_u_cloud_strategy_why,
u_cmdb_id,
u_ip_whitelist,
u_created_on,
u_data_location_type,
u_db_source_control_loc,
u_db_source_conv_date,
u_db_types,
u_deploy_method,
u_dev_platform,
u_dfs_links,
u_dr_doc_loc,
u_entitlement_requested,
u_entitlement_type,
u_external_app,
u_ext_audit,
u_functional_doc_loc,
u_gdpr_affirmed_date,
u_gdpr_affirmed_user,
dv_u_gdpr_affirmed_user,
u_gdpr_comments,
u_gdpr_data_location,
u_gdpr_individuals,
u_gdpr_personal_data,
u_gdpr_person_type,
u_gdpr_processing_grounds,
u_gdpr_recipients,
u_gdpr_scope_area,
u_gdpr_vendor_access,
u_has_data_breach,
u_information_classification,
u_instance_env,
u_invest_status,
u_is_consent_given_gdpr,
u_is_db_in_source_control,
u_is_gdpr_scope,
u_is_gpdr_affirmed,
u_is_open_source,
u_is_signedoff,
u_is_signedoff_cbt,
u_is_trade_data,
u_is_web,
u_last_update_by,
dv_u_last_update_by,
u_last_update_on,
u_num_external,
u_num_internal,
u_runbook_loc,
u_sdlc_product,
dv_u_sdlc_product,
u_service_acct,
dv_u_service_acct,
u_service_type,
u_signoff_date,
u_signoff_date_cbt,
u_signoff_user,
dv_u_signoff_user,
u_signoff_user_cbt,
dv_u_signoff_user_cbt,
u_source_code_loc,
u_svcnow_queue,
dv_u_svcnow_queue,
u_svcnow_queue_asd,
dv_u_svcnow_queue_asd,
u_url,
u_user_access_verify_date,
u_user_location,
u_use_dfs,
u_use_mail,
u_use_mail_desc,
u_vendor_names,
dv_u_vendor_names,
u_vip_infoblox_dns,
dv_u_vip_infoblox_dns,
u_dr_failover_date,
version,
u_percent_complete,
u_sso,
dv_u_sso,
sys_created_on,
u_codebase_location,
u_contract_link,
u_asd_distribution_list,
dv_u_asd_distribution_list,
u_distribution_list,
dv_u_distribution_list

FROM  cmdb_ci_service
WHERE u_active = 'true'
AND u_app_ci_class = 'application'
ORDER BY name;
    "#;

pub const QUERY_FIND_CMDB_CI_SERVICE: &str = r#"
    SELECT Top 1   
    sys_id,
    u_ie11_explanation,
    u_ie11_dependent,
    u_asset_type,
    dv_asset,
    asset,
    u_app_replacement_name,
    u_app_replacement_date,
    dv_u_app_replacement_project,
    u_app_replacement_project,
    asset_tag,
    u_azure_aws_hosted,
    assigned_to,
    u_vstsproject,
    dv_u_vstsproject,
    u_dr_failover_date,
    u_ip_whitelist,
    dv_assigned_to,
    assignment_group,
    dv_assignment_group,
    company,
    dv_company,
    cost_center
    dv_cost_center,
    department,
    dv_department,
    justification,
    location,
    dv_location,
    managed_by,
    dv_managed_by,
    manufacturer,
    dv_manufacturer,
    model_id,
    dv_model_id,
    name,
    owned_by,
    dv_owned_by,
    supported_by,
    dv_supported_by,
    support_group,
    dv_support_group,
    sys_class_name,
    sys_class_path,
    sys_created_by,
    sys_updated_by,
    sys_updated_on,
    vendor,
    dv_vendor,
    category,
    comments,
    operational_status,
    short_description,
    u_active,
    u_additional_it_approvers,
    dv_u_additional_it_approvers,
    u_cbt_owner,
    dv_u_cbt_owner,
    u_tech_owner,
    dv_u_tech_owner,
    busines_criticality,
    user_group,
    dv_user_group,
    u_6_rs_plan,
    u_ad_groups,
    dv_u_ad_groups,
    u_alt_app_name,
    u_appowner_asd_pri,
    dv_u_appowner_asd_pri,
    u_appowner_asd_sec,
    dv_u_appowner_asd_sec,
    u_appowner_biz_pri,
    dv_u_appowner_biz_pri,
    u_appowner_biz_sec,
    dv_u_appowner_biz_sec,
    u_appowner_pri,
    dv_u_appowner_pri,
    u_asd_manager,
    dv_u_asd_manager,
    u_tertiary_asd_owner,
    dv_u_tertiary_asd_owner,
    u_appowner_sec,
    dv_u_appowner_sec,
    u_app_access_evidence_loc,
    u_app_access_evidence_loc_path,
    u_app_access_reviewed_by,
    dv_u_app_access_reviewed_by,
    u_app_admin,
    dv_u_app_admin,
    u_app_ci_class,
    u_app_lifecycle,
    u_app_model,
    u_asd_supported_by,
    u_biz_functions,
    u_business_tier,
    u_cloud_strategy_plan_benefits,
    u_cloud_strategy_why,
    dv_u_cloud_strategy_why,
    u_cmdb_id,
    u_created_on,
    u_data_location_type,
    u_db_source_control_loc,
    u_db_source_conv_date,
    u_db_types,
    u_deploy_method,
    u_dev_platform,
    u_dfs_links,
    u_dr_doc_loc,
    u_entitlement_requested,
    u_entitlement_type,
    u_external_app,
    u_ext_audit,
    u_functional_doc_loc,
    u_gdpr_affirmed_date,
    u_gdpr_affirmed_user,
    dv_u_gdpr_affirmed_user,
    u_gdpr_comments,
    u_gdpr_data_location,
    u_gdpr_individuals,
    u_gdpr_personal_data,
    u_gdpr_person_type,
    u_gdpr_processing_grounds,
    u_gdpr_recipients,
    u_gdpr_scope_area,
    u_gdpr_vendor_access,
    u_has_data_breach,
    u_information_classification,
    u_instance_env,
    u_invest_status,
    u_is_consent_given_gdpr,
    u_is_db_in_source_control,
    u_is_gdpr_scope,
    u_is_gpdr_affirmed,
    u_is_open_source,
    u_is_signedoff,
    u_is_signedoff_cbt,
    u_is_trade_data,
    u_is_web,
    u_last_update_by,
    dv_u_last_update_by,
    u_last_update_on,
    u_num_external,
    u_num_internal,
    u_runbook_loc,
    u_sdlc_product,
    dv_u_sdlc_product,
    u_service_acct,
    dv_u_service_acct,
    u_service_type,
    u_signoff_date,
    u_signoff_date_cbt,
    u_signoff_user,
    dv_u_signoff_user,
    u_signoff_user_cbt,
    dv_u_signoff_user_cbt,
    u_source_code_loc,
    u_svcnow_queue,
    dv_u_svcnow_queue,
    u_svcnow_queue_asd,
    dv_u_svcnow_queue_asd,
    u_url,
    u_user_access_verify_date,
    u_user_location,
    u_use_dfs,
    u_use_mail,
    u_use_mail_desc,
    u_vendor_names,
    dv_u_vendor_names,
    u_vip_infoblox_dns,
    dv_u_vip_infoblox_dns,
    version,
    u_percent_complete,
    u_sso,
    dv_u_sso,
    sys_created_on,
    u_codebase_location,
    u_contract_link,
    u_asd_distribution_list,
    dv_u_asd_distribution_list,
    u_distribution_list,
    dv_u_distribution_list

    FROM  cmdb_ci_service
    WHERE u_active = 'true' and name = @P1 or sys_id = @P1
    ORDER BY name;
        "#;

pub const QUERY_FIND_ALL_CBT_APPLICATIONS: &str = r#"
SELECT 
sys_id,
u_ie11_explanation,
u_ie11_dependent,
u_asset_type,
dv_asset,
asset,
u_app_replacement_name,
u_app_replacement_date,
dv_u_app_replacement_project,
u_app_replacement_project,
asset_tag,
u_azure_aws_hosted,
assigned_to,
u_vstsproject,
dv_u_vstsproject,
dv_assigned_to,
assignment_group,
dv_assignment_group,
company,
dv_company,
cost_center
dv_cost_center,
department,
dv_department,
justification,
location,
dv_location,
managed_by,
u_ip_whitelist,
dv_managed_by,
manufacturer,
dv_manufacturer,
model_id,
dv_model_id,
name,
owned_by,
dv_owned_by,
supported_by,
dv_supported_by,
support_group,
dv_support_group,
sys_class_name,
sys_class_path,
sys_created_by,
sys_updated_by,
sys_updated_on,
vendor,
dv_vendor,
category,
comments,
operational_status,
short_description,
u_active,
u_additional_it_approvers,
dv_u_additional_it_approvers,
u_cbt_owner,
dv_u_cbt_owner,
u_tech_owner,
dv_u_tech_owner,
busines_criticality,
user_group,
dv_user_group,
u_6_rs_plan,
u_ad_groups,
dv_u_ad_groups,
u_alt_app_name,
u_appowner_asd_pri,
dv_u_appowner_asd_pri,
u_appowner_asd_sec,
dv_u_appowner_asd_sec,
u_appowner_biz_pri,
dv_u_appowner_biz_pri,
u_appowner_biz_sec,
dv_u_appowner_biz_sec,
u_appowner_pri,
dv_u_appowner_pri,
u_asd_manager,
dv_u_asd_manager,
u_tertiary_asd_owner,
dv_u_tertiary_asd_owner,
u_appowner_sec,
dv_u_appowner_sec,
u_app_access_evidence_loc,
u_app_access_evidence_loc_path,
u_app_access_reviewed_by,
dv_u_app_access_reviewed_by,
u_app_admin,
dv_u_app_admin,
u_app_ci_class,
u_app_lifecycle,
u_app_model,
u_asd_supported_by,
u_biz_functions,
u_business_tier,
u_cloud_strategy_plan_benefits,
u_cloud_strategy_why,
dv_u_cloud_strategy_why,
u_cmdb_id,
u_created_on,
u_data_location_type,
u_db_source_control_loc,
u_db_source_conv_date,
u_db_types,
u_deploy_method,
u_dev_platform,
u_dfs_links,
u_dr_doc_loc,
u_entitlement_requested,
u_entitlement_type,
u_external_app,
u_ext_audit,
u_functional_doc_loc,
u_gdpr_affirmed_date,
u_gdpr_affirmed_user,
dv_u_gdpr_affirmed_user,
u_gdpr_comments,
u_gdpr_data_location,
u_gdpr_individuals,
u_gdpr_personal_data,
u_gdpr_person_type,
u_gdpr_processing_grounds,
u_gdpr_recipients,
u_gdpr_scope_area,
u_gdpr_vendor_access,
u_has_data_breach,
u_information_classification,
u_instance_env,
u_invest_status,
u_is_consent_given_gdpr,
u_is_db_in_source_control,
u_is_gdpr_scope,
u_is_gpdr_affirmed,
u_is_open_source,
u_is_signedoff,
u_is_signedoff_cbt,
u_is_trade_data,
u_is_web,
u_last_update_by,
dv_u_last_update_by,
u_last_update_on,
u_num_external,
u_num_internal,
u_runbook_loc,
u_sdlc_product,
dv_u_sdlc_product,
u_service_acct,
dv_u_service_acct,
u_service_type,
u_signoff_date,
u_signoff_date_cbt,
u_signoff_user,
dv_u_signoff_user,
u_signoff_user_cbt,
dv_u_signoff_user_cbt,
u_source_code_loc,
u_svcnow_queue,
dv_u_svcnow_queue,
u_svcnow_queue_asd,
dv_u_svcnow_queue_asd,
u_url,
u_user_access_verify_date,
u_user_location,
u_use_dfs,
u_use_mail,
u_use_mail_desc,
u_vendor_names,
dv_u_vendor_names,
u_vip_infoblox_dns,
dv_u_vip_infoblox_dns,
u_dr_failover_date,
version,
u_percent_complete,
u_sso,
dv_u_sso,
sys_created_on,
u_codebase_location,
u_contract_link,
u_asd_distribution_list,
dv_u_asd_distribution_list,
u_distribution_list,
dv_u_distribution_list

FROM  cmdb_ci_service
WHERE u_cbt_owner = @P1
AND u_active = 'true'
AND u_app_ci_class = 'application'
ORDER BY name;
    "#;

pub const QUERY_FIND_SYS_ID_FROM_SYS_USER_FROM_USERNAME: &str = r#"
    SELECT Top 1   
          sys_id
    FROM  sys_user
    WHERE user_name = @P1;
"#;

pub const QUERY_FIND_APPLICATION_CERT_HISTORY: &str = r#"
    SELECT Top 1   
        AppName as name,
        data,
        certHistory as cert_history,
        createDate as create_date
    FROM  AppModelingSnapshots
    WHERE AppName = @P1;
"#;

pub const QUERY_FIND_APPLICATION_CERT_HISTORY_NAME_ONLY: &str = r#"
    SELECT Top 1   
        AppName as name
    FROM  AppModelingSnapshots
    WHERE AppName = @P1;
"#;

pub const QUERY_LIST_APPLICATION_CERT_HISTORY: &str = r#"
    SELECT
        AppName as name,
        data,
        certHistory as cert_history,
        createDate as create_date
    FROM  AppModelingSnapshots;
"#;

pub const UPDATE_APPLICATION_CERT_HISTORY: &str = r#"
    UPDATE AppModelingSnapshots
    SET data = @P2, certHistory = @P3
    WHERE AppName = @P1;
"#;

pub const INSERT_APPLICATION_CERT_HISTORY: &str = r#"
    INSERT INTO AppModelingSnapshots 
    (AppName, createDate, data, certHistory)
    VALUES (@P1, @P2, @P3, @P4);
"#;

pub const QUERY_LIST_APPLICATION_SCORECARD_DATA: &str = r#"
SELECT t1.cbt, CASE WHEN t1.cbt IS NULL THEN NULL ELSE t1.is_infra END AS is_infra, t1.total, t2.n_owner_signoff, t3.n_cbt_signoff 
FROM 
(
    select count(*) as total, dv_u_cbt_owner as cbt, 
    CASE
    When  dv_u_cbt_owner = 'Zampaglione, John' THEN 1 
    When  u.u_job_family = 'Infrastructure Technology' THEN 1 ELSE 0
    END AS is_infra 
    from cmdb_ci_service left join sys_user u on u.sys_id=u_cbt_owner 
    where u_app_ci_class='application' and u_active='true'
    group by dv_u_cbt_owner, u_job_family
) t1
LEFT JOIN
(
    select count(*) as n_owner_signoff, dv_u_cbt_owner as cbt from cmdb_ci_service 
    where u_is_signedoff='true' and u_app_ci_class='application' and u_active='true'
    group by dv_u_cbt_owner
) t2 
ON t1.cbt=t2.cbt
LEFT JOIN
(
    select count(*) as n_cbt_signoff, dv_u_cbt_owner as cbt from cmdb_ci_service 
    where u_is_signedoff_cbt='true' and u_app_ci_class='application' and u_active='true'
    group by dv_u_cbt_owner
) t3
ON t1.cbt=t3.cbt
    ORDER BY is_infra DESC, cbt;
"#;

pub const QUERY_FIND_INCIDENTS_FROM_CI: &str = r#"
    SELECT 
        cmdb_ci,
        dv_cmdb_ci,
        number,
        dv_opened_by,
        short_description,
        state,
        dv_assignment_group,
        priority,
        opened_at,
        sys_id
    FROM [sn_mirror].[dbo].[incident]
    WHERE cmdb_ci = @P1 and opened_at >= (GetDate() - 90)
    ORDER BY state ASC;
"#;

pub const QUERY_FIND_INCIDENTS_FROM_OPENED_BY: &str = r#"
    SELECT 
        cmdb_ci,
        dv_cmdb_ci,
        number,
        dv_opened_by,
        short_description,
        state,
        dv_assignment_group,
        priority,
        opened_at,
        sys_id
    FROM [sn_mirror].[dbo].[incident]
    WHERE opened_by = @P1 and state != '7'
    ORDER BY state ASC;
"#;

pub const QUERY_FIND_CHANGES_FROM_CI: &str = r#"
    SELECT 
        cmdb_ci,
        dv_cmdb_ci,
        number,
        short_description,
        dv_opened_by,
        dv_assignment_group,
        state,
        start_date,
        end_date,
        opened_at,
        sys_id
    FROM [sn_mirror].[dbo].[change_request]
    WHERE cmdb_ci = @P1 and opened_at >= (GetDate() - 90)
    ORDER BY state ASC;
"#;

pub const SEARCH_BAR_QUERY: &str = r#"
    SELECT TOP 10 
    sys_id,name,user_name,'sys_user' as table_name, employee_number
    FROM sys_user
    WHERE (name LIKE '%' + @P1 + '%' OR user_name LIKE '%' + @P1 + '%') AND active = 'true'

    UNION SELECT TOP 10 
    sys_id, name, NULL as user_name,  'cmdb_ci_service' as table_name, NULL as employee_number
    FROM cmdb_ci_service
    WHERE 
    name LIKE '%' + @P1 + '%'
    AND u_app_ci_class = 'application' AND u_active = 'true'

    UNION SELECT TOP 10 
    sys_id, name, NULL as user_name,'cmdb_ci_hardware' as table_name,  NULL as employee_number
    FROM cmdb_ci_hardware
    WHERE name LIKE '%' + @P1 + '%' AND u_active = 'true' AND sys_class_name != 'cmdb_ci_computer'

    UNION SELECT TOP 10 
    sys_id, name, NULL as user_name,sys_class_name as table_name,  NULL as employee_number
    FROM cmdb_ci_hardware
    WHERE name LIKE '%' + @P1 + '%' AND u_active = 'true' AND sys_class_name = 'cmdb_ci_computer'

    UNION SELECT TOP 10 
    sys_id, name,NULL as user_name,'cmdb_ci_database' as table_name, NULL as employee_number
    FROM cmdb_ci_database
    WHERE name LIKE '%' + @P1 + '%' AND u_active = 'true';
"#;
