use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let is_not_alphabetic = s.chars().any(|c| !c.is_alphabetic());
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace
            || is_too_long
            || is_not_alphabetic
            || contains_forbidden_characters
        {
            panic!("{} is not a valid subscriber name.", s);
        }

        Self(s)
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
