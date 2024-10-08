#![no_std]
#![no_main]
#![allow(unused_imports)]

extern crate alloc;


use agb::{
    display::{
        object::{Graphics, OamManaged, Object, Tag, TagMap},
        tiled::{
            InfiniteScrolledMap, PartialUpdateStatus, RegularBackgroundSize, TileFormat, TiledMap,
            VRamManager,
        },
        Priority, HEIGHT, WIDTH,
    }, fixnum::{FixedNum, Vector2D}, include_background_gfx, input::{self, Button, ButtonController}, sound::mixer::Frequency
};
use alloc::boxed::Box;
//use sfx::SfxPlayer;




mod title;

pub struct VaultLayer {
    foreground: &'static [u16],
    background: &'static [u16],
    dimensions: Vector2D<u32>,
    //colliosion: &'static [u32],
    start_pos: (i32, i32),


}

static GRAPHICS: &Graphics = agb::include_aseprite!("gfx/player.aseprite");
static ENVIRONMENT_TILES: &Graphics = agb::include_aseprite!("gfx/environment_tiles.aseprite");
agb::include_background_gfx!(environment_tiles_bgfx, "2ce8f4", background => deduplicate "gfx/environment_tiles.png");
static TAG_MAP: &TagMap = GRAPHICS.tags();
static ENVIRONMENT_TAGS: &TagMap = ENVIRONMENT_TILES.tags();

static LFACING: &Tag = TAG_MAP.get("LFacing");
static RFACING: &Tag = TAG_MAP.get("RFacing");
static WALL_TILE: &Tag = ENVIRONMENT_TAGS.get("Wall");
static FLOOR_TILE: &Tag = ENVIRONMENT_TAGS.get("Floor");



type FixedNumberType = FixedNum<10>;

pub struct Entity<'a> {
    sprite: Object<'a>,
    position: Vector2D<FixedNumberType>,
    velocity: Vector2D<FixedNumberType>,
    //probably for determining which bits are collidable for a sprite?
    collision_mask: Vector2D<u16>,

}

impl<'a> Entity<'a> {
    pub fn new(object: &'a OamManaged, collision_mask: Vector2D<u16>) -> Self {
        let mut dummy_object = object.object_sprite(LFACING.sprite(0));
        dummy_object.set_priority(Priority::P1);
        Entity {
            sprite: dummy_object,
            collision_mask,
            position: (0, 0).into(),
            velocity: (0, 0).into(),
        }
    }

}

pub struct Player<'a> {
    p_entity: Entity<'a>,
    is_on_ground: bool,
    facing: input::Tri,
}

impl<'a> Player<'a> {
    fn new(controller: &'a OamManaged, start_position: Vector2D<FixedNumberType>) -> Self {
        let mut p_entity = Entity::new(controller, (0_u16, 0_u16).into());
        
        p_entity.sprite.set_sprite(controller.sprite(RFACING.sprite(0)));

        Player {
            p_entity,
            is_on_ground: false,
            facing: input::Tri::Zero,
        }
    }

    fn update_frame(
        &mut self,
        input: &ButtonController,
        controller: &'a OamManaged,
        //timer: i32,
        ) {
        
        match input.x_tri() {
            input::Tri::Negative => { /*move left*/ },
            input::Tri::Positive => { /*move right*/ },
            input::Tri::Zero => { /**/ },
        }
    }
}

//entered via main.rs and implicit lib.rs module
//should never return
pub fn main(mut agb: agb::Gba) -> ! {

    let (tiled, mut vram) = agb.display.video.tiled0();
    vram.set_background_palettes(environment_tiles_bgfx::PALETTES);

    let mut splash_screen_background = tiled.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
        );


    let mut world_background = tiled.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
        );


    let tileset = &environment_tiles_bgfx::background.tiles;

    for y in 0..32u16 {
        for x in 0..32u16 {
            world_background.set_tile(
                &mut vram,
                (x, y),
                tileset,
                environment_tiles_bgfx::background.tile_settings[2],
            );
        }
    }



    world_background.commit(&mut vram);
    world_background.set_visible(true);
    
    
    //what is tilesheet
    //vram.set_background_palettes(tile_sheet::PALETTES);

    //create 2 backgrounds
    //fill world_display (bg2) with tiles via set_tile
    //
    //mixer/sfx stuff
    //
    //commit world_display to vram
    //show splash
    //
    //game loop
    //
    //goal: get a loop going that cycles between a few sprites?
    //
    //
    //


    //title::show_splash_screen(title::SplashScreen::Title, &mut splash_screen_background, &mut vram);


    loop {
    /*
        world_background.set_tile(
            &mut vram,
            (x, y),
            tileset,
            environment_tiles_bgfx::background.tile_settings[0],
        );

    */

    }

}


