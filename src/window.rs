use crate::a2d::Graphics2D;
use crate::anyhow::Result;
use crate::futures::executor::block_on;
use crate::gilrs;
use crate::gilrs::Gilrs;
use crate::winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};
use crate::AppContext;
use crate::DeviceId;
use crate::Game;
use crate::Key;
use crate::Options;
use crate::RenderContext;

pub fn run<G: Game, F: FnOnce(&mut AppContext) -> Result<G>>(f: F) -> ! {
    let window = block_on(Window::new()).unwrap();
    window.run(f)
}

pub struct Window {
    event_loop: EventLoop<OtherEvent>,
    window: crate::winit::window::Window,
    graphics: Graphics2D,
}

impl Window {
    pub async fn new() -> Result<Self> {
        let event_loop = EventLoop::with_user_event();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let graphics = Graphics2D::from_winit_window(&window).await.unwrap();
        Ok(Self {
            event_loop,
            window,
            graphics,
        })
    }

    pub fn run<G: Game, F: FnOnce(&mut AppContext) -> Result<G>>(self, f: F) -> ! {
        let event_loop = self.event_loop;
        let window = self.window;
        let mut graphics = self.graphics;

        let mut game = {
            let mut actx = AppContext {
                graphics: &mut graphics,
                control_flow: &mut ControlFlow::default(),
            };

            let size = window.inner_size();
            actx.set_scale([size.width as f32, size.height as f32]);

            let game = f(&mut actx).unwrap();

            if *actx.control_flow != ControlFlow::default() {
                panic!(
                    "Tried to modify control flow on init: {:?}",
                    actx.control_flow
                );
            }

            {
                let Options {
                    enable_gamepad,
                } = game.options();

                if enable_gamepad {
                    let proxy = event_loop.create_proxy();
                    spawn_gilrs_listener_thread(proxy);
                }
            }

            game
        };

        event_loop.run(move |event, _, control_flow| {
            let mut actx = AppContext {
                graphics: &mut graphics,
                control_flow,
            };
            match event {
                Event::RedrawRequested(_window_id) => {
                    let mut rctx = RenderContext { actx: &mut actx };
                    game.render(&mut rctx).unwrap();
                }
                Event::MainEventsCleared => {
                    game.update(&mut actx).unwrap();
                    window.request_redraw();
                }
                Event::UserEvent(other) => match other {
                    OtherEvent::Gilrs(event) => {
                        println!("gilrs event -> {:?}", event);
                    }
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => {
                        actx.exit();
                    }
                    WindowEvent::KeyboardInput {
                        input, device_id, ..
                    } => match input {
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let Some(key) = Key::from_winit(*keycode) {
                                let dev = (*device_id).into();
                                match state {
                                    ElementState::Pressed => {
                                        game.key_pressed(&mut actx, dev, key).unwrap();
                                    }
                                    ElementState::Released => {
                                        game.key_released(&mut actx, dev, key).unwrap();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    WindowEvent::ReceivedCharacter(ch) => {
                        game.ch(&mut actx, *ch).unwrap();
                    }
                    WindowEvent::Resized(physical_size) => {
                        let (width, height) = (physical_size.width, physical_size.height);
                        actx.graphics.resized(*physical_size);
                        actx.graphics.set_scale([width as f32, height as f32]);
                        game.resize(&mut actx, width, height).unwrap();
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let (width, height) = (new_inner_size.width, new_inner_size.height);
                        actx.graphics.resized(**new_inner_size);
                        actx.graphics.set_scale([width as f32, height as f32]);
                        game.resize(&mut actx, width, height).unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

fn spawn_gilrs_listener_thread(proxy: EventLoopProxy<OtherEvent>) {
    let mut gilrs = Gilrs::new().unwrap();
    std::thread::spawn(move || {
        loop {
            while let Some(event) = gilrs.next_event() {
                proxy.send_event(OtherEvent::Gilrs(event)).unwrap();
            }
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / 45.0));
        }
    });
}

#[derive(Debug, Clone)]
enum OtherEvent {
    Gilrs(gilrs::Event),
}
