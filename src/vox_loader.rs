
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

pub struct Voxel {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub c: u8
}
impl fmt::Debug for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Voxel {{ x: {}, y: {} z: {}, c: {} }}", self.x, self.y, self.z, self.c)
    }
}

pub struct Size {
    pub x: u32,
    pub y: u32,
    pub z: u32
}
impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Size {{ x: {}, y: {} z: {}}}", self.x, self.y, self.z)
    }
}

pub struct VoxLoader {
    filepath: &'static str,
    data: Vec<u8>,
    offset: usize,
    pub size: Size,
    pub voxels: Vec<Vec<Voxel>>,
    pub palette: Vec<u32>,
}
impl fmt::Debug for VoxLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VoxLoader {{ filepath: {:?}, data: {:?}, offset: {:?}, size: {:?}, voxels: {:?}, palette: {:?} }}", self.filepath, self.data, self.offset, self.size, self.voxels, self.palette)
    }
}

struct Chunk {
    id: String,
    length: u32,
    child_length: u32,
    childs: Vec<Chunk>,
}

impl VoxLoader {
    pub fn new(fp: &'static str) -> VoxLoader {
        let mut vl = VoxLoader {
            filepath: fp,
            data: Vec::new(),
            offset: 0,
            size: Size { x: 0, y: 0, z: 0 },
            voxels: Vec::new(),
            palette: Vec::new(),
        };
        vl.load();
        if vl.palette.len() != 256 {
            vl.palette = vec![0x0, 0xffffffff, 0xffffccff, 0xffff99ff, 0xffff66ff, 0xffff33ff, 0xffff00ff, 0xffccffff, 0xffccccff, 0xffcc99ff, 0xffcc66ff, 0xffcc33ff, 0xffcc00ff, 0xff99ffff, 0xff99ccff, 0xff9999ff, 0xff9966ff, 0xff9933ff, 0xff9900ff, 0xff66ffff, 0xff66ccff, 0xff6699ff, 0xff6666ff, 0xff6633ff, 0xff6600ff, 0xff33ffff, 0xff33ccff, 0xff3399ff, 0xff3366ff, 0xff3333ff, 0xff3300ff, 0xff00ffff, 0xff00ccff, 0xff0099ff, 0xff0066ff, 0xff0033ff, 0xff0000ff, 0xccffffff, 0xccffccff, 0xccff99ff, 0xccff66ff, 0xccff33ff, 0xccff00ff, 0xccccffff, 0xccccccff, 0xcccc99ff, 0xcccc66ff, 0xcccc33ff, 0xcccc00ff, 0xcc99ffff, 0xcc99ccff, 0xcc9999ff, 0xcc9966ff, 0xcc9933ff, 0xcc9900ff, 0xcc66ffff, 0xcc66ccff, 0xcc6699ff, 0xcc6666ff, 0xcc6633ff, 0xcc6600ff, 0xcc33ffff, 0xcc33ccff, 0xcc3399ff, 0xcc3366ff, 0xcc3333ff, 0xcc3300ff, 0xcc00ffff, 0xcc00ccff, 0xcc0099ff, 0xcc0066ff, 0xcc0033ff, 0xcc0000ff, 0x99ffffff, 0x99ffccff, 0x99ff99ff, 0x99ff66ff, 0x99ff33ff, 0x99ff00ff, 0x99ccffff, 0x99ccccff, 0x99cc99ff, 0x99cc66ff, 0x99cc33ff, 0x99cc00ff, 0x9999ffff, 0x9999ccff, 0x999999ff, 0x999966ff, 0x999933ff, 0x999900ff, 0x9966ffff, 0x9966ccff, 0x996699ff, 0x996666ff, 0x996633ff, 0x996600ff, 0x9933ffff, 0x9933ccff, 0x993399ff, 0x993366ff, 0x993333ff, 0x993300ff, 0x9900ffff, 0x9900ccff, 0x990099ff, 0x990066ff, 0x990033ff, 0x990000ff, 0x66ffffff, 0x66ffccff, 0x66ff99ff, 0x66ff66ff, 0x66ff33ff, 0x66ff00ff, 0x66ccffff, 0x66ccccff, 0x66cc99ff, 0x66cc66ff, 0x66cc33ff, 0x66cc00ff, 0x6699ffff, 0x6699ccff, 0x669999ff, 0x669966ff, 0x669933ff, 0x669900ff, 0x6666ffff, 0x6666ccff, 0x666699ff, 0x666666ff, 0x666633ff, 0x666600ff, 0x6633ffff, 0x6633ccff, 0x663399ff, 0x663366ff, 0x663333ff, 0x663300ff, 0x6600ffff, 0x6600ccff, 0x660099ff, 0x660066ff, 0x660033ff, 0x660000ff, 0x33ffffff, 0x33ffccff, 0x33ff99ff, 0x33ff66ff, 0x33ff33ff, 0x33ff00ff, 0x33ccffff, 0x33ccccff, 0x33cc99ff, 0x33cc66ff, 0x33cc33ff, 0x33cc00ff, 0x3399ffff, 0x3399ccff, 0x339999ff, 0x339966ff, 0x339933ff, 0x339900ff, 0x3366ffff, 0x3366ccff, 0x336699ff, 0x336666ff, 0x336633ff, 0x336600ff, 0x3333ffff, 0x3333ccff, 0x333399ff, 0x333366ff, 0x333333ff, 0x333300ff, 0x3300ffff, 0x3300ccff, 0x330099ff, 0x330066ff, 0x330033ff, 0x330000ff, 0xffffff, 0xffccff, 0xff99ff, 0xff66ff, 0xff33ff, 0xff00ff, 0xccffff, 0xccccff, 0xcc99ff, 0xcc66ff, 0xcc33ff, 0xcc00ff, 0x99ffff, 0x99ccff, 0x9999ff, 0x9966ff, 0x9933ff, 0x9900ff, 0x66ffff, 0x66ccff, 0x6699ff, 0x6666ff, 0x6633ff, 0x6600ff, 0x33ffff, 0x33ccff, 0x3399ff, 0x3366ff, 0x3333ff, 0x3300ff, 0xffff, 0xccff, 0x99ff, 0x66ff, 0x33ff, 0xee0000ff, 0xdd0000ff, 0xbb0000ff, 0xaa0000ff, 0x880000ff, 0x770000ff, 0x550000ff, 0x440000ff, 0x220000ff, 0x110000ff, 0xee00ff, 0xdd00ff, 0xbb00ff, 0xaa00ff, 0x8800ff, 0x7700ff, 0x5500ff, 0x4400ff, 0x2200ff, 0x1100ff, 0xeeff, 0xddff, 0xbbff, 0xaaff, 0x88ff, 0x77ff, 0x55ff, 0x44ff, 0x22ff, 0x11ff, 0xeeeeeeff, 0xddddddff, 0xbbbbbbff, 0xaaaaaaff, 0x888888ff, 0x777777ff, 0x555555ff, 0x444444ff, 0x222222ff, 0x111111ff];
        }

        return vl;
    }

