// 实体总数：59
// 关系总数：87

// [实体] Module: GameState (通过 use 导入)
// [关系] Use: 使用 GameState
use GameState;
// [实体] Module: bracket_lib::prelude (通过 use 导入)
// [关系] Use: 使用 bracket_lib::prelude 中的所有项
use bracket_lib::prelude::*;

// [实体] Enum: GameMode
// [关系] Define: 定义枚举 GameMode
enum GameMode {
    Menu,
    Playing,
    End,
}

/// 游戏屏幕宽度
// [实体] Constant: SCREEN_WIDTH
// [关系] Define: 定义常量 SCREEN_WIDTH
const SCREEN_WIDTH: i32 = 80;
/// 游戏屏幕高度
// [实体] Constant: SCREEN_HEIGHT
// [关系] Define: 定义常量 SCREEN_HEIGHT
const SCREEN_HEIGHT: i32 = 50;
/// 游戏单位时间，每隔 75 ms 做一些事情
// [实体] Constant: FRAME_DURATION
// [关系] Define: 定义常量 FRAME_DURATION
const FRAME_DURATION: f32 = 75.0;

// [实体] Struct: Player
// [关系] Define: 定义结构体 Player
// [关系] Contain: Player 包含 x, y, velocity 字段
struct Player {
    // [实体] Variable: x (字段)
    x: i32, // 世界空间的横坐标
    // [实体] Variable: y (字段)
    y: i32, // 世界空间的纵坐标
    // [实体] Variable: velocity (字段)
    velocity: f32, // 垂直方向的加速度，向下为正
}

// [关系] Impl: 为 Player 实现方法
impl Player {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    // [实体] Variable: x (参数)
    // [实体] Variable: y (参数)
    fn new(x: i32, y: i32) -> Self {
        Player {
            // [关系] UseVar: 使用变量 x, y
            x,
            y,
            // [关系] UseVar: 使用字面量 0.0
            velocity: 0.0,
        }
    }
    /// 渲染小鸟
    // [实体] Function/Method: render
    // [关系] Define: 定义方法 render
    // [实体] Variable: ctx (参数)
    fn render(&mut self, ctx: &mut BTerm) {
        // [关系] Call: 调用 ctx.set, to_cp437
        // [关系] UseVar: 使用变量 ctx, self.y, 常量 YELLOW, BLACK, '@'
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // [实体] Function/Method: gravity_and_move
    // [关系] Define: 定义方法 gravity_and_move
    fn gravity_and_move(&mut self) {
        // [关系] UseVar: 使用变量 self.velocity
        if self.velocity < 2.0 {
            // [关系] Modify: 修改 self.velocity
            // [关系] UseVar: 使用变量 self.velocity
            self.velocity += 0.2;
        }
        // [关系] Modify: 修改 self.x
        // [关系] UseVar: 使用变量 self.x
        self.x += 1;
        // [关系] Modify: 修改 self.y
        // [关系] UseVar: 使用变量 self.y, self.velocity
        self.y += self.velocity as i32;

        // [关系] UseVar: 使用变量 self.y
        if self.y < 0 {
            // [关系] Modify: 修改 self.y
            // [关系] UseVar: 使用变量 self.y
            self.y = 0;
        }
    }

    // [实体] Function/Method: flap
    // [关系] Define: 定义方法 flap
    fn flap(&mut self) {
        // [关系] Modify: 修改 self.velocity
        // [关系] UseVar: 使用变量 self.velocity
        self.velocity = -2.0;
    }
}

// [实体] Struct: State
// [关系] Define: 定义结构体 State
// [关系] Contain: State 包含 player, frame_time, mode, obstacle, score 字段
struct State {
    // [实体] Variable: player (字段)
    player: Player,
    // [实体] Variable: frame_time (字段)
    frame_time: f32, // 游戏累计时间
    // [实体] Variable: mode (字段)
    mode: GameMode,
    // [实体] Variable: obstacle (字段)
    obstacle: Obstacle,
    // [实体] Variable: score (字段)
    score: i32,
}

// [关系] Impl: 为 State 实现方法
impl State {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    fn new() -> Self {
        State {
            // [关系] Call: 调用 Player::new
            // [关系] UseVar: 使用字面量 5, 25
            player: Player::new(5, 25),
            // [关系] UseVar: 使用字面量 0.0
            frame_time: 0.0,
            // [关系] UseVar: 使用 GameMode::Menu
            mode: GameMode::Menu,
            // [关系] Call: 调用 Obstacle::new
            // [关系] UseVar: 使用常量 SCREEN_WIDTH, 字面量 0
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            // [关系] UseVar: 使用字面量 0
            score: 0,
        }
    }

