use crate::{
    attributes::*, graphql_result::GrapqhqlResult, list_templates_from_category::*,
    out_csv::out_csv_app_modeling, query_knowbe4::query_knowbe4,
    selenium::run_selenium::run_selenium, update_template::*,
};

use warp::Reply;
pub type WarpResult<T> = std::result::Result<T, warp::Rejection>;

/// Runs the knowbe4 template [EXT] automation.
pub async fn run_knowbe4_templates_program() -> WarpResult<impl Reply> {
    let program = run_knowbe4().await.map_err(|_| warp::reject())?;
    Ok(warp::reply::json(&program))
}

async fn run_knowbe4() -> anyhow::Result<String> {
    log::info!(target: "info", "Starting Knowbe4 [EXT] template automation");
    let cookie = run_selenium().await;

    match cookie {
        Ok(cookie) => {
            if let Some(cookie) = cookie {
                let client = reqwest::Client::new();
                // list templates
                let list_template_variables = TemplateVariables::new(None);
                let list_template_body = GrapqhqlResult::new(
                    LIST_TEMPLATES_FROM_CATEGORIES.to_string(),
                    list_template_variables,
                );
                let list_template_result = query_knowbe4::<ListTemplatesRoot, _>(
                    client.clone(),
                    &list_template_body,
                    &cookie,
                )
                .await?;

                // bundle each template into a Vec<AttributesWithId>
                let mapped_template_attributes = list_template_result
                    .data
                    .phishing_templates
                    .nodes
                    .into_iter()
                    .map(|template| AttributesWithId {
                        id: template.id,
                        created_at: template.created_at,
                        updated_at: template.updated_at,
                        attachment_file_name: template.attachment_filename,
                        subject: template.subject,
                        reply_to_display_name: template.reply_to_display_name,
                        reply_to: template.reply_to,
                        rating: template.rating,
                        name: template.name,
                        language_code: template.language_code,
                        landing_page_id: template.landing_page_id,
                        landing_domain_id: template.landing_domain_id,
                        from_display_name: template.from_display_name,
                        from: template.from,
                        content_html: template.content_html,
                        attachment_type: template.attachment_type,
                    })
                    .collect::<Vec<AttributesWithId>>();

                // Log data to a new csv file
                out_csv_app_modeling(mapped_template_attributes.as_slice()).await?;

                // for each of the templates, update the template in knowbe4, but prepending [EXT] to it if it does not already contain it.
                for template in mapped_template_attributes.into_iter() {
                    let attributes = Attributes {
                        attachment_file_name: template.attachment_file_name,
                        subject: template.subject,
                        reply_to_display_name: template.reply_to_display_name,
                        reply_to: template.reply_to,
                        rating: template.rating,
                        name: template.name,
                        language_code: template.language_code,
                        landing_page_id: template.landing_page_id,
                        landing_domain_id: template.landing_domain_id,
                        from_display_name: template.from_display_name,
                        from: template.from,
                        content_html: template.content_html,
                        attachment_type: template.attachment_type,
                    };

                    let variables = UpdateTemplateVariables::new(template.id, attributes);
                    let body = GrapqhqlResult::new(UPDATE_TEMPLATE.to_string(), variables);
                    let result =
                        query_knowbe4::<UpdateTemplateRoot, _>(client.clone(), &body, &cookie)
                            .await?;

                    log::info!(target: "info", 
                        "updating template id:  {:#?}",
                        result.data.phishing_template_update.node.id
                    );
                }
                log::info!(target: "info", "Knowbe4 Template Update Complete");
            } else {
                return Err(anyhow::anyhow!("Cookie not found, Aborting"));
            }
            Ok("Completed Successfully".to_string())
        }
        Err(err) => {
            log::error!(target: "error", "{}", err.to_string());
            Err(anyhow::anyhow!(err))
        }
    }
}
