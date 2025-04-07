use application::{application_run, platform_create, ApplicationConfig, ApplicationState};
use logger::log_fatal;

mod application;
mod errors;
fn main() {
    
    let application_config = ApplicationConfig::new(
        "Enzi 🧱",
        100,
        100,
        1280,
        720,
    );

    
    let initial_state = ApplicationState::new();

    let application_state = match platform_create(application_config, initial_state) {
        Ok(app_state) => app_state,
        Err(err) => {
            log_fatal!("Failure during platform creation: {}", err);
            std::process::exit(1);
        },
    };

    application_run(application_state);

    
}
