pub const QUERY_LIST_CMDB_CI_SERVERS: &str = r#"
    SELECT
    cmdb.sys_id,
    cmdb.name,
    cmdb.u_tech_owner,
    u.email as 'pri_owner_email',
    cmdb.dv_u_tech_owner,
    cmdb.u_active,
    cmdb.short_description,
    cmdb.dv_u_cbt_owner,
    cmdb.u_cbt_owner
    FROM  cmdb_ci cmdb
    LEFT JOIN sys_user u on u.sys_id=cmdb.u_tech_owner 
    WHERE cmdb.name IN (@P1)
    OR cmdb.sys_id IN (@P1)
    AND cmdb.u_active = 'true'
    AND (cmdb.sys_class_name = 'cmdb_ci_voice_hardware' 
    OR cmdb.sys_class_name LIKE '%server%')
    AND cmdb.sys_class_name NOT LIKE '%web_server%'
    AND cmdb.sys_class_name NOT IN (
    'cmdb_ci_app_server_websphere',
    'cmdb_ci_app_server_jboss',
    'cmdb_ci_app_server_tomcat',
    'cmdb_ci_appl_license_server',
    'cmdb_ci_app_server_weblogic'
    );
"#;

pub fn query_list_cmdb_ci_servers(keys: Vec<String>) -> String {
    format!(
        r#"
    SELECT
    cmdb.sys_id,
    cmdb.name,
    cmdb.u_tech_owner,
    u.email as 'pri_owner_email',
    cmdb.dv_u_tech_owner,
    cmdb.u_active,
    cmdb.short_description,
    cmdb.dv_u_cbt_owner,
    cmdb.u_cbt_owner
    FROM  cmdb_ci cmdb
    LEFT JOIN sys_user u on u.sys_id=cmdb.u_tech_owner 
    WHERE cmdb.name IN ({0})
    OR cmdb.sys_id IN ({0})
    AND cmdb.u_active = 'true'
    AND (cmdb.sys_class_name = 'cmdb_ci_voice_hardware' 
    OR cmdb.sys_class_name LIKE '%server%')
    AND cmdb.sys_class_name NOT LIKE '%web_server%'
    AND cmdb.sys_class_name NOT IN (
    'cmdb_ci_app_server_websphere',
    'cmdb_ci_app_server_jboss',
    'cmdb_ci_app_server_tomcat',
    'cmdb_ci_appl_license_server',
    'cmdb_ci_app_server_weblogic'
    );
    "#,
        keys.join(",")
    )
}

pub const QUERY_LIST_CMDB_CI_SERVER_PARENT_APPS: &str = r#"
    WITH recursiveParent AS 
    (
    SELECT DISTINCT
    dv_parent as rec_dv_parent,
    type as rec_type,
    parent as rec_parent
    FROM [sn_mirror].[dbo].[cmdb_rel_ci] 
    where dv_child = @P1

    UNION ALL 

    SELECT 
    dv_parent, 
    type, 
    parent 
    FROM recursiveParent, [sn_mirror].[dbo].[cmdb_rel_ci]
    WHERE recursiveParent.rec_dv_parent = [sn_mirror].[dbo].[cmdb_rel_ci].dv_child 
    AND
    [sn_mirror].[dbo].[cmdb_rel_ci].type = '7f3598c2a9fe156100bf97e8d77a89e3'

    )

    SELECT * FROM recursiveParent
    WHERE recursiveParent.rec_type = '7f3598c2a9fe156100bf97e8d77a89e3';
"#;

pub fn query_list_host_lookup(keys: Vec<String>) -> String {
    format!(
        r#"
    SELECT DISTINCT
    rel.dv_child as name,
    rel2.dv_parent as application,
    rel.dv_parent as parent,
    par2.u_business_tier as tier,
    par3.u_business_tier as 'parent_tier',
    par.name as service,
    par.u_active as active,
    par.dv_u_appowner_pri as owner,
    u.email as primary_email,
    par.dv_u_appowner_sec as secondary,
    par.dv_u_appowner_asd_pri as asd_owner,
	par.dv_u_appowner_asd_sec as asd_secondary_owner,
    s.email as secondary_email
    

    FROM [sn_mirror].[dbo].[cmdb_rel_ci] rel
    INNER JOIN cmdb_ci_service par ON rel.parent = par.sys_id
    LEFT JOIN cmdb_rel_ci rel2 ON rel2.child = par.sys_id
    LEFT JOIN cmdb_ci_service par2 ON rel2.parent = par2.sys_id
    LEFT JOIN cmdb_rel_ci rel3 ON rel3.child = par.sys_id
    LEFT JOIN cmdb_ci_service par3 ON rel3.parent = par3.sys_id
    LEFT JOIN sys_user u ON  par.u_appowner_pri = u.sys_id
    LEFT JOIN sys_user s ON par.u_appowner_sec = s.sys_id
    WHERE rel.dv_child in ({}) 
    AND 
    par.u_active = 'true'
    ORDER BY name;
    "#,
        keys.join(",")
    )
}
