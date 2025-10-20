// 实体总数：91
// 关系总数：约200

// [实体] Module: bevy::core (通过 use 导入)
// [关系] Use: 使用 bevy::core 模块中的 FixedTimestep
use bevy::core::FixedTimestep;
// [实体] Module: bevy::prelude (通过 use 导入)
// [关系] Use: 使用 bevy::prelude 模块中的所有项
use bevy::prelude::*;
// [实体] Module: rand::prelude (通过 use 导入)
// [关系] Use: 使用 rand::prelude 模块中的 random
use rand::prelude::random;

// [实体] Constant: SNAKE_HEAD_COLOR
// [关系] Define: 定义常量 SNAKE_HEAD_COLOR
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
// [实体] Constant: FOOD_COLOR
// [关系] Define: 定义常量 FOOD_COLOR
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
// [实体] Constant: SNAKE_SEGMENT_COLOR
// [关系] Define: 定义常量 SNAKE_SEGMENT_COLOR
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

// [实体] Constant: ARENA_WIDTH
// [关系] Define: 定义常量 ARENA_WIDTH
const ARENA_WIDTH: u32 = 10;
// [实体] Constant: ARENA_HEIGHT
// [关系] Define: 定义常量 ARENA_HEIGHT
const ARENA_HEIGHT: u32 = 10;

// [实体] Enum: SnakeMovement
// [关系] Define: 定义枚举 SnakeMovement
// [关系] Inherit: 通过 #[derive(SystemLabel, Clone, Debug, Hash, PartialEq, Eq)] 继承多个 trait 的行为
#[derive(SystemLabel, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

// [实体] Struct: Position
// [关系] Define: 定义结构体 Position
// [关系] Inherit: 通过 #[derive(Component, Clone, Copy, PartialEq, Eq)] 继承多个 trait 的行为
// [关系] Contain: Position 包含 x, y 字段
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    // [实体] Variable: x (字段)
    x: i32,
    // [实体] Variable: y (字段)
    y: i32,
}

// [实体] Struct: Size
// [关系] Define: 定义结构体 Size
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
// [关系] Contain: Size 包含 width, height 字段
#[derive(Component)]
struct Size {
    // [实体] Variable: width (字段)
    width: f32,
    // [实体] Variable: height (字段)
    height: f32,
}

// [关系] Impl: 为 Size 实现方法
impl Size {
    // [实体] Function/Method: square
    // [关系] Define: 定义方法 square
    // [实体] Variable: x (参数)
    pub fn square(x: f32) -> Self {
        // [关系] Call: 调用 Size 结构体字面量
        // [关系] UseVar: 使用变量 width, height, x
        Self {
            width: x,
            height: x,
        }
    }
}

// [实体] Struct: SnakeHead
// [关系] Define: 定义结构体 SnakeHead
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
// [关系] Contain: SnakeHead 包含 direction 字段
#[derive(Component)]
struct SnakeHead {
    // [实体] Variable: direction (字段)
    direction: Direction,
}

// [实体] Struct: GameOverEvent
// [关系] Define: 定义结构体 GameOverEvent
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
#[derive(Component)]
struct GameOverEvent;

// [实体] Struct: GrowthEvent
// [关系] Define: 定义结构体 GrowthEvent
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
#[derive(Component)]
struct GrowthEvent;

// [实体] Struct: LastTailPosition
// [关系] Define: 定义结构体 LastTailPosition
// [关系] Inherit: 通过 #[derive(Default)] 继承 Default trait 的行为
// [关系] Contain: LastTailPosition 包含 Option<Position> 字段
#[derive(Default)]
struct LastTailPosition(Option<Position>);

// [实体] Struct: SnakeSegment
// [关系] Define: 定义结构体 SnakeSegment
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
#[derive(Component)]
struct SnakeSegment;

// [实体] Struct: SnakeSegments
// [关系] Define: 定义结构体 SnakeSegments
// [关系] Inherit: 通过 #[derive(Default)] 继承 Default trait 的行为
// [关系] Contain: SnakeSegments 包含 Vec<Entity> 字段
#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

