use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Self {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{:?} is not a valid subscriber name.", s)
        }
        Self(s)
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
