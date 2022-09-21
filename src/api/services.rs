use quick_xml::de::from_str as xml_from_str;

use crate::error::{PSError, PSResult};

use super::{model::PSGroup, PSServer};

impl PSServer {
    /// Gets a group from the server.
    pub async fn get_group(&mut self, name: &str) -> PSResult<PSGroup> {
        let resp = self
            .checked_get(&format!("ps/service/groups/{}", name), None, None)
            .await?;
        let status = resp.status().as_u16();
        let text = match resp.text().await {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Failed to decode server response: {:?}", err),
                })
            }
            Ok(_text) => _text.clone(),
        };
        if !(200..300).contains(&status) {
            return Err(PSError::ServerError {
                msg: format!("Get group {} failed: {}", name, text),
            });
        }
        match xml_from_str(&text) {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Deserialisation of xml failed [[ {} ]]: {:?}", text, err),
                })
            }
            Ok(group) => return Ok(group),
        }
    }
}
