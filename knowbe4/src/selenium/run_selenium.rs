use serde::{Deserialize, Serialize};
use thirtyfour::extensions::chrome::ChromeDevTools;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::{prelude::*, GenericWebDriver};
#[derive(Deserialize, Serialize, Debug)]
pub struct Cookies {
    pub domain: String,
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CookiesContainer {
    pub cookies: Vec<Cookies>,
}

pub async fn run_selenium() -> anyhow::Result<Option<Cookies>> {
    println!("pre-driver");
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    // Starts the driver up on Dmitri's selenium server
    let driver = WebDriver::new("http://pipwsel001.nb.com:4444/wd/hub", &caps).await?;
    println!("driver");
    let result = knowbe4_selenium_sequence(&driver).await;

    if let Ok(result) = result {
        driver.close().await?;
        driver.quit().await?;
        Ok(result)
    } else {
        driver.close().await?;
        driver.quit().await?;
        Err(anyhow::anyhow!("Knowbe4 Job Failed. Aborting!"))
    }
}

pub async fn knowbe4_selenium_sequence<'a>(
    driver: &'a GenericWebDriver<ReqwestDriverAsync>,
) -> anyhow::Result<Option<Cookies>> {
    // Launch a webpage for knowbe4
    driver.get("https://training.knowbe4.com/ui/login").await?;
    println!("get");
    // Give time for the spa page to build
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    println!("sleep");
    // Get access to the email text field
    let email_field = driver.find_element(By::Id("email")).await?;
    println!("email field");
    // Insert my email
    email_field.send_keys("svcitsmsel@nb.com").await?;
    println!("send keys");
    // Get access to the email submit button
    let submit_button = driver
        .find_element(By::Css("button[type='submit']"))
        .await?;

    // Then click it
    submit_button.click().await?;

    // Give time for cookie to generate
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let dev_tools = ChromeDevTools::new(driver.session());

    // Extract Cookies and serialize it into Structs
    let cookies = dev_tools.execute_cdp("Network.getAllCookies").await?;
    let cookies: CookiesContainer = serde_json::from_value(cookies)?;

    let cookies = cookies
        .cookies
        .into_iter()
        .find(|cookie| cookie.name.as_str() == "_knowbe4_session_new");

    println!("{:#?}", &cookies);
    Ok(cookies)
}
