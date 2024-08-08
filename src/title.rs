
use agb::display::tiled::{RegularMap, TiledMap, VRamManager};

agb::include_background_gfx!(splash_screens,
                             title => deduplicate "gfx/title.png",
                             );

pub enum SplashScreen {
    Title,
    //End,
}

pub fn show_splash_screen(
    which: SplashScreen,
    //sfx: &mut SFXPlayer,
    map: &mut RegularMap,
    vram: &mut VRamManager,

    ) {

    map.set_scroll_pos((0i16, 0i16));

    let tile_data = match which {
        SplashScreen::Title => &splash_screens::title,
    };

    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    vblank.wait_for_vblank();

    map.fill_with(vram, tile_data);

    map.commit(vram);

    vram.set_background_palettes(splash_screens::PALETTES);
    map.set_visible(true);

    loop {
        input.update();
        if input.is_just_pressed(agb::input::Button::A) {
            break;
        }

        vblank.wait_for_vblank();
    }

    map.set_visible(false);
    map.clear(vram);



}


