use crate::a2d::Graphics2D;
use crate::anyhow::Result;
use crate::futures::executor::block_on;
use crate::gilrs;
use crate::gilrs::Gilrs;
use crate::winit::{
    event::{ElementState, Event, KeyboardInput, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};
use crate::AppContext;
use crate::Axis;
use crate::DeviceId;
use crate::Game;
use crate::GamepadButton;
use crate::Key;
use crate::MouseButton;
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
        let mut scale_factor: f64 = 1.0;
        let mut mouse_pos: [f32; 2] = [0.0, 0.0];

        let (mut game, options) = {
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

            let options = game.options();

            if options.enable_gamepad {
                let proxy = event_loop.create_proxy();
                spawn_gilrs_listener_thread(proxy);
            }

            (game, options)
        };

        let Options {
            enable_gamepad: _,
            scroll_pixel_factor,
        } = options;

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
                    OtherEvent::Gilrs(gilrs::Event {
                        id,
                        event,
                        time: _time,
                    }) => {
                        let id: DeviceId = id.into();
                        match event {
                            gilrs::EventType::ButtonPressed(button, _) => {
                                let button = GamepadButton::from_gilrs(button).unwrap();
                                game.gamepad_button_pressed(&mut actx, id, button).unwrap();
                            }
                            gilrs::EventType::ButtonReleased(button, _) => {
                                let button = GamepadButton::from_gilrs(button).unwrap();
                                game.gamepad_button_released(&mut actx, id, button).unwrap();
                            }
                            gilrs::EventType::Connected => {
                                game.gamepad_connected(&mut actx, id).unwrap();
                            }
                            gilrs::EventType::Disconnected => {
                                game.gamepad_connected(&mut actx, id).unwrap();
                            }
                            gilrs::EventType::AxisChanged(axis, value, _) => {
                                let axis = Axis::from_gilrs(axis);
                                game.gamepad_axis_changed(&mut actx, id, axis, value)
                                    .unwrap();
                            }
                            gilrs::EventType::Dropped
                            | gilrs::EventType::ButtonChanged(..)
                            | gilrs::EventType::ButtonRepeated(..) => {}
                        }
                    }
                },
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => {
                        actx.exit();
                    }
                    WindowEvent::KeyboardInput {
                        input,
                        device_id: _,
                        is_synthetic: _,
                    } => match input {
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        } => {
                            if let Some(key) = Key::from_winit(*keycode) {
                                match state {
                                    ElementState::Pressed => {
                                        game.key_pressed(&mut actx, key).unwrap();
                                    }
                                    ElementState::Released => {
                                        game.key_released(&mut actx, key).unwrap();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => {
                        let position = position.to_logical(scale_factor);
                        mouse_pos = [position.x, position.y];
                        game.mouse_moved(&mut actx, mouse_pos).unwrap();
                    }
                    WindowEvent::MouseWheel {
                        device_id: _,
                        delta,
                        phase,
                        ..
                    } => match phase {
                        TouchPhase::Moved => {
                            let dxdy = match delta {
                                MouseScrollDelta::LineDelta(dx, dy) => [*dx, *dy],
                                MouseScrollDelta::PixelDelta(
                                    crate::winit::dpi::LogicalPosition { x, y },
                                ) => [
                                    *x as f32 * scroll_pixel_factor,
                                    *y as f32 * scroll_pixel_factor,
                                ],
                            };
                            game.scroll(&mut actx, mouse_pos, dxdy).unwrap();
                        }
                        _ => {}
                    },
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        ..
                    } => {
                        let button = MouseButton::from_winit(*button);
                        match state {
                            ElementState::Pressed => {
                                game.mouse_button_pressed(&mut actx, mouse_pos, button)
                                    .unwrap();
                            }
                            ElementState::Released => {
                                game.mouse_button_released(&mut actx, mouse_pos, button)
                                    .unwrap();
                            }
                        }
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        game.ch(&mut actx, *ch).unwrap();
                    }
                    WindowEvent::Resized(physical_size) => {
                        on_resize(&mut actx, &mut game, scale_factor, *physical_size).unwrap();
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor: new_scale_factor,
                        new_inner_size: physical_size,
                    } => {
                        scale_factor = *new_scale_factor;
                        on_resize(&mut actx, &mut game, scale_factor, **physical_size).unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

fn on_resize<G: Game>(
    actx: &mut AppContext,
    game: &mut G,
    scale_factor: f64,
    physical_size: crate::winit::dpi::PhysicalSize<u32>,
) -> Result<()> {
    let logical_size = physical_size.to_logical(scale_factor);
    let (width, height) = (logical_size.width, logical_size.height);
    actx.graphics.resized(physical_size);
    actx.graphics.set_scale([width as f32, height as f32]);
    game.resize(actx, width, height)
}

fn spawn_gilrs_listener_thread(proxy: EventLoopProxy<OtherEvent>) {
    std::thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();
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
