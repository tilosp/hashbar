pub use digest::Update as Hasher;

pub trait Hashbar {
    fn hash<H: Hasher>(&self, hasher: &mut H);
}
