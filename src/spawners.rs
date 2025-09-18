use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        pos,
        Player,
        Health {
            current: 20,
            max: 20,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        pos,
        Enemy,
        MovingRandomly,
        Name(name),
        Health {
            current: hp,
            max: hp,
        },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (1, "Orc".to_string(), to_cp437('o'))
}
