pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{RefCell};
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // doesn't match the trait send method signature
        // fn send(&mut self, message: &str) {
        //     self.sent_messages.push(String::from(message));
        // }

        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // can't have two mutable borrows at the same time results in runtime error
            // we get mutable reference to Vec<String> from the immutable self via borrow_mut()
            one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }


    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_message.sent_messages.borrow().len(), 1);
    }
}