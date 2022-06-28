pub const QUERY_KNOWBE4_TESTS: &str = r#"
SELECT 
    id
    ,campaign_id
    ,pst_id
    ,status
    ,name
    ,phish_prone_percentage
    ,started_at
    ,duration
    ,scheduled_count
    ,delivered_count
    ,opened_count
    ,clicked_count
    ,replied_count
    ,attachment_open_count
    ,macro_enabled_count
    ,data_entered_count
    ,ISNULL(vulnerable_plugin_count,0) as vulnerable_plugin_count
    ,ISNULL(exploited_count,0) as exploited_count
    ,reported_count
    ,bounced_count
FROM infraportal.knowbe4.tests
WHERE id IN (@P1,@P2,@P3,@P4,@P5,@P6)
ORDER BY started_at DESC;
"#;

pub const LIST_KNOWBE4_TESTS: &str = r#"
SELECT 
    id
    ,campaign_id
    ,pst_id
    ,status
    ,name
    ,phish_prone_percentage
    ,started_at
    ,duration
    ,scheduled_count
    ,delivered_count
    ,opened_count
    ,clicked_count
    ,replied_count
    ,attachment_open_count
    ,macro_enabled_count
    ,data_entered_count
    ,ISNULL(vulnerable_plugin_count,0) as vulnerable_plugin_count
    ,ISNULL(exploited_count,0) as exploited_count
    ,reported_count
    ,bounced_count
FROM infraportal.knowbe4.tests
ORDER BY started_at DESC;
"#;

pub const LIST_KNOWBE4_RESULTS: &str = r#"
SELECT 
CASE
    When (
        r.clicked_at is not null or
        r.replied_at is not null or
        r.attachment_opened_at is not null or
        r.macro_enabled_at is not null or
        r.data_entered_at is not null or
        r.exploited_at is not null    
        ) 
    THEN 'true' ELSE 'false' END as 'failed'
    ,  (
	case when (r.clicked_at is not null) then 1 else 0 end +
	case when (r.replied_at is not null) then 1 else 0 end +
	case when (r.attachment_opened_at is not null) then 1 else 0 end +
	case when (r.macro_enabled_at is not null) then 1 else 0 end +
	case when (r.data_entered_at is not null) then 1 else 0 end +
	case when (r.exploited_at is not null) then 1 else 0 end 
	) as 'total_failures'
    ,r.id
    ,r.userId as 'user_id'
    ,r.first_name
    ,r.last_name
    ,r.email
    ,u.manager_name
    ,u.manager_email
    ,r.recipient_id
    ,r.pst_id
    ,r.scheduled_at
    ,r.delivered_at
    ,r.opened_at
    ,r.clicked_at
    ,r.replied_at
    ,r.attachment_opened_at
    ,r.macro_enabled_at
    ,r.data_entered_at
    ,r.exploited_at
    ,r.reported_at
    ,r.bounced_at
    ,r.ip
    ,r.ip_location
    ,r.browser
    ,r.browser_version
    ,r.os
    ,r.testsId as 'tests_id'
    ,u.custom_field_2
    ,u.division
FROM infraportal.knowbe4.results r
INNER JOIN knowbe4.users u
ON r.userId = u.id
ORDER BY r.scheduled_at DESC;
"#;

pub const QUERY_KNOWBE4_RESULTS: &str = r#"
SELECT 
CASE
    When (
        r.clicked_at is not null or
        r.replied_at is not null or
        r.attachment_opened_at is not null or
        r.macro_enabled_at is not null or
        r.data_entered_at is not null or
        r.exploited_at is not null    
        ) 
    THEN 'true' ELSE 'false' END as 'failed'
    ,  (
	case when (r.clicked_at is not null) then 1 else 0 end +
	case when (r.replied_at is not null) then 1 else 0 end +
	case when (r.attachment_opened_at is not null) then 1 else 0 end +
	case when (r.macro_enabled_at is not null) then 1 else 0 end +
	case when (r.data_entered_at is not null) then 1 else 0 end +
	case when (r.exploited_at is not null) then 1 else 0 end 
	) as 'total_failures'
    ,r.id
    ,r.userId as 'user_id'
    ,r.first_name
    ,r.last_name
    ,r.email
    ,u.manager_name
    ,u.manager_email
    ,r.recipient_id
    ,r.pst_id
    ,r.scheduled_at
    ,r.delivered_at
    ,r.opened_at
    ,r.clicked_at
    ,r.replied_at
    ,r.attachment_opened_at
    ,r.macro_enabled_at
    ,r.data_entered_at
    ,r.exploited_at
    ,r.reported_at
    ,r.bounced_at
    ,r.ip
    ,r.ip_location
    ,r.browser
    ,r.browser_version
    ,r.os
    ,r.testsId as 'tests_id'
    ,u.custom_field_2
    ,u.division
