use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render::render,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();

        render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut current_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => {
                        player.move_left();
                        audio.play("move");
                    }
                    KeyCode::Right => {
                        player.move_right();
                        audio.play("move");
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        player.update(delta);

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        if invaders.update(delta) {
            audio.play("move");
        }

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }

        let _ = render_tx.send(current_frame);
        thread::sleep(Duration::from_millis(1));

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
