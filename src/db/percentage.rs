#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Percentage(u32);

impl Percentage {
    pub fn new(percentage: u32) -> Option<Self> {
        if percentage > 100 {
            None
        } else {
            Some(Percentage(percentage))
        }
    }
}

impl<T> From<T> for Percentage
where
    T: Into<u32>,
{
    fn from(p: T) -> Self {
        Percentage::new(p.into()).unwrap()
    }
}
