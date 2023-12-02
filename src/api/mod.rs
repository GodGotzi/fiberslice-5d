pub mod math;

pub trait FlipYZ {
    fn flip(&mut self);
}

pub trait Reverse {
    fn reverse(&mut self);
}

pub trait Contains<P> {
    fn contains(&self, point: &P) -> bool;
}