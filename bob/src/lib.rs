#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

#[derive(Debug)]
enum Conversation {
    Question,
    Yell,
    Statement,
    Silence,
}

enum BobReplies {
    Sure,
    Whoa,
    Whatever,
    Fine,
}

#[derive(Debug, PartialEq)]
enum CharTypes {
    UpperCase,
    LowerCase,
    QuestionMark,
    Exclamation,
    WhiteSpace,
}

impl CharTypes {
    pub fn from_char(char_value: char) -> CharTypes {
        match char_value {
            _ if char_value.is_ascii_uppercase() => CharTypes::UpperCase,
            _ if char_value.is_ascii_lowercase() => CharTypes::LowerCase,
            _ if char_value == '?' => CharTypes::QuestionMark,
            _ if char_value == '!' => CharTypes::Exclamation,
            _ if char_value == ' ' => CharTypes::WhiteSpace,
            _ => CharTypes::LowerCase,
        }
    }
}

impl Conversation {
    fn is_question(message: &[CharTypes]) -> bool {
        let last_char = message.last();
        match last_char {
            Some(&CharTypes::QuestionMark) => true,
            _ => false,
        }
    }

    fn is_yell(message: &[CharTypes]) -> bool {
        let is_upper_r_exclaim =
            |ref val: &CharTypes| **val == CharTypes::UpperCase || **val == CharTypes::Exclamation;
        let is_yell_conv: bool = message.into_iter().map(is_upper_r_exclaim).fold(
            true,
            |acc, x| acc && x,
        );
        is_yell_conv
    }
    pub fn to_conversation(message: &[CharTypes]) -> Conversation {
        match message {
            _ if Self::is_question(message) => Conversation::Question,
            _ if Self::is_yell(message) => Conversation::Yell,
            _ => Conversation::Statement,
        }
    }
}





pub fn reply(message: &str) -> &str {
    let conv_type: Vec<CharTypes> = message.chars().map(CharTypes::from_char).collect();
    println!(
        "{:?} {:?}",
        conv_type,
        Conversation::to_conversation(conv_type.as_slice())
    );
    "Hello"
}
