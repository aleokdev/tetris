use std::{
    env,
    ops::Range,
    path,
    time::{Duration, Instant},
};

use crevice::std140::AsStd140;

use ggez::{
    audio::{self, SoundSource},
    conf::{WindowMode, WindowSetup},
    event,
    glam::*,
    graphics::{self, Color, DrawParam, InstanceArray, Mesh, MeshData, Quad, Rect, Vertex},
    mint::Point2,
    Context, GameResult,
};
use rand::thread_rng;

#[derive(Clone, Copy)]
pub enum PieceRotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl PieceRotation {
    pub fn rotate_cw(self) -> Self {
        use PieceRotation::*;
        match self {
            Deg0 => Deg90,
            Deg90 => Deg180,
            Deg180 => Deg270,
            Deg270 => Deg0,
        }
    }
    pub fn rotate_ccw(self) -> Self {
        use PieceRotation::*;
        match self {
            Deg0 => Deg270,
            Deg90 => Deg0,
            Deg180 => Deg90,
            Deg270 => Deg180,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PieceKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl PieceKind {
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        let num = rng.gen_range(0..7);
        match num {
            0 => PieceKind::I,
            1 => PieceKind::J,
            2 => PieceKind::L,
            3 => PieceKind::O,
            4 => PieceKind::S,
            5 => PieceKind::T,
            _ => PieceKind::Z,
        }
    }
}

impl PieceKind {
    pub fn get_grid(&self, rotation: PieceRotation) -> Grid {
        let o = None;
        let x = Some(Block {
            color: match self {
                I => Color::CYAN,
                J => Color::BLUE,
                L => Color::RED,
                O => Color::YELLOW,
                S => Color::GREEN,
                T => Color::MAGENTA,
                Z => Color::WHITE,
            },
        });
        let d = |data: [Option<Block>; 16]| Grid::with_data(4, 4, Box::new(data));
        use PieceKind::*;
        use PieceRotation::*;
        match (self, rotation) {
            #[rustfmt::skip]
            (I, Deg0 | Deg180) => d(
                [
                o, o, o, o,
                x, x, x, x,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (I, Deg90 | Deg270) => d(
                [
                o, x, o, o,
                o, x, o, o,
                o, x, o, o,
                o, x, o, o
                ],
            ),
            #[rustfmt::skip]
            (J, Deg0) => d(
                [
                o, x, o, o,
                o, x, o, o,
                x, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (J, Deg90) => d(
                [
                x, o, o, o,
                x, x, x, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (J, Deg180) => d(
                [
                o, x, x, o,
                o, x, o, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (J, Deg270) => d(
                [
                o, o, o, o,
                x, x, x, o,
                o, o, x, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (L, Deg0) => d(
                [
                o, x, o, o,
                o, x, o, o,
                o, x, x, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (L, Deg90) => d(
                [
                o, o, o, o,
                x, x, x, o,
                x, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (L, Deg180) => d(
                [
                x, x, o, o,
                o, x, o, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (L, Deg270) => d(
                [
                o, o, x, o,
                x, x, x, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (O, _) => d(
                [
                x, x, o, o,
                x, x, o, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (S, Deg0 | Deg180) => d(
                [
                o, x, x, o,
                x, x, o, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (S, Deg90 | Deg270) => d(
                [
                x, o, o, o,
                x, x, o, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (T, Deg0) => d(
                [
                o, x, o, o,
                x, x, x, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (T, Deg90) => d(
                [
                o, x, o, o,
                o, x, x, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (T, Deg180) => d(
                [
                o, o, o, o,
                x, x, x, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (T, Deg270) => d(
                [
                o, x, o, o,
                x, x, o, o,
                o, x, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (Z, Deg0 | Deg180) => d(
                [
                x, x, o, o,
                o, x, x, o,
                o, o, o, o,
                o, o, o, o
                ],
            ),
            #[rustfmt::skip]
            (Z, Deg90 | Deg270) => d(
                [
                o, x, o, o,
                x, x, o, o,
                x, o, o, o,
                o, o, o, o
                ],
            ),
        }
    }
}

pub struct Piece {
    pos: Point2<i32>,
    rotation: PieceRotation,
    kind: PieceKind,
}

impl Piece {
    pub fn collides_with(&self, grid: &Grid) -> bool {
        let piece_grid = self.kind.get_grid(self.rotation);
        grid.intersects(self.pos.x, self.pos.y, &piece_grid)
            || !grid.contains(self.pos.x, self.pos.y, &piece_grid)
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    color: Color,
}

pub struct Grid {
    blocks: Box<[Option<Block>]>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            blocks: vec![None; width * height].into_boxed_slice(),
        }
    }

    pub fn with_data(width: usize, height: usize, blocks: Box<[Option<Block>]>) -> Self {
        assert_eq!(width * height, blocks.len());
        Self {
            width,
            height,
            blocks,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> &Option<Block> {
        if self.contains_pos(x, y) {
            &self.blocks[x as usize + y as usize * self.width]
        } else {
            &None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: Option<Block>) {
        if self.contains_pos(x, y) {
            self.blocks[x as usize + y as usize * self.width] = value;
        } else {
            panic!()
        }
    }

    pub fn clear_line(&mut self, y: i32) {
        assert!(y >= 0 && y < self.height as i32);

        for iy in (1..=y as i32).rev() {
            for x in 0..self.width as i32 {
                self.set(x, iy, *self.at(x, iy - 1));
            }
        }
        for x in 0..self.width as i32 {
            self.set(x, 0, None);
        }
    }

    pub fn contains_pos(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    pub fn intersects(&self, x: i32, y: i32, other: &Grid) -> bool {
        for ix in 0..self.width as i32 {
            for iy in 0..self.height as i32 {
                if self
                    .at(ix, iy)
                    .is_some_and(|_| other.at(ix - x, iy - y).is_some())
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains(&self, x: i32, y: i32, other: &Grid) -> bool {
        for ix in 0..other.width as i32 {
            for iy in 0..other.height as i32 {
                if other.at(ix, iy).is_some() && !self.contains_pos(ix + x, iy + y) {
                    return false;
                }
            }
        }
        true
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn overlay(&mut self, x: i32, y: i32, other: Grid) {
        for ix in 0..self.width as i32 {
            for iy in 0..self.height as i32 {
                if let Some(block) = other.at(ix - x, iy - y) {
                    self.set(ix, iy, Some(*block));
                }
            }
        }
    }
}

#[derive(AsStd140)]
struct ShaderUniform {
    time: f32,
}

pub struct LineDestroyAnimation {
    lines_to_destroy: Vec<Range<u32>>,
    // 0.0 to 1.0
    progress: f32,
}

struct MainState {
    grid: Grid,
    grid_batch: InstanceArray,

    // TODO: Access ggez gfx ctx quad mesh
    quad_mesh: Mesh,

    time_last_moved_piece: Instant,

    rotate_sfx: audio::Source,
    place_sfx: audio::Source,
    clear_sfx: audio::Source,
    music: audio::Source,

    bg: graphics::Image,
    board_img: graphics::ScreenImage,
    bg_shader: graphics::Shader,
    bg_shader_params: graphics::ShaderParams<ShaderUniform>,

    piece_falling: Piece,

    line_destroy_animations: Option<LineDestroyAnimation>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let grid = Grid::new(10, 16);

        let grid_batch =
            InstanceArray::new(ctx, graphics::Image::from_path(ctx, "/textures/block.png")?);

        let bg_shader_params =
            graphics::ShaderParamsBuilder::new(&ShaderUniform { time: 0. }).build(ctx);

        let mut state = MainState {
            grid,
            grid_batch,
            rotate_sfx: audio::Source::new(ctx, "/sound/rotate.ogg")?,
            place_sfx: audio::Source::new(ctx, "/sound/place.ogg")?,
            clear_sfx: audio::Source::new(ctx, "/sound/clear.wav")?,
            music: audio::Source::new(ctx, "/music/game.mp3")?,
            bg: graphics::Image::from_path(ctx, "/textures/game_bg.png")?,
            bg_shader: graphics::ShaderBuilder::from_path("/shaders/game_bg.wgsl").build(ctx)?,
            bg_shader_params,
            board_img: graphics::ScreenImage::new(ctx, None, 10. / 400., 19. / 300., 1),
            quad_mesh: Mesh::from_data(
                &ctx.gfx,
                MeshData {
                    vertices: &[
                        Vertex {
                            position: [0., 0.],
                            uv: [0., 0.],
                            color: [1.; 4],
                        },
                        Vertex {
                            position: [1., 0.],
                            uv: [1., 0.],
                            color: [1.; 4],
                        },
                        Vertex {
                            position: [0., 1.],
                            uv: [0., 1.],
                            color: [1.; 4],
                        },
                        Vertex {
                            position: [1., 1.],
                            uv: [1., 1.],
                            color: [1.; 4],
                        },
                    ],
                    indices: &[0, 2, 1, 2, 3, 1],
                },
            ),
            piece_falling: Piece {
                pos: Point2 { x: 3, y: 0 },
                kind: PieceKind::J,
                rotation: PieceRotation::Deg90,
            },
            time_last_moved_piece: std::time::Instant::now(),
            line_destroy_animations: None,
        };

        state.music.play(ctx)?;
        state.music.set_volume(0.); // Comment to enable music
        state.update_grid_batch();

        Ok(state)
    }

    fn update_grid_batch(&mut self) {
        self.grid_batch.clear();
        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                if let Some(block) = self.grid.at(x as i32, y as i32) {
                    self.grid_batch.push(
                        DrawParam::new()
                            .dest(Point2 {
                                x: x as f32,
                                y: y as f32,
                            })
                            .color(block.color),
                    );
                } else if let Some(block) = self
                    .piece_falling
                    .kind
                    .get_grid(self.piece_falling.rotation)
                    .at(
                        x as i32 - self.piece_falling.pos.x,
                        y as i32 - self.piece_falling.pos.y,
                    )
                {
                    self.grid_batch.push(
                        DrawParam::new()
                            .dest(Point2 {
                                x: x as f32,
                                y: y as f32,
                            })
                            .color(block.color),
                    );
                }
            }
        }
    }

    fn place_current_piece(&mut self, ctx: &Context) {
        let piece_grid = self
            .piece_falling
            .kind
            .get_grid(self.piece_falling.rotation);

        self.grid.overlay(
            self.piece_falling.pos.x,
            self.piece_falling.pos.y,
            piece_grid,
        );
        self.piece_falling = Piece {
            pos: Point2 { x: 3, y: 0 },
            kind: PieceKind::random(&mut thread_rng()),
            rotation: PieceRotation::Deg0,
        };
        let _ = self.place_sfx.play(ctx);
        self.check_lines(ctx);
    }

    fn check_lines(&mut self, ctx: &Context) {
        let mut last_line_to_destroy = None;
        let mut lines_to_destroy = vec![];
        for y in 0..self.grid.height() as u32 {
            if (0..self.grid.width() as i32).all(|x| self.grid.at(x, y as i32).is_some()) {
                if last_line_to_destroy.is_none() {
                    last_line_to_destroy = Some(y);
                }
            } else if let Some(l) = last_line_to_destroy {
                last_line_to_destroy = None;
                lines_to_destroy.push(l..y);
            }
        }
        if let Some(l) = last_line_to_destroy {
            lines_to_destroy.push(l..self.grid.height() as u32);
        }
        if !lines_to_destroy.is_empty() {
            self.line_destroy_animations = Some(LineDestroyAnimation {
                lines_to_destroy: lines_to_destroy,
                progress: 0.,
            });
            let _ = self.clear_sfx.play(ctx);
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(anim) = &mut self.line_destroy_animations {
            anim.progress += ctx.time.delta().as_secs_f32() * 2.;
            if anim.progress >= 1. {
                for lines in &anim.lines_to_destroy {
                    for line in lines.clone() {
                        self.grid.clear_line(line as i32);
                    }
                }
                self.line_destroy_animations = None;
            }
        } else {
            let mut did_any_changes = false;

            if ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Left)
            {
                self.piece_falling.pos.x -= 1;
                if self.piece_falling.collides_with(&self.grid) {
                    self.piece_falling.pos.x += 1;
                } else {
                    did_any_changes = true;
                }
            }
            if ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Right)
            {
                self.piece_falling.pos.x += 1;
                if self.piece_falling.collides_with(&self.grid) {
                    self.piece_falling.pos.x -= 1;
                } else {
                    did_any_changes = true;
                }
            }
            if ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Up)
            {
                self.piece_falling.rotation = self.piece_falling.rotation.rotate_cw();
                if self.piece_falling.collides_with(&self.grid) {
                    self.piece_falling.rotation = self.piece_falling.rotation.rotate_ccw();
                } else {
                    let _ = self.rotate_sfx.play(ctx);
                    did_any_changes = true;
                }
            }
            let time_per_fall = if ctx
                .keyboard
                .is_key_pressed(ggez::winit::event::VirtualKeyCode::Down)
            {
                Duration::from_millis(100)
            } else {
                Duration::from_millis(500)
            };
            if ctx
                .keyboard
                .is_key_just_pressed(ggez::winit::event::VirtualKeyCode::Space)
            {
                self.time_last_moved_piece = std::time::Instant::now();
                while !self.piece_falling.collides_with(&self.grid) {
                    self.piece_falling.pos.y += 1;
                }
                self.piece_falling.pos.y -= 1;
                self.place_current_piece(ctx);
                did_any_changes = true;
            }
            if std::time::Instant::now() > self.time_last_moved_piece + time_per_fall {
                self.time_last_moved_piece = std::time::Instant::now();
                self.piece_falling.pos.y += 1;
                if self.piece_falling.collides_with(&self.grid) {
                    self.piece_falling.pos.y -= 1;
                    self.place_current_piece(ctx);
                }
                did_any_changes = true;
            }

            if did_any_changes {
                self.update_grid_batch();
            }
        }
        self.bg_shader_params.set_uniforms(
            ctx,
            &ShaderUniform {
                time: ctx.time.time_since_start().as_secs_f32() / 10.,
            },
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.set_shader(&self.bg_shader);
        canvas.set_shader_params(&self.bg_shader_params);
        canvas.draw(
            &Quad,
            DrawParam::new().dest_rect(Rect::new(0., 0., 400., 300.)),
        );
        canvas.set_default_shader();
        canvas.draw(&self.bg, DrawParam::new());

        canvas.draw_instanced_mesh(
            self.quad_mesh.clone(),
            &self.grid_batch,
            DrawParam::default().dest_rect(Rect::new(120., 16., 16., 16.)),
        );
        if let Some(anim) = &self.line_destroy_animations {
            for lines in &anim.lines_to_destroy {
                for line in lines.clone() {
                    canvas.draw(
                        &self.quad_mesh,
                        DrawParam::default().dest_rect(Rect::new(
                            120.,
                            16. + 16. * line as f32,
                            self.grid.width() as f32 * 16.,
                            16.,
                        )),
                    );
                }
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    let cb = ggez::ContextBuilder::new("tetris", "aleok")
        .window_setup(WindowSetup::default().title("Tetris"))
        .window_mode(WindowMode::default().dimensions(400., 300.))
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
