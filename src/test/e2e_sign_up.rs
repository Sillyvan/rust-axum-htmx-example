#[cfg(test)]
pub async fn e2e_sign_up(
    client: &mut fantoccini::Client,
    mock_user: &str,
) -> Result<(), fantoccini::error::CmdError> {
    client
        .find(fantoccini::Locator::LinkText("Sign Up"))
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
pub async fn e2e_sign_up_taken_username(
    client: &mut fantoccini::Client,
) -> Result<(), fantoccini::error::CmdError> {
    use crate::test::MOCK_LOGIN;

    client
        .find(fantoccini::Locator::LinkText("Sign Up"))
        .await?
        .click()
        .await?;

    client
        .find(fantoccini::Locator::Id("username"))
        .await?
        .send_keys(MOCK_LOGIN)
        .await?;

    client
        .find(fantoccini::Locator::Id("password"))
        .await?
        .send_keys("wrong")
        .await?;

    // click on the submit button by type
    client
        .find(fantoccini::Locator::Css("input[type=submit]"))
        .await?
        .click()
        .await?;

    //

    let error = client
        .wait()
        .for_element(fantoccini::Locator::Id("signup-error"))
        .await?
        .find(fantoccini::Locator::Id("signup-error"))
        .await?
        .is_displayed()
        .await?;

    assert!(error);

    Ok(())
}
