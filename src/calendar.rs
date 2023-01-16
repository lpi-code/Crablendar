
pub async fn get_calendar(username : &str, password : &str, url : &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("Refresh calendar\t");
    let client_builder = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0");


    let client = client_builder.build()?;
    
    let index_resp = client.get(url).send().await?;
    if index_resp.status() != 200 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Index failed")));
    }    
    let login_resp = client.post(url.replace("index.php", "login/login.php"))
        .form(&[
            ("Username", username), 
            ("Password", password),
            ("url", ""),
            ("login", "")])
        .send()
        .await?;
    if login_resp.status() != 200 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Login failed")));
    }
    let res = client.get(url)
        .send()
        .await;
    match res {
        Ok(res) => {
            let text = res.text().await?;
            println!("Success !");
            Ok(text)
        },
        Err(e) => {
            println!("Error ! : {}", e);
            Err(Box::new(e))
        }
    }
    
}