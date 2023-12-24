#[cfg(test)]
pub async fn e2e_sign_out(
    client: &mut fantoccini::Client,
) -> Result<(), fantoccini::error::CmdError> {
    client
        .find(fantoccini::Locator::XPath(
            "/html/body/header/nav/ul[2]/li[2]/a",
        ))
        .await?
        .click()
        .await?;

    assert_eq!(client.get_all_cookies().await?.len(), 0);
    Ok(())
}
