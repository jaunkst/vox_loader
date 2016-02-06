#[macro_use]
extern crate byteorder;
mod vox_loader;
pub use vox_loader::VoxLoader;
pub use vox_loader::Voxel;

#[test]
fn it_works() {
    // use vox_loader::VoxLoader;
    // VoxLoader::new("../vox_files/3x3x3.vox");
}
