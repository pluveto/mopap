// file = { section* }
#[derive(Debug, PartialEq)]
pub struct File {
    pub account_sections: Vec<AccountSection>,
    pub record_sections: Vec<RecordSection>,
}

#[derive(Debug, PartialEq)]
// section = { account_section | record_section }
pub enum Section {
    Account(AccountSection),
    Record(RecordSection),
}
// account_section = { "#" ~ KW_ACCOUNT ~ account_subsection* }

#[derive(Debug, PartialEq)]
pub struct AccountSection {
    pub subsections: Vec<AccountSubsection>,
}

//       account_subsection = { account_basic_info | account_init_states }
#[derive(Debug, PartialEq)]
pub enum AccountSubsection {
    BasicInfo(AccountBasicInfo),
    InitStates(AccountInitStates),
}

//       account_basic_info = { "##" ~ KW_BASIC_INFO ~ kv_pair * }

#[derive(Debug, PartialEq)]
pub struct AccountBasicInfo {
    pub pairs: Vec<KvPair>,
}

//           kv_pair = { kv_pair_key ~ ": " ~ kv_pair_value }

#[derive(Debug, PartialEq)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}
//             kv_pair_key = ${ (!(":") ~ ident_char)+ }
//             kv_pair_value = ${ (!("\n" | "\r") ~ ANY)* }

//       account_init_states = { "##" ~ KW_INT_STATE ~ account_init_state* }

#[derive(Debug, PartialEq)]
pub struct AccountInitStates {
    pub states: Vec<AccountInitState>,
}

//       account_init_state = { acount_name ~ KW_BALANCE ~ account_balance }

#[derive(Debug, PartialEq)]
pub struct AccountInitState {
    pub name: String,
    pub balance: MoneyAmount,
}

//       acount_name = ${ ident_char+ }

//       ident_char = _{ !(WHITESPACE) ~ ANY }

//       account_balance = { money_amount }

//             money_amount = _{ value ~ currency }

#[derive(Debug, PartialEq)]
pub struct MoneyAmount {
    pub value: String,
    pub currency: String,
}

//                   value  = { ("+" | "-" | "~" )? ~ (ASCII_DIGIT+ ~ ".")? ~ ASCII_DIGIT+ }

//                   currency = { "CNY" | "USD" | "JPY" | "EUR" }

// record_section = { "#" ~ KW_RECORD ~ record_day*}

#[derive(Debug, PartialEq)]
pub struct RecordSection {
    pub subsection: Vec<RecordSubsection>,
}

// record_day = { "## " ~ date ~ record_entry*}

#[derive(Debug, PartialEq)]
pub struct RecordSubsection {
    pub date: String,
    pub entries: Vec<RecordEntry>,
}

//     date = ${ year ~ "-" ~ month ~ "-"~ day}

//             year = _{ASCII_DIGIT+}

//             month = _{ASCII_DIGIT+}

//             day = _{ASCII_DIGIT+}

//     record_entry = { money_amount ~ acount_name ~ transfer? ~ note }

#[derive(Debug, PartialEq)]
pub struct RecordEntry {
    pub amount: MoneyAmount,
    pub trans_from: String,
    pub trans_to: Option<String>,
    pub is_transfer: bool,
    pub note: Note,
}
//         transfer = { KW_TRANSFER ~ acount_name }

//         note = { text ~ tags}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub text: String,
    pub tags: Vec<String>,
}
//         text = { note_char* }
//         tags = { hashtag* }
//         note_char = _{ !("\n" | "#") ~ ANY }
//         hashtag = ${ "#" ~ (!(" ") ~ note_char)+ }
