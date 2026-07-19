use iced::Element;

pub fn card<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {}
