use hyper::Method;
use url::Url;

use requests::base::{Request, RawResponse, add_optional_param};
use types::{Priority, Sound};

/// Send a message
///
/// Return type is [SendMessageResponse](struct.SendMessageResponse.html).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SendMessage {
    pub token: String,
    pub user_key: String,
    pub message: String,
    pub devices: Vec<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub url_title: Option<String>,
    pub priority: Option<Priority>,
    pub timestamp: Option<String>,
    pub sound: Option<Sound>,
}

impl SendMessage {
    pub fn new<M, T, U>(token: T, user_key: U, message: M) -> Self
        where T: Into<String>,
              U: Into<String>,
              M: Into<String>
    {
        Self {
            token: token.into(),
            user_key: user_key.into(),
            message: message.into(),
            devices: Vec::new(),
            title: None,
            url: None,
            url_title: None,
            priority: None,
            timestamp: None,
            sound: None,
        }
    }

    pub fn add_device<T: Into<String>>(&mut self, device: T) {
        self.devices.push(device.into());
    }

    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = Some(title.into());
    }

    pub fn set_url<T: Into<String>>(&mut self, url: T) {
        self.url = Some(url.into());
    }

    pub fn set_url_title<T: Into<String>>(&mut self, title: T) {
        self.url_title = Some(title.into());
    }

    pub fn set_timestamp<T: Into<String>>(&mut self, timestamp: T) {
        self.timestamp = Some(timestamp.into());
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = Some(priority);
    }

    pub fn set_sound(&mut self, sound: Sound) {
        self.sound = Some(sound);
    }
}

impl Request for SendMessage {
    type ResponseType = SendMessageResponse;
    type RawResponseType = RawSendMessageResponse;

    fn build_url(&self, url: &mut Url) {
        url.path_segments_mut().unwrap().push("messages.json");

        let mut params = url.query_pairs_mut();

        params.append_pair("token", &self.token);
        params.append_pair("user", &self.user_key);
        params.append_pair("message", &self.message);
        add_optional_param(&mut params, "title", &self.title);
        add_optional_param(&mut params, "url", &self.url);
        add_optional_param(&mut params, "url_title", &self.url_title);
        add_optional_param(&mut params, "timestamp", &self.timestamp);
        add_optional_param(&mut params, "sound", &self.sound);

        if !self.devices.is_empty() {
            let list = self.devices.join(",");

            params.append_pair("device", &list);
        }

        if let Some(ref value) = self.priority {
            params.append_pair("priority", &value.to_string());

            if let Priority::Emergency {
                       retry,
                       expire,
                       ref callback_url,
                   } = *value {
                params.append_pair("retry", &retry.to_string());
                params.append_pair("expire", &expire.to_string());
                add_optional_param(&mut params, "callback", callback_url);
            }
        }
    }

    fn get_method(&self) -> Method {
        Method::Post
    }

    fn map(raw: Self::RawResponseType) -> Self::ResponseType {
        Self::ResponseType {
            request: raw.request,
            receipt: raw.receipt,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// Return type for [SendMessage](struct.SendMessage.html)
pub struct SendMessageResponse {
    pub receipt: Option<String>,
    pub request: String,
}

#[derive(Deserialize)]
pub struct RawSendMessageResponse {
    pub status: i32,
    pub request: String,
    pub errors: Option<Vec<String>>,
    pub receipt: Option<String>,
}

impl RawResponse for RawSendMessageResponse {
    raw_response_basic_getters!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::assert_req_url;

    #[test]
    fn get_url_with_all_fields() {
        let mut req = SendMessage::new("send_token", "send user", "send message");
        req.add_device("send device");
        req.set_title("send title");
        req.set_url("send url");
        req.set_url_title("send url title");
        req.set_timestamp("send timestamp");
        req.set_priority(Priority::Normal);
        req.set_sound(Sound::Pushover);

        assert_req_url(&req,
                       "messages.json",
                       Some(&[("token", &req.token),
                         ("user", &req.user_key),
                         ("message", &req.message),
                         ("title", req.title.as_ref().unwrap()),
                         ("url", req.url.as_ref().unwrap()),
                         ("url_title", req.url_title.as_ref().unwrap()),
                         ("timestamp", req.timestamp.as_ref().unwrap()),
                         ("sound", &req.sound.as_ref().unwrap().to_string()),
                         ("device", &req.devices[0]),
                         ("priority", &req.priority.as_ref().unwrap().to_string())]));
    }

    #[test]
    fn get_url_with_mandatory_fields() {
        let req = SendMessage::new("send_token", "send user", "send message");

        assert_req_url(&req,
                       "messages.json",
                       Some(&[("token", &req.token),
                         ("user", &req.user_key),
                         ("message", &req.message)]));
    }

    #[test]
    fn get_url_with_multiple_devices() {
        let mut req = SendMessage::new("send_token", "send user", "send message");
        req.add_device("device 1");
        req.add_device("device 2");
        req.add_device("device 3");

        let list: String = req.devices.join(",");

        assert_req_url(&req,
                       "messages.json",
                       Some(&[("token", &req.token),
                         ("user", &req.user_key),
                         ("message", &req.message),
                         ("device", &list)]));
    }

    #[test]
    fn get_url_with_emergency_priority_with_callback() {
        let mut req = SendMessage::new("send_token", "send user", "send message");
        req.set_priority(Priority::Emergency {
                             retry: 10,
                             expire: 20,
                             callback_url: Some(String::from("emergency url")),
                         });

        assert_req_url(&req,
                       "messages.json",
                       Some(&[("token", &req.token),
                         ("user", &req.user_key),
                         ("message", &req.message),
                         ("priority", &req.priority.as_ref().unwrap().to_string()),
                         ("retry", "10"),
                         ("expire", "20"),
                         ("callback", "emergency url")]));
    }

    #[test]
    fn get_url_with_emergency_priority_without_callback() {
        let mut req = SendMessage::new("send_token", "send user", "send message");
        req.set_priority(Priority::Emergency{ retry: 10, expire: 20, callback_url: None});

        assert_req_url(&req,
                       "messages.json",
                       Some(&[("token", &req.token),
                         ("user", &req.user_key),
                         ("message", &req.message),
                         ("priority", &req.priority.as_ref().unwrap().to_string()),
                         ("retry", "10"),
                         ("expire", "20")])); 
    }
}