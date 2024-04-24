pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger
{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("you are more than 100%")
        } else if percentage_of_max >= 0.8 {
            self.messenger.send("you are more than 80%")
        } else {
            self.messenger.send("you are less than 80%")
        }
    }
}