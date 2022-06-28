pub const QUERY_FIND_NB_DISABLE: &str = r#"
SELECT TOP 50
    id,
    item,
    want_on_gmt,
    want_on_loc,
    have_on_gmt,
    want_off_gmt,
    want_off_loc,
    have_off_gmt,
    note_bak,
    disabled_by,
    enabled_by,
    disable_stamp,
    enable_stamp,
    note
    FROM [ITSMFMS].[dbo].[ovdisables]
    WHERE item = @P1 + '.nb.com' 
    OR
    item = @P1
    ORDER BY want_off_loc desc;
"#;

pub const LIST_ALL_NB_DISABLE: &str = r#"
SELECT
    id,
    item,
    want_on_gmt,
    want_on_loc,
    have_on_gmt,
    want_off_gmt,
    want_off_loc,
    have_off_gmt,
    note_bak,
    disabled_by,
    enabled_by,
    disable_stamp,
    enable_stamp,
    note
    FROM [ITSMFMS].[dbo].[ovdisables]
    ORDER BY want_off_loc desc;
"#;