// [实体] Struct: Food
// [关系] Define: 定义结构体 Food
// [关系] Inherit: 通过 #[derive(Component)] 继承 Component trait 的行为
#[derive(Component)]
struct Food;

// [实体] Enum: Direction
// [关系] Define: 定义枚举 Direction
// [关系] Inherit: 通过 #[derive(PartialEq, Clone, Copy)] 继承多个 trait 的行为
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

// [关系] Impl: 为 Direction 实现方法
impl Direction {
    // [实体] Function/Method: opposite
    // [关系] Define: 定义方法 opposite
    fn opposite(self) -> Self {
        // [关系] UseVar: 使用变量 self
        match self {
            // [关系] UseVar: 使用变量 Self::Left, Self::Right
            Self::Left => Self::Right,
            // [关系] UseVar: 使用变量 Self::Right, Self::Left
            Self::Right => Self::Left,
            // [关系] UseVar: 使用变量 Self::Up, Self::Down
            Self::Up => Self::Down,
            // [关系] UseVar: 使用变量 Self::Down, Self::Up
            Self::Down => Self::Up,
        }
    }
}

// [实体] Function: setup_camera
// [关系] Define: 定义函数 setup_camera
// [实体] Variable: mut commands (参数)
fn setup_camera(mut commands: Commands) {
    // [关系] Call: 调用 commands.spawn_bundle, OrthographicCameraBundle::new_2d
    // [关系] UseVar: 使用变量 commands
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// [实体] Function: spawn_snake
// [关系] Define: 定义函数 spawn_snake
// [实体] Variable: mut commands (参数)
// [实体] Variable: mut segments (参数)
fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    // [关系] Modify: 修改 segments.0
    // [关系] Call: 调用 vec!, commands.spawn_bundle, SpriteBundle, Sprite, Default::default, insert, id, spawn_segment
    // [关系] UseVar: 使用变量 segments.0, commands, segments, SNAKE_HEAD_COLOR, SnakeHead, Direction::Up, SnakeSegment, Position, Size::square
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ];
}

// [实体] Function: spawn_segment
// [关系] Define: 定义函数 spawn_segment
// [实体] Variable: mut commands (参数)
// [实体] Variable: position (参数)
fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    // [关系] Call: 调用 commands.spawn_bundle, SpriteBundle, Sprite, Default::default, insert, id
    // [关系] UseVar: 使用变量 commands, position, SNAKE_SEGMENT_COLOR, SnakeSegment, Size::square
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

