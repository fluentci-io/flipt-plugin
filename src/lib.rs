use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        version
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![&format!(
            "type flipt > /dev/null || pkgx install flipt@{}",
            version
        )])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn bundle(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("bundle")?
        .pkgx()?
        .with_packages(vec!["flipt"])?
        .with_exec(vec!["flipt", "bundle", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn validate(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("validate")?
        .pkgx()?
        .with_packages(vec!["flipt"])?
        .with_exec(vec!["flipt", "validate", &args])?
        .stdout()?;
    Ok(stdout)
}
