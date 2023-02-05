use mopap_parser::{
    ast::MoneyAmount,
    parser::{self, MopapParser, Rule},
};
use pest::Parser;

#[test]

fn test_note() {
    let mut pairs = parser::MopapParser::parse(Rule::note, "text #tag1 #tag2")
        .unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let note = parser::parse_note(pair).unwrap();
    assert_eq!(note.text, "text");
    assert_eq!(note.tags.len(), 2);
    assert_eq!(note.tags[0], "tag1");
    assert_eq!(note.tags[1], "tag2");
}

#[test]
fn test_transfer() {
    let mut pairs = parser::MopapParser::parse(Rule::transfer, "to account")
        .unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let account = parser::parse_transfer(pair);
    assert_eq!(account, "account");
}

#[test]
fn test_record_entry() {
    let mut pairs = parser::MopapParser::parse(
        Rule::record_entry,
        "~100.00 USD account to account2 #tag1 #tag2",
    )
    .unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let entry = parser::parse_record_entry(pair).unwrap();
    assert_eq!(
        entry.amount,
        MoneyAmount {
            value: "~100.00".to_string(),
            currency: "USD".to_string(),
        }
    );
}
#[test]

fn test_date() {
    let mut pairs =
        parser::MopapParser::parse(Rule::date, "2020-01-01").unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let date = parser::parse_date(pair).unwrap();
    assert_eq!(date, "2020-01-01".to_string());
}

#[test]
fn test_record_subsection() {
    let mut pairs = parser::MopapParser::parse(
        Rule::record_subsection,
        r#"## 2020-01-01
        -100.00 USD account spend money #tag1 #tag2
        ~200.00 USD account to account2 #tag1 #tag2
    "#,
    )
    .unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let subsection = parser::parse_record_subsection(pair).unwrap();
    assert_eq!(subsection.date, "2020-01-01".to_string());
    assert_eq!(subsection.entries.len(), 2);
    assert_eq!(
        subsection.entries[0].amount,
        MoneyAmount {
            value: "-100.00".to_string(),
            currency: "USD".to_string()
        }
    );
    assert_eq!(
        subsection.entries[1].amount,
        MoneyAmount {
            value: "~200.00".to_string(),
            currency: "USD".to_string()
        }
    );
}

#[test]
fn test_account_init_state() {
    let mut pairs =
        parser::MopapParser::parse(Rule::account_init_state, "account1 balance 100.00 USD")
            .unwrap_or_else(|e| panic!("{}", e));
    let pair = pairs.next().unwrap();
    let account = parser::parse_account_init_state(pair).unwrap();
    assert_eq!(account.name, "account1".to_string());
    assert_eq!(
        account.balance,
        MoneyAmount {
            value: "100.00".to_string(),
            currency: "USD".to_string()
        }
    );
}
