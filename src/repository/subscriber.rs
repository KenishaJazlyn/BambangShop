use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;
use super::product;
// Singleton of Database
lazy_static! {
    pub static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(product_type.to_string(), DashMap::new());
        }

        SUBSCRIBERS.get(product_type).unwrap().
            insert(subscriber_value.url.clone(), subscriber_value.clone());
        return subscriber;
    }
}