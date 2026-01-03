use simpletron_rust::parser::{ParserInterface, lowlevel_parser::LowLevelParser};

fn main() {
    let parser = LowLevelParser::new(true);

    let _ = parser.parse("test.sml".into());
}
