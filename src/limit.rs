#[derive(Debug)]
pub enum Limit {
    KiB(f32),
    MiB(f32),
    GiB(f32),
}