// [实体] Function: snake_movement
// [关系] Define: 定义函数 snake_movement
// [实体] Variable: mut last_tail_position (参数)
// [实体] Variable: mut game_over_writer (参数)
// [实体] Variable: segments (参数)
// [实体] Variable: mut heads (参数)
// [实体] Variable: mut positions (参数)
// [实体] Variable: head_entity (if let 变量)
// [实体] Variable: head (if let 变量)
// [实体] Variable: segment_positions (局部变量)
// [实体] Variable: head_pos (局部变量)
fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    // [关系] Call: 调用 heads.iter_mut().next()
    // [关系] UseVar: 使用变量 heads
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // [关系] Call: 调用 segments.0.iter(), map, positions.get_mut, collect
        // [关系] UseVar: 使用变量 segments.0, positions
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        // [关系] Call: 调用 positions.get_mut
        // [关系] UseVar: 使用变量 head_entity, positions
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        // [关系] UseVar: 使用变量 head.direction
        match &head.direction {
            // [关系] Modify: 修改 head_pos.x
            // [关系] UseVar: 使用变量 head_pos.x
            Direction::Left => {
                head_pos.x -= 1;
            }
            // [关系] Modify: 修改 head_pos.x
            // [关系] UseVar: 使用变量 head_pos.x
            Direction::Right => {
                head_pos.x += 1;
            }
            // [关系] Modify: 修改 head_pos.y
            // [关系] UseVar: 使用变量 head_pos.y
            Direction::Up => {
                head_pos.y += 1;
            }
            // [关系] Modify: 修改 head_pos.y
            // [关系] UseVar: 使用变量 head_pos.y
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        // [关系] UseVar: 使用变量 head_pos.x, head_pos.y, 常量 ARENA_WIDTH, ARENA_HEIGHT
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            // [关系] Call: 调用 game_over_writer.send
            // [关系] UseVar: 使用变量 game_over_writer
            game_over_writer.send(GameOverEvent); // 边界
        }
        // [关系] Call: 调用 segment_positions.contains
        // [关系] UseVar: 使用变量 segment_positions, head_pos
        if segment_positions.contains(&head_pos) {
            // [关系] Call: 调用 game_over_writer.send
            // [关系] UseVar: 使用变量 game_over_writer
            game_over_writer.send(GameOverEvent); // 蛇身体
        }
        // [关系] Call: 调用 segment_positions.iter(), segments.0.iter().skip, zip, for_each, positions.get_mut
        // [关系] Modify: 修改 positions.get_mut(*segment).unwrap()
        // [关系] UseVar: 使用变量 segment_positions, segments.0
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        // [关系] Modify: 修改 last_tail_position.0
        // [关系] Call: 调用 segment_positions.last
        // [关系] UseVar: 使用变量 last_tail_position.0, segment_positions
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

// [实体] Function: snake_movement_input
// [关系] Define: 定义函数 snake_movement_input
// [实体] Variable: keyboard_input (参数)
// [实体] Variable: mut heads (参数)
// [实体] Variable: mut head (if let 变量)
// [实体] Variable: dir (局部变量)
fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    // [关系] Call: 调用 heads.iter_mut().next()
    // [关系] UseVar: 使用变量 heads
    if let Some(mut head) = heads.iter_mut().next() {
        // [关系] Call: 调用 keyboard_input.pressed
        // [关系] UseVar: 使用变量 keyboard_input, KeyCode::Left, KeyCode::Down, KeyCode::Up, KeyCode::Right, head.direction
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        // [关系] Call: 调用 head.direction.opposite
        // [关系] UseVar: 使用变量 dir, head.direction
        if dir != head.direction.opposite() {
            // [关系] Modify: 修改 head.direction
            // [关系] UseVar: 使用变量 head.direction, dir
            head.direction = dir;
        }
    }
}

// [实体] Function: game_over
// [关系] Define: 定义函数 game_over
// [实体] Variable: mut commands (参数)
// [实体] Variable: mut reader (参数)
// [实体] Variable: segments_res (参数)
// [实体] Variable: food (参数)
// [实体] Variable: segments (参数)
// [实体] Variable: ent (for 循环变量)
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    // [关系] Call: 调用 reader.iter().next()
    // [关系] UseVar: 使用变量 reader
    if reader.iter().next().is_some() {
        // [关系] Call: 调用 food.iter(), segments.iter(), chain
        // [关系] UseVar: 使用变量 food, segments
        for ent in food.iter().chain(segments.iter()) {
            // [关系] Call: 调用 commands.entity, despawn
            // [关系] UseVar: 使用变量 commands, ent
            commands.entity(ent).despawn();
        }
        // [关系] Call: 调用 spawn_snake
        // [关系] UseVar: 使用变量 commands, segments_res
        spawn_snake(commands, segments_res);
    }
}

