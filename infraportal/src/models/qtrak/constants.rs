pub const QUERY_QTRAK_LIST_USERS: &str = r#"
SELECT 
    u_display_name 'Fullname',
    first_name 'Firstname',
    last_name 'Lastname',
    email 'Email',
    phone 'Phone',
    mobile_phone 'Phone2',
    dv_location 'Location01',
    u_floor_name 'Location02',
    dv_department 'Location03',
    employee_number 'ContactId'

FROM [sn_mirror].[dbo].[sys_user]
WHERE active = 'true' and dv_location like  @P1;
"#;
