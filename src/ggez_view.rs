use ggez::{
    ContextBuilder,
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    input::keyboard::KeyCode,
    mint::Point2,
};

use crate::{Direction, Robot, World, interface};
use std::{
    f32::consts::PI,
    sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
    time::{Duration, Instant},
};

const MARGIN: f32 = 0.1;

pub struct GgezView {
    sender: SyncSender<(World, Vec<Robot>)>,
}

impl GgezView {
    pub fn spawn() -> (Self, Receiver<(World, Vec<Robot>)>) {
        let (sender, receiver) = sync_channel(0);

        (Self { sender }, receiver)
    }
}

impl interface::Display for GgezView {
    fn draw(&self, w: &World, bots: &mut dyn Iterator<Item = &Robot>) {
        // If the window is gone there is nothing we can do
        let _ = self.sender.send((w.clone(), bots.cloned().collect()));
    }
}

pub struct Karel {
    receiver: Receiver<(World, Vec<Robot>)>,
    world: World,
    robots: Vec<Robot>,
    scale: f32,
    wait_time: Duration,
    last_update: Instant,
    is_done: bool,

    meshes: Meshes,
}

impl Karel {
    pub fn run(receiver: Receiver<(World, Vec<Robot>)>) {
        // Wait for initial state
        let Ok((world, robots)) = receiver.recv() else {
            // There is no world...
            return;
        };

        let scale = 20.0;
        let window_mode = WindowMode {
            width: (world.width() as f32 + 2.0 * MARGIN) * scale,
            height: (world.height() as f32 + 2.0 * MARGIN) * scale,
            resizable: true,
            ..Default::default()
        };

        let setup = WindowSetup {
            title: "Karel".into(),
            vsync: true, // Turn to false to increase speed
            ..Default::default()
        };

        // Make a Context.
        let (ctx, event_loop) = ContextBuilder::new("Karel", "Trifecta Tech Foundation")
            .window_setup(setup)
            .window_mode(window_mode)
            .build()
            .expect("aieee, could not create ggez context!");

        let meshes = Meshes::new(scale, &world, &ctx);

        // Create an instance of your event handler.
        // Usually, you should provide it with the Context object to
        // use when setting your game up.
        let my_game = Self {
            receiver,
            world,
            robots,
            scale,
            wait_time: Duration::from_millis(100),
            last_update: Instant::now(),
            meshes,
            is_done: false,
        };

        // Run!
        event::run(ctx, event_loop, my_game);
    }
}

impl EventHandler for Karel {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if self.last_update.elapsed() < self.wait_time {
            return Ok(());
        }

        match self.receiver.try_recv() {
            Ok((world, robots)) => {
                self.world = world;
                self.robots = robots;
                self.last_update = Instant::now();
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.is_done = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let scale = self.scale;
        let scaled = |x: usize| x as f32 * scale;

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Offset the screen to create a margin
        if let Some(screen) = canvas.screen_coordinates() {
            canvas.set_screen_coordinates(Rect {
                x: -scale * MARGIN,
                y: -scale * MARGIN,
                ..screen
            });
        }

        // Draw vertical grid lines
        for x in 1..self.world.width() {
            canvas.draw(&self.meshes.v_grid, DrawParam::new().dest([scaled(x), 0.0]));
        }

        // Draw horizontal grid lines
        for y in 1..self.world.height() {
            canvas.draw(&self.meshes.h_grid, DrawParam::new().dest([0.0, scaled(y)]));
        }

        // Draw walls at the top of the world
        for x in 0..self.world.width() {
            if self.world.walls(0, x).has(Direction::North) {
                canvas.draw(
                    &self.meshes.wall,
                    DrawParam::new()
                        .dest([scaled(x) + scale, -0.05 * scale])
                        .rotation(PI / 2.0),
                );
            }
        }

        // Draw walls at the left side of the world
        for y in 0..self.world.height() {
            if self.world.walls(y, 0).has(Direction::West) {
                canvas.draw(
                    &self.meshes.wall,
                    DrawParam::new().dest([-0.05 * scale, scaled(y)]),
                );
            }
        }

        // Draw all east and south walls
        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                let walls = self.world.walls(y, x);
                if walls.has(Direction::East) {
                    canvas.draw(
                        &self.meshes.wall,
                        DrawParam::new().dest([scaled(x) + 0.95 * scale, scaled(y)]),
                    );
                }

                if walls.has(Direction::South) {
                    canvas.draw(
                        &self.meshes.wall,
                        DrawParam::new()
                            .dest([scaled(x) + scale, scaled(y) + 0.95 * scale])
                            .rotation(PI / 2.0),
                    );
                }
            }
        }

