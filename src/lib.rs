extern crate winit;
extern crate wgpu;
extern crate futures;
extern crate shaderc;

mod tut;

pub use tut::main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
