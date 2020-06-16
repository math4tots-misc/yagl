extern crate winit;
extern crate wgpu;
extern crate futures;
extern crate shaderc;
extern crate bytemuck;
extern crate image;
extern crate failure;

mod tut;

pub use tut::main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
