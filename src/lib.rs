use caevern_sdk::{
    game::{self, log},
    networking,
    objects::{object::Object, player::Player},
};

static mut PORTAL: Option<Object> = None;
static mut PLAYER: Option<Player> = None;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    log("Initializing Portal Module");

    let portal = game::get_element_by_name("portal");
    let player = game::get_local_player();

    if let Some(portal) = portal {
        log(&format!("Portal ID: {}", portal.get_id()));
        unsafe {
            PORTAL = Some(portal);
        }
    } else {
        log("No portal found");
    }

    unsafe {
        PLAYER = Some(player);
    }

    log("Portal Module has started");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(_dt: f32) {
    let portal = unsafe { PORTAL };
    let player = unsafe { PLAYER };
    if let Some(portal) = portal
        && let Some(player) = player
    {
        let portal_position = portal.get_position();
        let player_position = player.get_position();

        let distance = (portal_position.0 - player_position.0).abs()
            + (portal_position.2 - player_position.2).abs();

        if distance < 0.5 {
            log("Player is near the portal, connecting to server 1");
            networking::connect(1);
        }
    }
}
