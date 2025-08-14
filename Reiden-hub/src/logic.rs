use crate::parse_date;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub(crate) id: Option<u64>,
    pub(crate) anonymous: bool,
    pub(crate) name: String,
    password: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Some(0),
            anonymous: true,
            name: "guest".to_string(),
            password: None,
        }
    }
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        Self {
            id: None,
            anonymous: false,
            name,
            password: Some(password),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserSQL {
    name: String,
    password: String,
}

impl UserSQL {
    pub fn new(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            password: user.password.clone().unwrap(),
        }
    }

    pub async fn register(user: UserSQL) -> Option<i32> {
        let returned = reqwest::Client::new()
            .post("http://127.0.0.1:8081/register")
            .json(&user)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        if returned >= 0 {
            return Some(returned);
        } else {
            return None;
        }
    }

    pub async fn validate(user: UserSQL) -> Option<u64> {
        // send name and supposed password and get log_in result
        let returned: i32 = reqwest::Client::new()
            .post("http://127.0.0.1:8081/log_in")
            .json(&user)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        if returned == -1 {
            return None;
        } else {
            return Some(returned as u64);
        }
    }
}

pub async fn add_span(
    start_date: String,
    end_date: String,
    name: String,
    user: User,
) -> Option<crate::span::Span> {
    let parsed_start = parse_date(&start_date);
    let parsed_end = parse_date(&end_date);

    let duration = (parsed_end - parsed_start).num_days().abs();

    if duration <= 1 {
        return None;
    }

    let mut span = crate::span::Span {
        id: None,
        name,
        start_date,
        end_date,
        duration,
        created_by: user.id.unwrap(),
    };

    let id = send_to_server(&span).await;

    span.id = Some(id);
    Some(span)
}

pub async fn send_to_server(span: &crate::span::Span) -> u64 {
    reqwest::Client::new()
        .post("http://127.0.0.1:8081/add_span")
        .json(&span)
        .send()
        .await
        .unwrap()
        // request to add span to db
        .json()
        .await
        .unwrap()
    // get id of added span as a response
}

pub async fn get_spans(user: User) -> Vec<crate::span::Span> {
    //get all spans from db on first launch of page
    reqwest::Client::new()
        .post("http://127.0.0.1:8081/get_spans")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