        // Draw crabs
        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                if self.world.has_shell(y, x) {
                    canvas.draw(
                        &self.meshes.crab,
                        DrawParam::new().dest([scaled(x), scaled(y)]),
                    );
                }
            }
        }

        for robo in &self.robots {
            canvas.draw(
                &self.meshes.robo,
                DrawParam::new()
                    .dest([
                        scaled(robo.pos.1) + scale * 0.5,
                        scaled(robo.pos.0) + scale * 0.5,
                    ])
                    .rotation(match robo.dir {
                        Direction::North => PI,
                        Direction::South => 0.0,
                        Direction::West => PI / 2.0,
                        Direction::East => 3.0 * PI / 2.0,
                    }),
            );
        }

        if self.is_done {
            let mut text = Text::new("Simulation finished\nPress Esc to close");
            text.set_scale(scale * 3.0)
                .set_layout(graphics::TextLayout {
                    h_align: graphics::TextAlign::Middle,
                    v_align: graphics::TextAlign::Middle,
                });
            canvas.draw(
                &text,
                DrawParam::new()
                    .color(Color::CYAN)
                    .dest(canvas.screen_coordinates().unwrap().center()),
            );
        }

        // Draw code here...
        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        let Some(keycode) = input.keycode else {
            return Ok(());
        };

        match keycode {
            KeyCode::Escape => ctx.request_quit(),
            KeyCode::Plus => {
                self.wait_time = self.wait_time.saturating_sub(Duration::from_millis(100))
            }
            KeyCode::Minus => {
                self.wait_time = self.wait_time.saturating_add(Duration::from_millis(100))
            }
            _ => {}
        };

        Ok(())
    }

    fn resize_event(
        &mut self,
        ctx: &mut ggez::Context,
        width: f32,
        height: f32,
    ) -> Result<(), ggez::GameError> {
        let width_scale = width / (self.world.width() as f32 + 2.0 * MARGIN);
        let height_scale = height / (self.world.height() as f32 + 2.0 * MARGIN);

        self.scale = width_scale.min(height_scale);
        self.meshes = Meshes::new(self.scale, &self.world, ctx);

        Ok(())
    }
}

struct Meshes {
    robo: Mesh,
    wall: Mesh,
    crab: Mesh,
    v_grid: Mesh,
    h_grid: Mesh,
}

impl Meshes {
    pub fn new(scale: f32, world: &World, ctx: &ggez::Context) -> Self {
        Self {
            robo: Self::robo_mesh(scale, ctx),
            wall: Self::wall_mesh(scale, ctx),
            crab: Self::crab_mesh(scale, ctx),
            v_grid: Self::v_grid(scale, world, ctx),
            h_grid: Self::h_grid(scale, world, ctx),
        }
    }

    fn robo_mesh(scale: f32, ctx: &ggez::Context) -> Mesh {
        let robo_scale = scale * 0.9;
        Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Point2 {
                    x: 0.0 * robo_scale,
                    y: 0.5 * robo_scale,
                },
                Point2 {
                    x: 0.5 * robo_scale,
                    y: -0.5 * robo_scale,
                },
                Point2 {
                    x: -0.5 * robo_scale,
                    y: -0.5 * robo_scale,
                },
            ],
            Color::GREEN,
        )
        .unwrap()
    }

    fn crab_mesh(scale: f32, ctx: &ggez::Context) -> Mesh {
        let crab_bb = Rect::new(0.1 * scale, 0.1 * scale, 0.8 * scale, 0.8 * scale);
        Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), crab_bb, 0.25 * scale, Color::RED)
            .unwrap()
    }

    fn wall_mesh(scale: f32, ctx: &ggez::Context) -> Mesh {
        Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.05 * scale, 0.1 * scale, 0.9 * scale),
            0.1 * scale,
            Color::BLACK,
        )
        .unwrap()
    }

    fn gray() -> Color {
        Color::from_rgb(0xD0, 0xD0, 0xD0)
    }

    fn v_grid(scale: f32, world: &World, ctx: &ggez::Context) -> Mesh {
        Mesh::new_line(
            ctx,
            &[[0.0, 1.0], [0.0, world.height() as f32 * scale]],
            1.0,
            Self::gray(),
        )
        .unwrap()
    }

    fn h_grid(scale: f32, world: &World, ctx: &ggez::Context) -> Mesh {
        Mesh::new_line(
            ctx,
            &[[1.0, 0.0], [world.width() as f32 * scale, 0.0]],
            1.0,
            Self::gray(),
        )
        .unwrap()
    }
}
