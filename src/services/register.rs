use std::error::Error;

use serde::{Deserialize, Serialize};

// TDOD:Implement an enum for response type
pub struct RegisterUrlsBuilder {
    /// The shortcode of the organization
    shortcode: Option<i32>,
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.
    /// Only two value are allowed: Completed or Cancelled .
    ///Completed meas Mpesa will automatically complete your transaction whereas Cancelled means Mpesa will automatically cancel the transaction,in the event MPesa is unable to reach your ValidationURL
    responsetype: Option<String>,
    /// Thie is the URL that receives the confirmation request from API upon payment completion
    confirmationurl: Option<String>,
    /// This is the URL that receives the validation request from API upon payment submission.
    /// The validation URL is only called if external validation on the registered shortcode is enabled. (By default external validation is disabled)
    validationurl: Option<String>,
    /// AccessToken
    token: Option<String>,
    //env environment string
    env: Option<String>,
}

/// Register validation and confirmation URLs on M-Pesa
#[derive(Serialize)]
struct RegisterUrls {
    /// The shortcode of the organization
    #[serde(rename = "ShortCode")]
    shortcode: i32,
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.
    /// Only two value are allowed: Completed or Cancelled .
    ///Completed meas Mpesa will automatically complete your transaction whereas Cancelled means Mpesa will automatically cancel the transaction,in the event MPesa is unable to reach your ValidationURL
    #[serde(rename = "ResponseType")]
    responsetype: String,
    /// Thie is the URL that receives the confirmation request from API upon payment completion
    #[serde(rename = "ConfirmationURL")]
    confirmationurl: String,
    /// This is the URL that receives the validation request from API upon payment submission.
    /// The validation URL is only called if external validation on the registered shortcode is enabled. (By default external validation is disabled)
    #[serde(rename = "ValidationURL")]
    validationurl: String,
}

impl RegisterUrlsBuilder {
    pub fn new(token: Option<String>, env: Option<String>) -> RegisterUrlsBuilder {
        RegisterUrlsBuilder {
            shortcode: None,
            responsetype: None,
            confirmationurl: None,
            validationurl: None,
            token,
            env,
        }
    }
    /// The shortcode of the organization

    pub fn shortcode(&mut self, shortcode: i32) -> &mut Self {
        self.shortcode = Some(shortcode);
        self
    }
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.

    pub fn responsetype(&mut self, responsetype: String) -> &mut Self {
        self.responsetype = Some(responsetype);
        self
    }
    /// This is sets the URL that receives the validation request from API upon payment submission.

    pub fn validationurl(&mut self, validationurl: String) -> &mut Self {
        self.validationurl = Some(validationurl);
        self
    }
    /// Thie is sets URL that receives the confirmation request from API upon payment completion

    pub fn confirmationurl(&mut self, confirmationurl: String) -> &mut Self {
        self.confirmationurl = Some(confirmationurl);
        self
    }
    pub async fn register(&self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();

        let registerurl = RegisterUrls {
            shortcode: self.shortcode.ok_or("Short code required")?,
            responsetype: self
                .responsetype
                .as_ref()
                .ok_or("response type required")?
                .to_string(),
            confirmationurl: self
                .confirmationurl
                .as_ref()
                .ok_or("confirmation url required")?
                .to_string(),
            validationurl: self
                .validationurl
                .as_ref()
                .ok_or("validation url required")?
                .to_string(),
        };

        let resp = client
            .post(format!(
                "{}/mpesa/c2b/v1/registerurl",
                self.env.as_ref().unwrap()
            ))
            .bearer_auth(self.token.as_ref().unwrap().to_string())
            .json(&registerurl)
            .send()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
