use caevern_sdk::game::{self, log};

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    log("Initializing Portal Module");

    let portal = game::get_element_by_name("portal");
    let player = game::get_local_player();
    if let Some(portal) = portal {
        log(&format!("Portal ID: {}", portal.get_id()));

        let portal_position = portal.get_position();
        log(&format!("Portal position: {:?}", portal_position));

        let player_position = player.get_position();
        log(&format!("Player position: {:?}", player_position));
    } else {
        log("No portal found");
    }

    log("Portal Module has started");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(_dt: f32) {
    return;
}
