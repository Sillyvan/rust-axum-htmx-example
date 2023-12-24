#[cfg(test)]
pub async fn e2e_has_add_cat_form(
    client: &mut fantoccini::Client,
    has_auth: bool,
) -> Result<(), fantoccini::error::CmdError> {
    // check if add cat is on page

    match has_auth {
        true => {
            let is_displayed = client
                .wait()
                .for_element(fantoccini::Locator::Id("add-cat-form"))
                .await?
                .find(fantoccini::Locator::Id("add-cat-form"))
                .await?
                .is_displayed()
                .await?;
            assert!(is_displayed)
        }
        false => {
            let is_not_displayed = client
                .find(fantoccini::Locator::Id("add-cat-form"))
                .await
                .is_err();
            assert!(is_not_displayed)
        }
    }

    Ok(())
}
