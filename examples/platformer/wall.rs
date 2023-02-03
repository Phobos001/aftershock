
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Wall {
    pub flags: u16,
    pub solid: bool,
    pub aabb: AABB,
}