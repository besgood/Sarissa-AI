//! Provides common request templates for POST and JSON-based fuzzing

pub struct Template {
    pub content_type: &'static str,
    pub body: String,
}

pub fn get_json_template(payload: &str) -> Template {
    Template {
        content_type: "application/json",
        body: format!("{{\"username\":\"{}\",\"password\":\"{}\"}}", payload, payload),
    }
}

pub fn get_form_template(payload: &str) -> Template {
    Template {
        content_type: "application/x-www-form-urlencoded",
        body: format!("username={}&password={}", payload, payload),
    }
}

pub fn get_xml_template(payload: &str) -> Template {
    Template {
        content_type: "application/xml",
        body: format!(
            "<credentials><user>{}</user><pass>{}</pass></credentials>",
            payload, payload
        ),
    }
}