// [实体] Function: snake_eating
// [关系] Define: 定义函数 snake_eating
// [实体] Variable: mut commands (参数)
// [实体] Variable: mut growth_writer (参数)
// [实体] Variable: food_positions (参数)
// [实体] Variable: head_positions (参数)
// [实体] Variable: head_pos (for 循环变量)
// [实体] Variable: ent (for 循环变量)
// [实体] Variable: food_pos (for 循环变量)
fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    // [关系] Call: 调用 head_positions.iter()
    // [关系] UseVar: 使用变量 head_positions
    for head_pos in head_positions.iter() {
        // [关系] Call: 调用 food_positions.iter()
        // [关系] UseVar: 使用变量 food_positions
        for (ent, food_pos) in food_positions.iter() {
            // [关系] UseVar: 使用变量 food_pos, head_pos
            if food_pos == head_pos {
                // [关系] Call: 调用 commands.entity, despawn
                // [关系] UseVar: 使用变量 commands, ent
                commands.entity(ent).despawn();
                // [关系] Call: 调用 growth_writer.send
                // [关系] UseVar: 使用变量 growth_writer
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

// [实体] Function: snake_growth
// [关系] Define: 定义函数 snake_growth
// [实体] Variable: commands (参数)
// [实体] Variable: last_tail_position (参数)
// [实体] Variable: mut segments (参数)
// [实体] Variable: mut growth_reader (参数)
fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    // [关系] Call: 调用 growth_reader.iter().next()
    // [关系] UseVar: 使用变量 growth_reader
    if growth_reader.iter().next().is_some() {
        // [关系] Modify: 修改 segments.0
        // [关系] Call: 调用 segments.0.push, spawn_segment
        // [关系] UseVar: 使用变量 segments.0, commands, last_tail_position.0
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

// [实体] Function: size_scaling
// [关系] Define: 定义函数 size_scaling
// [实体] Variable: windows (参数)
// [实体] Variable: mut q (参数)
// [实体] Variable: window (局部变量)
// [实体] Variable: sprite_size (for 循环变量)
// [实体] Variable: mut transform (for 循环变量)
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    // [关系] Call: 调用 windows.get_primary
    // [关系] UseVar: 使用变量 windows
    let window = windows.get_primary().unwrap();
    // [关系] Call: 调用 q.iter_mut()
    // [关系] UseVar: 使用变量 q
    for (sprite_size, mut transform) in q.iter_mut() {
        // [关系] Call: 调用 Vec3::new
        // [关系] UseVar: 使用变量 transform.scale, sprite_size.width, sprite_size.height, 常量 ARENA_WIDTH, ARENA_HEIGHT, window.width(), window.height()
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

// [实体] Function: position_translation
// [关系] Define: 定义函数 position_translation
// [实体] Variable: windows (参数)
// [实体] Variable: mut q (参数)
// [实体] Variable: window (局部变量)
// [实体] Variable: pos (for 循环变量)
// [实体] Variable: mut transform (for 循环变量)
// [实体] Function: convert (内部函数)
// [实体] Variable: pos (convert 参数)
// [实体] Variable: bound_window (convert 参数)
// [实体] Variable: bound_game (convert 参数)
// [实体] Variable: tile_size (convert 局部变量)
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    // 内部函数 convert
    // [关系] Define: 定义函数 convert
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        // [关系] UseVar: 使用变量 pos, bound_window, bound_game
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    // [关系] Call: 调用 windows.get_primary
    // [关系] UseVar: 使用变量 windows
    let window = windows.get_primary().unwrap();
    // [关系] Call: 调用 q.iter_mut()
    // [关系] UseVar: 使用变量 q
    for (pos, mut transform) in q.iter_mut() {
        // [关系] Call: 调用 Vec3::new, convert
        // [关系] UseVar: 使用变量 transform.translation, pos.x, pos.y, window.width(), window.height(), 常量 ARENA_WIDTH, ARENA_HEIGHT
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

// [实体] Function: food_spawner
// [关系] Define: 定义函数 food_spawner
// [实体] Variable: mut commands (参数)
// [实体] Variable: x (Position 字面量字段)
// [实体] Variable: y (Position 字面量字段)
fn food_spawner(mut commands: Commands) {
    // [关系] Call: 调用 commands.spawn_bundle, SpriteBundle, Sprite, Default::default, insert, Position, Size::square, random
    // [关系] UseVar: 使用变量 commands, FOOD_COLOR, x, y, ARENA_WIDTH, ARENA_HEIGHT
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}
