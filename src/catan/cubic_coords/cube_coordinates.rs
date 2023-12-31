use core::fmt;

use bevy::prelude::{ Vec2, Vec3 };

#[derive(Debug, Clone, Copy, Default)]
pub struct CubicCoord {
    pub q: i32, // W/E (left to right)
    pub r: i32, // NW/SE (up-left to down-right)
    pub s: i32, // NE/SW (up-right to down-left)
    pub ring: i32, // the ring of the grid the hex is on
}

// TODO try to offset it by PI / 6
// let u = |c: Vec3| (c.x + c.y / 2. - c.z / 2.0) * 2.;
// let v = |c: Vec3| ((c.y + c.z) * 3_f32.sqrt() / 2.) * 2.;
// let uv = |c: Vec3| Vec3::new(u(c), v(c), 0.0);

// old formulas
// let x = |v: Vec3| (v.x + v.y / 2.) * 2. * h;
// let y = |v: Vec3| (v.y * 3_f32.sqrt() / 2.) * 2. * h;
// let xy = |v: Vec3| Vec3::new(1.01 * x(v), 1.01 * y(v), 0.0);
// let mut hex_arr = vec![];

impl CubicCoord {
    pub fn new(q: i32, r: i32, s: i32) -> CubicCoord {
        let ring = (q.abs() + r.abs() + s.abs()) / 2;
        CubicCoord { q, r, s, ring }
    }
    pub fn _from_qrs(q: i32, r: i32, s: i32) -> CubicCoord {
        CubicCoord::new(q, r, s)
    }

    pub fn from_tuple((q, r, s): (i32, i32, i32)) -> CubicCoord {
        CubicCoord::new(q, r, s)
    }
    /// Rounds x,y,z into i32
    pub fn _from_vec(vec: Vec3) -> CubicCoord {
        let q: i32 = vec.x.round() as i32;
        let r: i32 = vec.y.round() as i32;
        let s: i32 = vec.z.round() as i32;
        CubicCoord::new(q, r, s)
    }
    /// Rounds arr[x,y,z] into i32
    pub fn _from_arr(arr: [f32; 3]) -> CubicCoord {
        let q: i32 = arr[0].round() as i32;
        let r: i32 = arr[1].round() as i32;
        let s: i32 = arr[2].round() as i32;
        CubicCoord::new(q, r, s)
    }

    pub fn to_cartesian_vec3(&self, h: f32) -> Vec3 {
        let q = self.s as f32;
        let r = self.q as f32;

        let x = CubicCoord::x((q, r, h));
        let y = CubicCoord::y((r, h));
        let z = 0.0;
        Vec3::new(x, y, z)
    }

    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.q, self.r, self.s).to_string()
    }

    pub fn get_neighbors(&self) -> [CubicCoord; 6] {
        [
            CubicCoord::new(self.q + 0, self.r - 1, self.s + 1),
            CubicCoord::new(self.q + 1, self.r - 1, self.s + 0),
            CubicCoord::new(self.q + 1, self.r + 0, self.s - 1),
            CubicCoord::new(self.q + 0, self.r + 1, self.s - 1),
            CubicCoord::new(self.q - 1, self.r + 1, self.s + 0),
            CubicCoord::new(self.q - 1, self.r + 0, self.s + 1),
        ]
    }

    // pub fn get_neighbors(&self) -> Vec<CubCoord> {
    //     vec![
    //         CubCoord::new(self.q + 0, self.r - 1, self.s + 1),
    //         CubCoord::new(self.q + 1, self.r - 1, self.s + 0),
    //         CubCoord::new(self.q + 1, self.r + 0, self.s - 1),
    //         CubCoord::new(self.q + 0, self.r + 1, self.s - 1),
    //         CubCoord::new(self.q - 1, self.r + 1, self.s + 0),
    //         CubCoord::new(self.q - 1, self.r + 0, self.s + 1),
    //     ]
    // }

    pub fn _to_cartesian_vec2(&self, h: f32) -> Vec2 {
        let q = self.s as f32;
        let r = self.q as f32;

        let x = CubicCoord::x((q, r, h));
        let y = CubicCoord::y((r, h));
        Vec2::new(x, y)
    }

    /// Computes the X cartesian component.
    fn x((q, r, h): (f32, f32, f32)) -> f32 {
        (q + r / 2.0) * h
    }

    /// Computes the Y cartesian component.
    fn y((r, h): (f32, f32)) -> f32 {
        let sqrt_3_div_2 = (3_f32).sqrt() / 2.0;
        r * sqrt_3_div_2 * h
    }
}

impl fmt::Display for CubicCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "q: {}, r: {}, s: {}, ring: {}", self.q, self.r, self.s, self.ring)
    }
}
