#[cfg(test)]
pub async fn e2e_add_cat(
    client: &mut fantoccini::Client,
) -> Result<(), fantoccini::error::CmdError> {
    // check if add cat is on page

    // enter name in input field
    client
        .find(fantoccini::Locator::XPath(
            "/html/body/main/div/form/input[1]",
        ))
        .await?
        .send_keys("meow")
        .await?;
    // enter breed in input field
    client
        .find(fantoccini::Locator::XPath(
            "/html/body/main/div/form/input[2]",
        ))
        .await?
        .send_keys("meow")
        .await?;

    // click on the submit button by type
    client
        .find(fantoccini::Locator::XPath(
            "/html/body/main/div/form/input[3]",
        ))
        .await?
        .click()
        .await?;

    Ok(())
}
