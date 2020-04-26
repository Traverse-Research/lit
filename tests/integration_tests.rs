use lit::run;

const CRATE_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

/// Runs all of the integration tests in the top-level directory
/// of the repository.
#[test]
fn integration_tests() {
    pretty_env_logger::init();

    run::tests(lit::event_handler::Default::default(), |config| {
        config.add_search_path(format!("{}/integration-tests", CRATE_PATH));
        config.add_extension("txt");
    }).expect("unit test(s) failed");
}
