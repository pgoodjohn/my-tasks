pub fn _setup_database() -> Result<(), String> {
    // log::debug!("Initializing old db");
    // let manager = SqliteConnectionManager::file(std::path::PathBuf::from(
    //     configuration::Configuration::db_path(cfg!(debug_assertions)),
    // ));
    // log::debug!("DB Was initialized");

    // match r2d2::Pool::new(manager) {
    //     Ok(pool) => {
    //         // setup_structure(&pool, configuration).unwrap();
    //         log::debug!("Pool Was initialized");
    //         Ok(pool)
    //     }
    //     Err(e) => {
    //         log::error!("Could not initialize db: {:?}", e);
    //         Err(String::from("Could not initialize database"))
    //     }
    // }
    Ok(())
}
