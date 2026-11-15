use caevern_sdk::game::{self, log};

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    log("Initializing Portal Module");

    let portal = game::get_element_by_name("portal");
    if let Some(portal) = portal {
        log(&format!("Portal: {}", portal.get_id()));
        portal.set_position(6.0, 1.0, 2.0);
    } else {
        log("No portal found");
    }

    log("Portal Module has started");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    log(&format!("I'm a module... {}", dt));
}
