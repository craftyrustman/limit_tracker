use std::cell::RefCell;

use limit_tracker::{LimitTracker, Messenger};

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(60);
    limit_tracker.set_value(90);
    limit_tracker.set_value(110);

    for msg in mock_messenger.sent_messages.borrow().iter() {
        println!("{}", msg)
    }
}


struct MockMessenger {
    sent_messages: RefCell<Vec<String>>
}

impl MockMessenger {
    fn new() -> Self {
        Self { sent_messages: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str){
        self.sent_messages.borrow_mut().push(String::from(message));
    }   
}