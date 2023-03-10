use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{
        to_cp437, BTerm, BTermBuilder, FontCharType, GameState, Point, BLACK, RED, RGB, YELLOW,
    },
};
use specs::prelude::*;

mod monster_ai_system;
use monster_ai_system::*;
mod visibility_system;
pub use visibility_system::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
//pub use rect::Rect;

pub struct State {
    ecs: World,
    pub runstate: RunState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.run_systems();
        self.runstate = player_input(self, ctx);

        // Using this instead of two lines above, makes the game very slow for some reason ??
        //        if self.runstate == RunState::Running {
        //            self.run_systems();
        //            self.runstate = RunState::Paused;
        //        } else {
        //            self.runstate = player_input(self, ctx);
        //        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    bracket_lib::terminal::main_loop(context, gs).expect("program crashed");
}
