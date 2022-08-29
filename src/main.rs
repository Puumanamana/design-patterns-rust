pub use design_patterns::factory_method::{NotificationFactory, NotificationType};

fn main() {
    let push_notif = NotificationFactory::create_notification(NotificationType::Push);
    push_notif.notify_user();
}
