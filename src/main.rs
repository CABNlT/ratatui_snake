use rand::Rng;
use std::{
    array,
    fmt::Display,
    io::stdout,
    ptr::null,
    string,
    time::{Duration, Instant},
    u8,
};

use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},
};
use itertools::Itertools;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::{Buffer, Cell},
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Stylize},
    symbols::{Marker, border},
    text::Text,
    widgets::{
        Block, Borders, Widget,
        canvas::{Canvas, Circle, Line, Map, MapResolution, Points, Rectangle},
    },
};

fn main() -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let terminal: ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>> =
        ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    stdout().execute(DisableMouseCapture)?;

    app_result
}

pub struct App {
    exit: bool,
    tick_count: i32,
    apple_xpos: u16,
    apple_ypos: u16,
    snake_xpos: f64,
    apple_xf: f64,
    apple_yf: f64,
    snake_ypos: f64,
    snake_dir: u8,
    sqr_x: f64,
    sqr_y: f64,
    area: Rect,
    score: i32,
    snake_lasposx: f64,
    snake_lasposy: f64,
    forup: f64,
    other_posx: [f64; 10000],
    other_posy: [f64; 10000],
    fixme: i32,
}

impl App {
    const fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
            apple_xpos: 0,
            apple_ypos: 0,
            apple_xf: 1.0,
            apple_yf: 1.0,
            snake_xpos: 1.0,
            snake_ypos: 1.0,
            snake_dir: 0,
            sqr_x: 0.0,
            sqr_y: 0.0,
            area: Rect::new(10, 10, 200, 100),
            score: 0,
            snake_lasposx: 1.0,
            snake_lasposy: 1.0,
            forup: 0.0,
            other_posx: [0.0; 10000],
            other_posy: [0.0; 10000],
            fixme: 0,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        self.gen_apple();
        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if !event::poll(timeout)? {
                self.on_tick();

                last_tick = Instant::now();
                continue;
            }

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.key_input(key),
                    _ => (),
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();

                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn key_input(&mut self, key: event::KeyEvent) {
        //  if key.kind != crossterm::event::KeyEventKind::Press {
        //     return;
        //  }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up => self.Snake_Up(),
            KeyCode::Down => self.Snake_Down(),
            KeyCode::Left => self.Snake_Left(),
            KeyCode::Right => self.Snake_Right(),
            KeyCode::Char('w') => self.score += 1,
            _ => {}
        }
    }
    fn Snake_Up(&mut self) {
        self.snake_dir = 1;
    }

    fn Snake_Down(&mut self) {
        self.snake_dir = 2;
    }

    fn Snake_Left(&mut self) {
        self.snake_dir = 3;
    }
    fn Snake_Right(&mut self) {
        self.snake_dir = 4;
    }

    fn gen_apple(&mut self) {
        self.apple_ypos = rand::rng().random_range(1..=self.area.height);
        self.apple_xpos = rand::rng().random_range(1..=self.area.width);

        self.apple_xf = self.apple_xpos.into();
        self.apple_yf = self.apple_ypos.into();
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;

        if self.snake_xpos == 0.0 {
            self.exit = true;
        }
        if self.snake_ypos == 0.0 {
            self.exit = true;
        }

        if [self.snake_xpos, self.snake_ypos] == [self.apple_xf, self.apple_yf] {
            self.score += 1;
            self.gen_apple();
        }

        match self.snake_dir {
            0 => self.snake_xpos += 0.0,
            1 => self.snake_ypos += 1.0,
            2 => self.snake_ypos -= 1.0,
            3 => self.snake_xpos -= 1.0,
            4 => self.snake_xpos += 1.0,
            _ => todo!(),
        }


    }
    

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self.blocky(), frame.area());
      
    }









    pub fn blocky(&mut self) -> impl Widget {
        let left = 0.0;
        let right = f64::from(self.area.width);
        let bottom = 0.0;
        let top = f64::from(self.area.height);

        for scorey in 0..self.score {
            
            if scorey == 0 {
                 self.fixme = 1;
            } else if scorey > 0 {self.fixme = 0;}
            
           // if (self.tick_count % (scorey + self.fixme)) == 0 {
            match self.snake_dir {
                0 => self.other_posx[scorey as usize] = self.snake_xpos + 0.0,
                1 => self.other_posy[scorey as usize] = self.snake_ypos - f64::from(scorey + self.fixme), //up
                2 => self.other_posy[scorey as usize] = self.snake_ypos + f64::from(scorey + self.fixme), //down
                3 => self.other_posx[scorey as usize] = self.snake_xpos + f64::from(scorey + self.fixme), //left
                4 => self.other_posx[scorey as usize] = self.snake_xpos - f64::from(scorey + self.fixme), //right
                _ => todo!(),
            };
            match self.snake_dir {
                0 => self.other_posx[scorey as usize] = self.snake_xpos + 0.0,
                1 => self.other_posx[scorey as usize] = self.snake_xpos, //up
                2 => self.other_posx[scorey as usize] = self.snake_xpos, //down
                3 => self.other_posy[scorey as usize] = self.snake_ypos, //left
                4 => self.other_posy[scorey as usize] = self.snake_ypos, //right
                _ => todo!(),
            };
        }
      //  }
        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(border::THICK),
            )
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .marker(Marker::HalfBlock)
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: self.apple_xf,
                    y: self.apple_yf,
                    width: 1.0,
                    height: 1.0,
                    color: Color::Red,
                });
                ctx.draw(&Rectangle {
                    x: self.snake_xpos,
                    y: self.snake_ypos,
                    width: 1.0,
                    height: 1.0,
                    color: Color::White,
                });
                for scorey in 0..self.score{
                    ctx.draw(&Rectangle {
                        x: self.other_posx[scorey as usize],
                        y: self.other_posy[scorey as usize],
                        width: 1.0,
                        height: 1.0,
                        color: Color::Red,
                    });
                }
            
                     
                
            })
        
            
        
  
    }
}
