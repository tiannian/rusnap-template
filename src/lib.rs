rusnap::entry!({
    use log::Level;

    console_log::init_with_level(Level::Debug).expect("Failed to init log");
});
