#[cfg(test)]
pub async fn e2e_sign_in(
    client: &mut fantoccini::Client,
    mock_user: &str,
) -> Result<(), fantoccini::error::CmdError> {
    client
        .find(fantoccini::Locator::LinkText("Sign In"))
        .await?
        .click()
        .await?;

    client
        .find(fantoccini::Locator::Id("username"))
        .await?
        .send_keys(mock_user)
        .await?;

    client
        .find(fantoccini::Locator::Id("password"))
        .await?
        .send_keys(mock_user)
        .await?;

    // click on the submit button by type
    client
        .find(fantoccini::Locator::Css("input[type=submit]"))
        .await?
        .click()
        .await?;

    // Finally, close the client
    Ok(())
}

#[cfg(test)]
pub async fn e2e_sign_in_wrong_login(
    client: &mut fantoccini::Client,
) -> Result<(), fantoccini::error::CmdError> {
    client
        .find(fantoccini::Locator::LinkText("Sign In"))
        .await?
        .click()
        .await?;

    client
        .find(fantoccini::Locator::Id("username"))
        .await?
        .send_keys("admin")
        .await?;

    client
        .find(fantoccini::Locator::Id("password"))
        .await?
        .send_keys("admin2")
        .await?;

    // click on the submit button by type
    client
        .find(fantoccini::Locator::Css("input[type=submit]"))
        .await?
        .click()
        .await?;

    let error = client
        .wait()
        .for_element(fantoccini::Locator::Id("signin-error"))
        .await?
        .find(fantoccini::Locator::Id("signin-error"))
        .await?
        .is_displayed()
        .await?;

    assert!(error);

    // Finally, close the client
    Ok(())
}
