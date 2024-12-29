// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::headers::unext_headers;
use crate::unlock_test::utils::{create_reqwest_client, parse_response_to_html, UA_BROWSER};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;

pub struct UNext;

#[async_trait]
impl Service for UNext {
    fn name(&self) -> String {
        "U-Next".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match client.post("https://cc.unext.jp")
            .headers(unext_headers())
            .body("{\"operationName\":\"cosmo_getPlaylistUrl\",\"variables\":{\"code\":\"ED00479780\",\"playMode\":\"caption\",\"bitrateLow\":192,\"bitrateHigh\":null,\"validationOnly\":false},\"query\":\"query cosmo_getPlaylistUrl($code: String, $playMode: String, $bitrateLow: Int, $bitrateHigh: Int, $validationOnly: Boolean) {\\n  webfront_playlistUrl(\\n    code: $code\\n    playMode: $playMode\\n    bitrateLow: $bitrateLow\\n    bitrateHigh: $bitrateHigh\\n    validationOnly: $validationOnly\\n  ) {\\n    subTitle\\n    playToken\\n    playTokenHash\\n    beaconSpan\\n    result {\\n      errorCode\\n      errorMessage\\n      __typename\\n    }\\n    resultStatus\\n    licenseExpireDate\\n    urlInfo {\\n      code\\n      startPoint\\n      resumePoint\\n      endPoint\\n      endrollStartPosition\\n      holderId\\n      saleTypeCode\\n      sceneSearchList {\\n        IMS_AD1\\n        IMS_L\\n        IMS_M\\n        IMS_S\\n        __typename\\n      }\\n      movieProfile {\\n        cdnId\\n        type\\n        playlistUrl\\n        movieAudioList {\\n          audioType\\n          __typename\\n        }\\n        licenseUrlList {\\n          type\\n          licenseUrl\\n          __typename\\n        }\\n        __typename\\n      }\\n      umcContentId\\n      movieSecurityLevelCode\\n      captionFlg\\n      dubFlg\\n      commodityCode\\n      movieAudioList {\\n        audioType\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\"}")
            .send().await {
            Ok(result) => result,
            Err(_) => return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available / Network connection error".to_string()),
            },
        };

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains(r#""resultStatus":475"#) {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if html.contains(r#""resultStatus":200"#) {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if html.contains(r#""resultStatus":467"#) {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            }
        }
    }
}