FROM infraportal.knowbe4.results r
INNER JOIN knowbe4.users u
ON r.userId = u.id
WHERE r.testsId = @P1
ORDER BY r.scheduled_at DESC;
"#;

pub const QUERY_KNOWBE4_SUBMITTED_PHISHING_EMAIL: &str = r#"
SELECT 
    [from] as submitter
    ,subject
    ,DATEADD(s,sent,'1970-01-01') as timestamp
FROM suspicious_emails 
WHERE message_id LIKE '%.knowbe4.com>' 
AND sent <100000000000 
AND category=512
AND DATEADD(s,sent,'1970-01-01') > CAST(DATEADD(m, -6, GetDate()) as date)
ORDER BY sent DESC;
"#;

pub const QUERY_KNOWBE4_USERS_THAT_WORK_FOR_ME: &str = r#"
WITH recursiveParent AS
(
SELECT DISTINCT
	 id 'idx'
    ,first_name 'first_namex'
    ,last_name 'last_namex'
    ,manager_name 'man_name'
    ,manager_email 'manager_emailx'
	,email 'emailx'
    ,employee_number 'employee_numberx'
    ,job_title 'job_titlex'
    ,phish_prone_percentage 'phish_prone_percentagex'
    ,phone_number 'phone_numberx'
    ,mobile_phone_number 'mobile_phone_numberx'
    ,location 'locationx'
    ,division 'divisionx'
    ,adi_manageable 'adi_manageablex'
    ,current_risk_score 'current_risk_scorex'
    ,joined_on 'joined_onx'
    ,last_sign_in 'last_sign_inx'
    ,status 'statusx'
    ,custom_field_1 'custom_field_1x'
    ,custom_field_2 'custom_field_2x'
    ,moreInfoId 'moreInfoIdx'
FROM infraportal.knowbe4.users
WHERE manager_email = @P1


UNION ALL

SELECT 
	 id 
	,first_name 
	,last_name 
	,manager_name 
	,manager_email 
	,email 
	,employee_number 
	,job_title 
	,phish_prone_percentage 
	,phone_number 
	,mobile_phone_number 
	,location 
	,division 
	,adi_manageable 
	,current_risk_score 
	,joined_on 
	,last_sign_in 
	,status 
	,custom_field_1 
	,custom_field_2 
	,moreInfoId 
FROM recursiveParent, infraportal.knowbe4.users
WHERE recursiveParent.emailx = infraportal.knowbe4.users.manager_email
)

 SELECT  
     idx 'id'
    ,first_namex 'first_name'
    ,last_namex 'last_name'
    ,man_name 'manager_name'
    ,manager_emailx 'manager_email'
	,emailx 'email'
    ,employee_numberx 'employee_number'
    ,job_titlex 'job_title'
    ,phish_prone_percentagex 'phish_prone_percentage'
    ,phone_numberx 'phone_number'
    ,mobile_phone_numberx 'mobile_phone_number'
    ,locationx 'location'
    ,divisionx 'division'
    ,adi_manageablex 'adi_manageable'
    ,current_risk_scorex 'current_risk_score'
    ,joined_onx 'joined_on'
    ,last_sign_inx 'last_sign_in'
    ,statusx 'status'
    ,custom_field_1x 'custom_field_1'
    ,custom_field_2x 'custom_field_2'
    ,moreInfoIdx 'more_info_id'
	,roll_ups_to = @P1
 FROM recursiveParent
 ORDER BY manager_emailx
 OPTION (MAXRECURSION 10000);
"#;

pub const LIST_KNOWBE4_USERS: &str = r#"
SELECT 
    id
    ,employee_number
    ,first_name
    ,last_name
    ,job_title
    ,email
    ,phish_prone_percentage
    ,phone_number
    ,mobile_phone_number
    ,location
    ,division
    ,manager_name
    ,manager_email
    ,adi_manageable
    ,current_risk_score
    ,joined_on
    ,last_sign_in
    ,status
    ,custom_field_1
    ,custom_field_2
    ,moreInfoId as 'more_info_id'
    ,roll_ups_to = ''
FROM infraportal.knowbe4.users
ORDER BY email;
"#;

pub const LIST_KNOWBE4_TRAINING: &str = r#"
SELECT 
    enrollment_id
    ,content_type
    ,module_name
    ,campaign_name
    ,enrollment_date  
    ,start_date
    ,completion_date
    ,status
    ,time_spent
    ,policy_acknowledged
    ,id
    ,first_name
    ,last_name
    ,email
FROM infraportal.knowbe4.training
WHERE [status] IN ('In Progress', 'Not Started') 
AND
year(enrollment_date) = year(GETDATE())
ORDER BY start_date DESC;
"#;
