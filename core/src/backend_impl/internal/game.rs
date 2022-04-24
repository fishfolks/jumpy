use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::time::{Duration, Instant};

use glutin::event::{StartCause, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use hecs::World;

use crate::math::Vec2;

use crate::audio::{apply_audio_config, stop_music};

use crate::color::{colors, Color};
use crate::event::{Event, EventHandler};
use crate::gl::create_gl_context;
use crate::input::{
    apply_input_config, is_key_pressed, is_key_released, mouse_movement, mouse_position,
    update_gamepad_context, KeyCode,
};
use crate::math::Size;
use crate::physics::{fixed_delta_time, physics_world};
use crate::prelude::{input_event_handler, DefaultEventHandler};
use crate::rendering::{clear_screen, end_frame};
use crate::window::{
    apply_window_config, create_window, get_window, WindowMode, DEFAULT_WINDOW_TITLE,
};
use crate::{Config, Result};

use crate::state::{GameState, GameStateBuilderFn};

pub struct Game<E: 'static + Debug> {
    config: Config,
    state: Rc<RefCell<dyn GameState>>,
    event_loop: Option<EventLoop<Event<E>>>,
    event_handler: Option<Box<dyn EventHandler<E>>>,
    clear_color: Option<Color>,
    fixed_draw_delta_time: Option<Duration>,
    last_update: Instant,
    last_draw: Instant,
    fixed_update_accumulator: f32,
}

impl<E: 'static + Debug> Game<E> {
    pub fn new<S: 'static + GameState>(state: S) -> Self {
        Game {
            config: Config::default(),
            state: Rc::new(RefCell::new(state)),
            event_loop: None,
            event_handler: None,
            clear_color: None,
            fixed_draw_delta_time: None,
            last_update: Instant::now(),
            last_draw: Instant::now(),
            fixed_update_accumulator: 0.0,
        }
    }

    pub fn with_config(self, config: &Config) -> Self {
        Game {
            config: config.clone(),
            ..self
        }
    }

    pub fn with_event_loop(self, event_loop: EventLoop<Event<E>>) -> Self {
        Game {
            event_loop: Some(event_loop),
            ..self
        }
    }

    pub fn with_event_handler<H: 'static + EventHandler<E>>(self, event_handler: H) -> Self {
        Game {
            event_handler: Some(Box::new(event_handler)),
            ..self
        }
    }

    pub fn with_clear_color(self, color: Color) -> Self {
        Game {
            clear_color: Some(color),
            ..self
        }
    }

    pub fn change_state(&mut self, state: Rc<RefCell<dyn GameState>>) -> Result<()> {
        stop_music();

        physics_world().clear();

        let world = self.get_state().end()?;

        self.state = state;

        self.get_state().begin(world)?;

        Ok(())
    }

    fn apply_current_config(&mut self) {
        self.fixed_draw_delta_time = config
            .video
            .max_fps
            .map(|max_fps| Duration::from_secs_f32(1.0 / max_fps as f32));

        apply_window_config(&config.window);

        apply_audio_config(&config.audio);

        apply_input_config(&config.input);
    }

    fn apply_config(&mut self, config: &Config) {
        self.config = config.clone();

        self.apply_current_config();
    }

    pub fn try_get_state(&mut self) -> Option<&mut (dyn GameState + 'static)> {
        Rc::get_mut(&mut self.state).map(|rc| rc.get_mut())
    }

    pub fn get_state(&mut self) -> &mut (dyn GameState + 'static) {
        self.try_get_state().unwrap()
    }

    pub async fn run(self) -> Result<()> {
        let mut game = self;

        game.apply_current_config();

        let event_loop = game
            .event_loop
            .take()
            .unwrap_or_else(|| EventLoop::<Event<E>>::with_user_event());

        let mut event_handler = game
            .event_handler
            .take()
            .unwrap_or_else(|| Box::new(DefaultEventHandler));

        event_loop.run(move |event, _, control_flow| {
            event_handler.handle(&event, control_flow);

            match &event {
                glutin::event::Event::NewEvents(cause) => {
                    match cause {
                        StartCause::Init => {
                            game.get_state().begin(None);
                        }
                        _ => {}
                    }

                    update_gamepad_context()
                        .unwrap_or_else(|err| panic!("Error in gamepad context update: {}", err));
                }
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                glutin::event::Event::RedrawRequested(..) => {}
                glutin::event::Event::UserEvent(event) => match event {
                    Event::Custom(event) => event_handler.handle_custom(event, control_flow),
                    Event::ConfigChanged(config) => game.apply_config(config),
                    Event::StateTransition(state) => game
                        .change_state(state.clone())
                        .unwrap_or_else(|err| panic!("Error when changing state: {}", err)),
                    Event::Quit => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }

            if input_event_handler(&event) {
                if *control_flow == ControlFlow::Exit {
                    stop_music();

                    game.get_state().end();

                    return;
                } else {
                    let now = Instant::now();

                    let delta_time = now.duration_since(game.last_update);

                    game.get_state()
                        .update(delta_time.as_secs_f32())
                        .unwrap_or_else(|err| panic!("Error in game state update: {}", err));

                    game.last_update = now;

                    game.fixed_update_accumulator += delta_time.as_secs_f32();

                    let fixed_delta_time = fixed_delta_time().as_secs_f32();

                    while game.fixed_update_accumulator >= fixed_delta_time {
                        game.fixed_update_accumulator -= fixed_delta_time;

                        let integration_factor =
                            if game.fixed_update_accumulator >= fixed_delta_time {
                                1.0
                            } else {
                                game.fixed_update_accumulator / fixed_delta_time
                            };

                        game.get_state()
                            .fixed_update(fixed_delta_time, integration_factor)
                            .unwrap_or_else(|err| {
                                panic!("Error in game state fixed update: {}", err)
                            });
                    }

                    {
                        let fixed_draw_delta_time =
                            game.fixed_draw_delta_time.unwrap_or(Duration::ZERO);

                        let draw_delta_time = now.duration_since(game.last_draw);

                        if draw_delta_time >= fixed_draw_delta_time {
                            clear_screen(self.clear_color.into());

                            game.get_state()
                                .draw(draw_delta_time.as_secs_f32())
                                .unwrap_or_else(|err| {
                                    panic!("Error in game state fixed draw: {}", err)
                                });

                            end_frame();

                            game.last_draw = now;
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
