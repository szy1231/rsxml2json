use serde_json::{to_string_pretty, Value};
use rsxml2json::{Convert, ConvertConfig};

fn main() {
    let conf = ConvertConfig::default();
    let convert = Convert::new(conf);
    let json_option = convert.execute(r#"<?xml version="1.0"?>
<Tests xmlns="http://www.adatum.com">
    <Test TestId="0001" TestType="CMD">
        <Name>Convert number to string</Name>
        <CommandLine>Examp1.EXE</CommandLine>
        <Input>1</Input>
        <Output>One</Output>
    </Test>
    <Test TestId="0002" TestType="CMD">
        <Name>Find succeeding characters</Name>
        <CommandLine>Examp2.EXE</CommandLine>
        <Input>abc</Input>
        <Output>def</Output>
    </Test>
    <Test TestId="0003" TestType="GUI">
        <Name>Convert multiple numbers to strings</Name>
        <CommandLine>Examp2.EXE /Verbose</CommandLine>
        <Input>123</Input>
        <Output>One Two Three</Output>
    </Test>
    <Test TestId="0004" TestType="GUI">
        <Name>Find correlated key</Name>
        <CommandLine>Examp3.EXE</CommandLine>
        <Input>a1</Input>
        <Output>b1</Output>
    </Test>
    <Test TestId="0005" TestType="GUI">
        <Name>Count characters</Name>
        <CommandLine>FinalExamp.EXE</CommandLine>
        <Input>This is a test</Input>
        <Output>14</Output>
    </Test>
    <Test TestId="0006" TestType="GUI">
        <Name>Another Test</Name>
        <CommandLine>Examp2.EXE</CommandLine>
        <Input>Test Input</Input>
        <Output>10</Output>
    </Test>
</Tests>"#.to_string());
    let json_str = match json_option {
        Some(val) => val,
        None => {return;}
    };
    println!("json_str = {}",json_str);
    let parsed_json: Value = serde_json::from_str(json_str.as_str()).expect("Unable to parse JSON");
    let pretty_json = to_string_pretty(&parsed_json).expect("Unable to convert to pretty JSON");
    println!("{}", pretty_json);
}