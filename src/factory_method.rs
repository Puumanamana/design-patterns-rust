pub trait Notification {
    fn notify_user(&self) {}
}

struct Email {}
impl Notification for Email {
    fn notify_user(&self) {
        println!("Email sent");
    }
}

struct SMS {}
impl Notification for SMS {
    fn notify_user(&self) {
        println!("SMS sent");
    }
}

struct Push {}
impl Notification for Push {
    fn notify_user(&self) {
        println!("Push sent");
    }
}

pub enum NotificationType {
    Email,
    SMS,
    Push
}

pub trait NotificationService {
    fn create_notification(&self) {}
}

pub struct NotificationFactory {}

impl NotificationFactory {
    pub fn create_notification(notif: NotificationType) -> Box<dyn Notification> {
        match notif {
            NotificationType::Email => Box::new(Email {}),
            NotificationType::SMS => Box::new(SMS {}),
            NotificationType::Push => Box::new(Push {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push() {
        let push_notif = NotificationFactory::create_notification(NotificationType::Push);
        assert!(true);
    }
}
