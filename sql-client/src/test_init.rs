/// Initializes logging for all units tests.  
/// The "ctor" macro ensures this is run once at startup.
#[cfg(test)]
#[ctor::ctor]
fn test_init() {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
}