    fn read_string(&mut self) -> String {
        let mut char_vector: Vec<char> = Vec::new();
        for _ in 0..4 {
            char_vector.push(self.data[self.offset] as char);
            self.offset += 1;
        }
        return char_vector.iter().cloned().collect::<String>();
    }

    fn read_byte(&mut self) -> u8 {
        let result: u8 = self.data[self.offset];
        self.offset += 1;
        return result;
    }

    fn read_int(&mut self, big_endian: bool) -> u32 {
        let mut u8_vector: Vec<u8> = Vec::new();
        for _ in 0..4 {
            u8_vector.push(self.data[self.offset]);
            self.offset += 1;
        }
        let mut buf = Cursor::new(u8_vector);
        if big_endian {
            return buf.read_u32::<BigEndian>().unwrap();
        } else {
            return buf.read_u32::<LittleEndian>().unwrap();
        }
    }

    fn read_chunk(&mut self) -> Chunk {
        let mut chunk = Chunk {
            id: self.read_string(),
            length: self.read_int(false),
            child_length: self.read_int(false),
            childs: vec![],
        };

        if chunk.id == "MAIN" && chunk.child_length > 0 {
            let mut child_bytes_remaining = chunk.child_length;
            while child_bytes_remaining > 0 {
                let child_chunk = self.read_chunk();
                child_bytes_remaining -= child_chunk.length + 12;
                chunk.childs.push(child_chunk);
            }
        } else if chunk.id == "SIZE" {
            self.size.x = self.read_int(false);
            self.size.y = self.read_int(false);
            self.size.z = self.read_int(false);
        } else if chunk.id == "XYZI" {
            let num_voxels = self.read_int(false);
            let mut voxels: Vec<Voxel> = Vec::new();

            for _ in 0..num_voxels {
                let voxel: Voxel = Voxel {
                    x: self.read_byte(),
                    y: self.read_byte(),
                    z: self.read_byte(),
                    c: self.read_byte(),
                };
                voxels.push(voxel);
            }
            self.voxels.push(voxels)

        } else if chunk.id == "RGBA" {
            for _ in 0..256 {
                let color: u32 = self.read_int(true);
                self.palette.push(color);
            }
        } else {
            println!("unsupported chunk type {}", chunk.id);
        }

        return chunk;
    }

    fn load(&mut self) {
        let path = Path::new(self.filepath);
        let display = path.display();
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        file.read_to_end(&mut self.data).unwrap();
        self.read_string();
        self.read_int(false);
        self.read_chunk();
    }

}
