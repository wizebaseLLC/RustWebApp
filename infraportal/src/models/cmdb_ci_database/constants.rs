pub const ALL_APPS_URL: &str = "https://neubergertest.service-now.com/api/now/table/cmdb_ci_service?sysparm_display_value=all&sysparm_fields=name%2Csys_id%2Cu_active%2Cu_is_gpdr_affirmed%2Cu_is_signedoff%2Cu_is_signedoff_cbt%2Cu_app_lifecycle%2Cu_appowner_pri";

pub const QUERY_FIND_ALL_CMDB_CI_DATABASE: &str = r#"
SELECT 
    db.name,
    db.u_active,
    db.u_information_classification,
    db.u_sec_owner,
    db.u_pri_owner,
    db.short_description,
    db.sys_class_name,
    db.u_signoff_user,
    db.dv_u_signoff_user,
    db.u_is_signedoff,
    db.u_signoff_date,
    db.u_is_auth_source,
    db.u_is_vendor_component,
    db.dv_u_sec_owner,
    db.dv_u_pri_owner,
    db.u_dataserver_name,
    db.u_db_name,
    db.u_db_engine,
    db.u_db_tier,
    db.u_is_decom,
    db.dv_u_app,
    db.u_app,
    db.sys_id,
    db.type AS field_type,
    db.model_id,
    db.sys_class_name,
    db.u_cname,
    u.dv_u_cbt
FROM  cmdb_ci_database db
LEFT JOIN sys_user u on u.sys_id=db.u_pri_owner 
WHERE db.u_active = 'true' 
AND db.name <> '' 
AND db.u_is_decom = 'false'
AND db.discovery_source != 'user_request'
ORDER BY name;
    "#;

pub const QUERY_FIND_CMDB_CI_DATABASE: &str = r#"
    SELECT Top 1   
    db.name,
    db.u_active,
    db.u_information_classification,
    db.u_sec_owner,
    db.u_pri_owner,
    db.short_description,
    db.sys_class_name,
    db.u_signoff_user,
    db.dv_u_signoff_user,
    db.u_is_signedoff,
    db.u_signoff_date,
    db.u_is_auth_source,
    db.u_is_vendor_component,
    db.dv_u_sec_owner,
    db.dv_u_pri_owner,
    db.u_dataserver_name,
    db.u_db_name,
    db.u_db_engine,
    db.u_db_tier,
    db.u_is_decom,
    db.dv_u_app,
    db.u_app,
    db.sys_id,
    db.type AS field_type,
    db.u_cname,
    db.model_id,
    db.sys_class_name,
    u.dv_u_cbt
FROM  cmdb_ci_database db
LEFT JOIN sys_user u on u.sys_id=db.u_pri_owner 
WHERE db.name = @P1 
OR db.sys_id = @P1;
        "#;

pub const QUERY_LIST_DATABASE_SCORECARD_DATA: &str = r#"
SELECT t1.cbt, CASE WHEN t1.cbt IS NULL THEN NULL ELSE t1.is_infra END AS is_infra, t1.total, t2.n_owner_signoff 
FROM 
(
          select count(*) as total, u.dv_u_cbt as cbt,
           CASE 
            WHEN u.dv_u_cbt = 'Zampaglione, John' THEN 1 
            WHEN u2.u_job_family = 'Infrastructure Technology' THEN 1 ELSE 0 END AS is_infra 
          from cmdb_ci_database db 
          left join sys_user u on u.sys_id=db.u_pri_owner 
          left join sys_user u2 on u2.sys_id=u.u_cbt
          where db.u_active='true' and db.u_is_decom = 'false' and db.discovery_source != 'user_request'
          group by u.dv_u_cbt, u2.u_job_family
) t1
LEFT JOIN
(
          select count(*) as n_owner_signoff, u.dv_u_cbt as cbt
          from cmdb_ci_database db left join sys_user u on u.sys_id=db.u_pri_owner
          where u_is_signedoff='true' and db.u_active='true' and db.u_is_decom = 'false' and db.discovery_source != 'user_request'
          group by u.dv_u_cbt
) t2 
ON t1.cbt=t2.cbt
ORDER BY is_infra DESC, cbt;

"#;
