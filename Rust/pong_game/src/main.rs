use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::nalgebra as na;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 20.0;
const MIDDLE_LINE_W: f32 = 2.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT*0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH*0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 500.0;

fn clamp(value: &mut f32, low: f32, high: f32){
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h =  graphics::drawable_size(ctx).1;
    let dt =        ggez::timer::delta(ctx).as_secs_f32();
    
    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y -= y_dir * PLAYER_SPEED * dt;
    }

    clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h-RACKET_HEIGHT_HALF);

}

fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32){
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5){
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5){
        true => y,
        false => -y,
    };
}

fn set_score(ctx: &mut Context, ball_pos: &mut na::Point2<f32>, mut ball_vel: &mut na::Vector2<f32>, player_score: &mut i32, is_player_1: bool){
    let (screen_w, screen_h) =  graphics::drawable_size(ctx);
    if is_player_1 { 
        if ball_pos.x < 0.0 {
            ball_pos.x = screen_w * 0.5;
            ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);
            *player_score += 1;
        }
    } else { 
        if ball_pos.x > screen_w {
            ball_pos.x = screen_w * 0.5;
            ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);
            *player_score += 1;
        }
    }
}

fn handle_ball_on_y_borders(ctx: &mut Context, ball_pos: &mut na::Point2<f32>, ball_vel: &mut na::Vector2<f32>){
    let (_screen_w, screen_h) = graphics::drawable_size(ctx);

    if ball_pos.y < BALL_SIZE_HALF {
        ball_pos.y = BALL_SIZE_HALF;
        ball_vel.y = ball_vel.y.abs();
    } else if ball_pos.y > screen_h - BALL_SIZE_HALF {
        ball_pos.y = screen_h - BALL_SIZE_HALF;
        ball_vel.y = -ball_vel.y.abs();
    }
}

fn intersect_player(player_pos: & na::Point2<f32>, ball_pos: & na::Point2<f32>, ball_vel: &mut na::Vector2<f32>){
    let intersect_player = 
        ball_pos.x - BALL_SIZE_HALF < player_pos.x + RACKET_WIDTH_HALF
    &&  ball_pos.x + BALL_SIZE_HALF > player_pos.x - RACKET_WIDTH_HALF
    &&  ball_pos.y - BALL_SIZE_HALF < player_pos.y + RACKET_HEIGHT_HALF
    &&  ball_pos.y + BALL_SIZE_HALF > player_pos.y - RACKET_HEIGHT_HALF;

    if intersect_player {
        ball_vel.x = ball_vel.x * -1_f32;
    }
}

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
       
        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState{
            player_1_pos:   na::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            player_2_pos:   na::Point2::new(screen_w-RACKET_WIDTH_HALF - PADDING, screen_h_half),
            ball_pos:       na::Point2::new(screen_w_half, screen_h_half),
            ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        
        move_racket(&mut self.player_1_pos, KeyCode::W, 1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, -1.0, ctx);

        set_score(ctx, &mut self.ball_pos, &mut self.ball_vel, &mut self.player_1_score, true);
        set_score(ctx, &mut self.ball_pos, &mut self.ball_vel, &mut self.player_2_score, false);
                
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.ball_pos += self.ball_vel * dt;
        
        handle_ball_on_y_borders(ctx, &mut self.ball_pos, &mut self.ball_vel);
        
        intersect_player(&self.player_1_pos, &self.ball_pos, &mut self.ball_vel);
        intersect_player(&self.player_2_pos, &self.ball_pos, &mut self.ball_vel);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE)?;
        
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, graphics::WHITE)?;

        let screen_h = graphics::drawable_size(ctx).1;
        let middle_rect = graphics::Rect::new(-MIDDLE_LINE_W*0.5, 0.0, MIDDLE_LINE_W, screen_h);
        let middle_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), middle_rect, graphics::WHITE)?;
        
        let mut draw_param = graphics::DrawParam::default();
        
        let screen_middle_x = graphics::drawable_size(ctx).0 * 0.5;
        draw_param.dest = [screen_middle_x, 0.0].into();
        graphics::draw(ctx, &middle_mesh, draw_param)?;

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text    = graphics::Text::new(format!("{}             {}", self.player_1_score, self.player_2_score));
        let screen_w      = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;

        let mut score_pos = na::Point2::new(screen_w_half, 40.0);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Gavila");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");
    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}