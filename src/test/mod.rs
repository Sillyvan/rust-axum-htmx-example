mod e2e_add_cat;
mod e2e_has_add_cat_form;
mod e2e_sign_in;
mod e2e_sign_out;
mod e2e_sign_up;

#[cfg(test)]
const BASE_URL: &str = "http://localhost:5173";
#[cfg(test)]
const WEBDRIVER_URL: &str = "http://localhost:9515";
#[cfg(test)]
const MOCK_LOGIN: &str = "admin";
#[tokio::test]
async fn test_sign_up_with_taken_username() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;

    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_up::e2e_sign_up_taken_username(&mut client).await?;

    Ok(client.close().await?)
}

#[tokio::test]
async fn test_sign_in_with_wrong_login() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;

    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_in::e2e_sign_in_wrong_login(&mut client).await?;

    Ok(client.close().await?)
}

#[tokio::test]
async fn test_sign_up_and_sign_in() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;

    let new_user = "test_runner";

    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_up::e2e_sign_up(&mut client, new_user).await?;
    client
        .wait()
        .for_url(url::Url::parse(&format!("{}/signin", BASE_URL)).unwrap())
        .await?;
    e2e_sign_in::e2e_sign_in(&mut client, new_user).await?;

    Ok(client.close().await?)
}

#[tokio::test]
async fn test_cat_form_hidden() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;
    client.goto(&format!("{}/", BASE_URL)).await?;

    e2e_has_add_cat_form::e2e_has_add_cat_form(&mut client, false).await?;

    Ok(client.close().await?)
}

#[tokio::test]
async fn test_cat_form_visable() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;
    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_in::e2e_sign_in(&mut client, MOCK_LOGIN).await?;
    client
        .wait()
        .for_url(url::Url::parse(&format!("{}/", BASE_URL)).unwrap())
        .await?;
    e2e_has_add_cat_form::e2e_has_add_cat_form(&mut client, true).await?;

    Ok(client.close().await?)
}

/* #[tokio::test]
async fn test_log_out() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;
    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_in::e2e_sign_in(&mut client, MOCK_LOGIN).await?;
    e2e_sign_out::e2e_sign_out(&mut client).await?;

    Ok(client.close().await?)
} */

// login and add a cat. check if the cat is added
#[tokio::test]
async fn test_add_cat() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;
    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_in::e2e_sign_in(&mut client, MOCK_LOGIN).await?;
    client
        .wait()
        .for_url(url::Url::parse(&format!("{}/", BASE_URL)).unwrap())
        .await?;
    e2e_add_cat::e2e_add_cat(&mut client).await?;

    Ok(client.close().await?)
}

// delete a cat and check if the cat is deleted
#[tokio::test]

async fn test_delete_cat() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = fantoccini::ClientBuilder::native()
        .connect(WEBDRIVER_URL)
        .await?;
    client.goto(&format!("{}/", BASE_URL)).await?;
    e2e_sign_in::e2e_sign_in(&mut client, MOCK_LOGIN).await?;
    //e2e delete cat

    Ok(client.close().await?)
}