    // [实体] Function/Method: main_menu
    // [关系] Define: 定义方法 main_menu
    // [实体] Variable: ctx (参数)
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // 清空屏幕
        // [关系] Call: 调用 ctx.cls()
        // [关系] UseVar: 使用变量 ctx
        ctx.cls();
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 5, "Welcome to Flappy Bird"
        ctx.print_centered(5, "Welcome to Flappy Bird");
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 8, "(P) Play Game"
        ctx.print_centered(8, "(P) Play Game");
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 9, "(Q) Quit Game"
        ctx.print_centered(9, "(Q) Quit Game");

        // [实体] Variable: key (if let 变量)
        // [关系] UseVar: 使用变量 ctx.key
        if let Some(key) = ctx.key {
            // [关系] UseVar: 使用变量 key
            match key {
                // [关系] Call: 调用 self.restart()
                // [关系] UseVar: 使用变量 self
                VirtualKeyCode::P => self.restart(),
                // [关系] Modify: 修改 ctx.quitting
                // [关系] UseVar: 使用变量 ctx
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // [实体] Function/Method: play
    // [关系] Define: 定义方法 play
    // [实体] Variable: ctx (参数)
    fn play(&mut self, ctx: &mut BTerm) {
        // 清空屏幕，并设置屏幕的背景颜色
        // [关系] Call: 调用 ctx.cls_bg
        // [关系] UseVar: 使用变量 ctx, 常量 NAVY
        ctx.cls_bg(NAVY);
        // frame_time_ms 记录了每次调用 tick() 所经过的时间
        // [关系] Modify: 修改 self.frame_time
        // [关系] UseVar: 使用变量 self.frame_time, ctx.frame_time_ms
        self.frame_time += ctx.frame_time_ms;
        // 向前移动并且重力增加
        // [关系] UseVar: 使用变量 self.frame_time, 常量 FRAME_DURATION
        if self.frame_time > FRAME_DURATION {
            // [关系] Modify: 修改 self.frame_time
            // [关系] UseVar: 使用变量 self.frame_time
            self.frame_time = 0.0;
            // [关系] Call: 调用 self.player.gravity_and_move()
            // [关系] UseVar: 使用变量 self.player
            self.player.gravity_and_move();
        }
        // 用户点击了空格，往上飞
        // [关系] UseVar: 使用变量 ctx.key
        if let Some(VirtualKeyCode::Space) = ctx.key {
            // [关系] Call: 调用 self.player.flap()
            // [关系] UseVar: 使用变量 self.player
            self.player.flap();
        }
        // 渲染
        // [关系] Call: 调用 self.player.render
        // [关系] UseVar: 使用变量 self.player, ctx
        self.player.render(ctx);
        // [关系] Call: 调用 ctx.print
        // [关系] UseVar: 使用变量 ctx, 字面量 0, 0, "Press SPACE to flap"
        ctx.print(0, 0, "Press SPACE to flap");
        // [关系] Call: 调用 ctx.print, format!
        // [关系] UseVar: 使用变量 ctx, 字面量 0, 1, self.score
        ctx.print(0, 1, &format!("Score: {}", self.score));
        // 渲染障碍物
        // [关系] Call: 调用 self.obstacle.render
        // [关系] UseVar: 使用变量 self.obstacle, ctx, self.player.x
        self.obstacle.render(ctx, self.player.x);
        // 判断是否越过障碍物
        // [关系] UseVar: 使用变量 self.player.x, self.obstacle.x
        if self.player.x > self.obstacle.x {
            // [关系] Modify: 修改 self.score
            // [关系] UseVar: 使用变量 self.score
            self.score += 1;
            // 渲染新的障碍物
            // [关系] Modify: 修改 self.obstacle
            // [关系] Call: 调用 Obstacle::new
            // [关系] UseVar: 使用变量 self.obstacle, self.player.x, 常量 SCREEN_WIDTH, self.score
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        // 如果 y 大于游戏屏幕高度，视为坠地，或者撞到障碍物，则游戏结束
        // [关系] UseVar: 使用变量 self.player.y, 常量 SCREEN_HEIGHT, self.obstacle.hit_obstacle, self.player
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            // [关系] Modify: 修改 self.mode
            // [关系] UseVar: 使用变量 self.mode, GameMode::End
            self.mode = GameMode::End;
        }
    }

    // [实体] Function/Method: dead
    // [关系] Define: 定义方法 dead
    // [实体] Variable: ctx (参数)
    fn dead(&mut self, ctx: &mut BTerm) {
        // [关系] Call: 调用 ctx.cls()
        // [关系] UseVar: 使用变量 ctx
        ctx.cls();
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 5, "You are dead!"
        ctx.print_centered(5, "You are dead!");
        // [关系] Call: 调用 ctx.print_centered, format!
        // [关系] UseVar: 使用变量 ctx, 字面量 6, self.score
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 8, "(P) Play Again"
        ctx.print_centered(8, "(P) Play Again");
        // [关系] Call: 调用 ctx.print_centered
        // [关系] UseVar: 使用变量 ctx, 字面量 9, "(Q) Quit Game"
        ctx.print_centered(9, "(Q) Quit Game");

        // [实体] Variable: key (if let 变量)
        // [关系] UseVar: 使用变量 ctx.key
        if let Some(key) = ctx.key {
            // [关系] UseVar: 使用变量 key
            match key {
                // [关系] Call: 调用 self.restart()
                // [关系] UseVar: 使用变量 self
                VirtualKeyCode::P => self.restart(),
                // [关系] Modify: 修改 ctx.quitting
                // [关系] UseVar: 使用变量 ctx
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // [实体] Function/Method: restart
    // [关系] Define: 定义方法 restart
    fn restart(&mut self) {
        // [关系] Modify: 修改 self.player
        // [关系] Call: 调用 Player::new
        // [关系] UseVar: 使用变量 self.player, 字面量 5, 25
        self.player = Player::new(5, 25);
        // [关系] Modify: 修改 self.frame_time
        // [关系] UseVar: 使用变量 self.frame_time
        self.frame_time = 0.0;
        // [关系] Modify: 修改 self.mode
        // [关系] UseVar: 使用变量 self.mode, GameMode::Playing
        self.mode = GameMode::Playing;
        // [关系] Modify: 修改 self.obstacle
        // [关系] Call: 调用 Obstacle::new
        // [关系] UseVar: 使用变量 self.obstacle, 常量 SCREEN_WIDTH, 字面量 0
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        // [关系] Modify: 修改 self.score
        // [关系] UseVar: 使用变量 self.score
        self.score = 0;
    }
}

// [实体] Trait: GameState (通过 impl 知道其存在)
// [关系] Impl: 实现 GameState trait for State
impl GameState for State {
    // [实体] Function/Method: tick
    // [关系] Define: 定义方法 tick
    // [实体] Variable: ctx (参数)
    fn tick(&mut self, ctx: &mut BTerm) {
        // [关系] UseVar: 使用变量 self.mode
        match self.mode {
            // [关系] Call: 调用 self.main_menu
            // [关系] UseVar: 使用变量 self.mode, GameMode::Menu, ctx
            GameMode::Menu => self.main_menu(ctx),
            // [关系] Call: 调用 self.play
            // [关系] UseVar: 使用变量 self.mode, GameMode::Playing, ctx
            GameMode::Playing => self.play(ctx),
            // [关系] Call: 调用 self.dead
            // [关系] UseVar: 使用变量 self.mode, GameMode::End, ctx
            GameMode::End => self.dead(ctx),
        }
    }
}

// [实体] Struct: Obstacle
// [关系] Define: 定义结构体 Obstacle
// [关系] Contain: Obstacle 包含 x, gap_y, size 字段
struct Obstacle {
    // [实体] Variable: x (字段)
    x: i32, // 世界空间的横坐标
    // [实体] Variable: gap_y (字段)
    gap_y: i32, // 中间的空隙坐标
    // [实体] Variable: size (字段)
    size: i32, // 空隙大小
}

// [关系] Impl: 为 Obstacle 实现方法
impl Obstacle {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    // [实体] Variable: x (参数)
    // [实体] Variable: score (参数)
    // [实体] Variable: random (局部变量)
    fn new(x: i32, score: i32) -> Self {
        // [关系] Call: 调用 RandomNumberGenerator::new()
        // [关系] UseVar: 使用变量 random
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            // [关系] UseVar: 使用变量 x
            x,
            // [关系] Call: 调用 random.range
            // [关系] UseVar: 使用变量 random, 字面量 10, 40
            gap_y: random.range(10, 40), // [10, 40)
            // [关系] Call: 调用 i32::max
            // [关系] UseVar: 使用变量 score, 字面量 2, 20
            size: i32::max(2, 20 - score), // 积分越多，洞越窄
        }
    }
    /// 渲染障碍物
    // [实体] Function/Method: render
    // [关系] Define: 定义方法 render
    // [实体] Variable: ctx (参数)
    // [实体] Variable: player_x (参数)
    // [实体] Variable: screen_x (局部变量)
    // [实体] Variable: half_size (局部变量)
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        // [关系] UseVar: 使用变量 self.x, player_x
        let screen_x = self.x - player_x; // 屏幕空间的横坐标
        // [关系] UseVar: 使用变量 self.size
        let half_size = self.size / 2;

        // 障碍物的上半部分
        // [实体] Variable: y (for 循环变量)
        // [关系] UseVar: 使用变量 self.gap_y, half_size
        for y in 0..self.gap_y - half_size {
            // [关系] Call: 调用 ctx.set, to_cp437
            // [关系] UseVar: 使用变量 ctx, screen_x, y, 常量 RED, BLACK, '|'
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // 障碍物的下半部分
        // [实体] Variable: y (for 循环变量)
        // [关系] UseVar: 使用变量 self.gap_y, half_size, 常量 SCREEN_HEIGHT
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            // [关系] Call: 调用 ctx.set, to_cp437
            // [关系] UseVar: 使用变量 ctx, screen_x, y, 常量 RED, BLACK, '|'
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }
    /// 判断是否撞到
    // [实体] Function/Method: hit_obstacle
    // [关系] Define: 定义方法 hit_obstacle
    // [实体] Variable: player (参数)
    // [实体] Variable: half_size (局部变量)
    // [实体] Variable: does_x_match (局部变量)
    // [实体] Variable: player_above_gap (局部变量)
    // [实体] Variable: player_below_gap (局部变量)
    fn hit_obstacle(&self, player: &Player) -> bool {
        // [关系] UseVar: 使用变量 self.size
        let half_size = self.size / 2;
        // 玩家的 x 坐标和障碍物的坐标是否一样
        // [关系] UseVar: 使用变量 player.x, self.x
        let does_x_match = player.x == self.x;
        // 是否在障碍物的上半部分的坐标范围内
        // [关系] UseVar: 使用变量 player.y, self.gap_y, half_size
        let player_above_gap = player.y < self.gap_y - half_size;
        // 是否在障碍物的下半部分的坐标范围内
        // [关系] UseVar: 使用变量 player.y, self.gap_y, half_size
        let player_below_gap = player.y > self.gap_y + half_size;
        // [关系] UseVar: 使用变量 does_x_match, player_above_gap, player_below_gap
        does_x_match && (player_above_gap || player_below_gap)
    }
}

// [实体] Function: main
// [关系] Define: 定义函数 main
// [实体] Variable: context (局部变量)
fn main() -> BError {
    // [关系] Call: 调用 BTermBuilder::simple80x50, with_title, build
    // [关系] UseVar: 使用变量 context, 字面量 "Flappy Bird"
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Bird")
        .build()?;

    // [关系] Call: 调用 main_loop, State::new
    // [关系] UseVar: 使用变量 context
    main_loop(context, State::new())
}
