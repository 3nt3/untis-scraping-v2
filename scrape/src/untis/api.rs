use serde::Serialize;

/// Makes a reqest to `/WebUntis/j_spring_security_check` to get a `JSESSIONID`
pub async fn login(school: String, username: String, password: String) -> anyhow::Result<String> {
    #[derive(Serialize, Debug)]
    struct LoginData {
        school: String,
        j_username: String,
        j_password: String,
        token: String,
    }

    let login_data = LoginData {
        school,
        j_username: username,
        j_password: password,
        token: "".to_string(),
    };

    // for some reason it seems to randomly not work sometimes so we just retry 10 times until we
    // succeed
    for i in 1..=10 {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0")
            .build()?;
        let resp = client
            .post("https://hepta.webuntis.com/WebUntis/j_spring_security_check")
            .form(&login_data)
            .header("Accept", "application/json")
            .send()
            .await;

        if resp.is_err() {
            println!("login failed {} times", i);
            continue;
        }

        let cookie_value = resp?
            .cookies()
            .find(|x| x.name() == "JSESSIONID")
            .map(|x| x.value().to_string());

        if cookie_value.is_none() {
            continue;
        }

        return Ok(cookie_value.unwrap().to_string());
    }

    Err(anyhow::Error::msg("max retries of 10 exceeded"))
}

/// Makes a request to `/WebUntis/api/token/new` to get a token
pub async fn new_token(session_id: String) -> anyhow::Result<String> {
    // for some reason it seems to randomly not work sometimes so we just retry 10 times until we
    // succeed
    for i in 1..=10 {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0")
            .build()?;
        let resp = client
            .get("https://hepta.webuntis.com/WebUntis/api/token/new")
            .header("Accept", "application/json")
            .header("Cookie", format!("JSESSIONID={}", session_id))
            .send()
            .await;
        if resp.is_err() {
            println!("getting token failed {} times", i);
            continue;
        }

        return Ok(resp?.text().await?);
    }

    Err(anyhow::Error::msg("max retries of 10 exceeded"))
}
