//! Following various tutorials:
//! https://rust-tutorials.github.io/learn-gfx-hal/
//! https://falseidolfactory.com/2018/08/16/gfx-hal-part-0-drawing-a-triangle.html
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod hal;
use crate::anyhow::{
    Result,
    Context,
};
use crate::winit::{
    event::{
        Event,
        WindowEvent,
        KeyboardInput,
        ElementState,
        VirtualKeyCode,
    },
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    window::{
        WindowBuilder,
    },
};
use hal::Hal;

pub unsafe fn tutorial_gfxhal_main() -> Result<()> {
    simple_logger::init().context("Failed to init simple_logger")?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let hal = Hal::new(&window)?;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        // state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
