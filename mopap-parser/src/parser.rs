extern crate pest;
use pest::{
    error::{Error as ParseError, ErrorVariant},
    iterators::Pair,
    pratt_parser::PrattParser,
    Parser,
};
use pest_derive::Parser;

use crate::ast;

#[derive(Parser)]
#[grammar = "mopap.pest"]
pub struct MopapParser;
type ParseResult<T> = Result<T, ParseError<Rule>>;

impl MopapParser {
    pub fn new() -> Self {
        MopapParser
    }

    pub fn parse_src(&self, input: &str) -> ParseResult<ast::File> {
        let file_pairs = MopapParser::parse(Rule::file, input)?;
        let mut file = ast::File {
            account_sections: Vec::new(),
            record_sections: Vec::new(),
        };
        for sec in file_pairs {
            let section = parse_section(sec.into_inner().next().unwrap()).unwrap();
            match section {
                ast::Section::Account(s) => file.account_sections.push(s),
                ast::Section::Record(s) => file.record_sections.push(s),
            }
        }
        Ok(file)
    }
}

pub fn parse_section(pair: Pair<Rule>) -> ParseResult<ast::Section> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::account_section => {
            let mut account_section = ast::AccountSection {
                subsections: Vec::new(),
            };
            for subsec in pair.into_inner() {
                let subsection = parse_account_subsection(subsec)?;
                account_section.subsections.push(subsection);
            }
            Ok(ast::Section::Account(account_section))
        }
        Rule::record_section => {
            let mut record_section = ast::RecordSection {
                subsection: Vec::new(),
            };
            for subsec in pair.into_inner() {
                let subsection = parse_record_subsection(subsec)?;
                record_section.subsection.push(subsection);
            }
            Ok(ast::Section::Record(record_section))
        }
        _ => Err(ParseError::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Unexpected rule: {:?}", pair.as_rule()),
            },
            pair.as_span(),
        )),
    }
}

pub fn parse_account_subsection(pair: Pair<Rule>) -> ParseResult<ast::AccountSubsection> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::account_basic_info => {
            let mut basic_info = ast::AccountBasicInfo { pairs: Vec::new() };
            for pair in pair.into_inner() {
                let pair = parse_kv_pair(pair)?;
                basic_info.pairs.push(pair);
            }
            Ok(ast::AccountSubsection::BasicInfo(basic_info))
        }
        Rule::account_init_states => {
            let mut init_states = ast::AccountInitStates { states: Vec::new() };
            for pair in pair.into_inner() {
                let state = parse_account_init_state(pair)?;
                init_states.states.push(state);
            }
            Ok(ast::AccountSubsection::InitStates(init_states))
        }
        _ => Err(ParseError::new_from_span(
            ErrorVariant::CustomError {
                message: format!("Unexpected rule: {:?}", pair.as_rule()),
            },
            pair.as_span(),
        )),
    }
}

pub fn parse_kv_pair(pair: Pair<Rule>) -> ParseResult<ast::KvPair> {
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str().to_string();
    let value = inner.next().unwrap().as_str().to_string();
    Ok(ast::KvPair { key, value })
}

pub fn parse_account_init_state(pair: Pair<Rule>) -> ParseResult<ast::AccountInitState> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let balance = parse_account_balance(inner.next().unwrap())?;
    Ok(ast::AccountInitState { name, balance })
}

// { money_amount }
pub fn parse_account_balance(pair: Pair<Rule>) -> ParseResult<ast::MoneyAmount> {
    let amount = parse_money_amount(pair.into_inner().next().unwrap())?;
    Ok(amount)
}

// record_subsection = { "## " ~ date ~ record_entry*}
pub fn parse_record_subsection(pair: Pair<Rule>) -> ParseResult<ast::RecordSubsection> {
    let mut inner = pair.into_inner();
    let date = parse_date(inner.next().unwrap())?;
    let mut entries = Vec::new();
    for pair in inner {
        let entry = parse_record_entry(pair)?;
        entries.push(entry);
    }
    Ok(ast::RecordSubsection { date, entries })
}

pub fn parse_date(pair: Pair<Rule>) -> ParseResult<String> {
    Ok(pair.as_str().to_string())
}

// record_entry = { money_amount ~ acount_name ~ transfer? ~ note }
pub fn parse_record_entry(pair: Pair<Rule>) -> ParseResult<ast::RecordEntry> {
    let mut inner = pair.into_inner();
    let amount = parse_money_amount(inner.next().unwrap())?;
    let account = inner.next().unwrap().as_str().to_string();
    let mut is_transfer = false;
    let trans_to = match inner.next() {
        Some(pair) => {
            is_transfer = true;
            Some(parse_transfer(pair))
        }
        None => None,
    };
    let note = match inner.next() {
        Some(pair) => Some(parse_note(pair)),
        None => None,
    };
    Ok(ast::RecordEntry {
        amount,
        trans_from: account,
        is_transfer,
        trans_to,
        note: note.unwrap_or(Ok(ast::Note {
            text: "".to_string(),
            tags: Vec::new(),
        }))?,
    })
}
// note = { text ~ tags}
pub fn parse_note(pair: Pair<Rule>) -> ParseResult<ast::Note> {
    let mut inner = pair.into_inner();
    let text = inner.next().unwrap().as_str().to_string();
    let tags = parse_tags(inner.next().unwrap())?;
    Ok(ast::Note { text, tags })
}

// tags = { hashtag* }
// note_char = _{ !("\n" | "#") ~ ANY }
// hashtag = ${ "#" ~ (!(" ") ~ note_char)+ }
pub fn parse_tags(pair: Pair<Rule>) -> ParseResult<Vec<String>> {
    let mut tags = Vec::new();
    for pair in pair.into_inner() {
        let tag = pair.as_str().to_string();
        assert!(tag.starts_with("#"));
        let tag = tag[1..].to_string();
        tags.push(tag);
    }
    Ok(tags)
}

pub fn parse_transfer(pair: Pair<Rule>) -> String {
    let mut inner = pair.into_inner();
    let account = inner.next().unwrap().as_str().to_string();
    account
}

pub fn parse_money_amount(pair: Pair<Rule>) -> ParseResult<ast::MoneyAmount> {
    let mut inner = pair.into_inner();
    let value = inner.next().unwrap().as_str().to_string();
    let currency = inner.next().unwrap().as_str().to_string();
    Ok(ast::MoneyAmount { value, currency })
}

pub fn parse(input: &str) -> ParseResult<ast::File> {
    let parser = MopapParser::new();
    let ast = parser.parse_src(input)?;
    Ok(ast)
}
