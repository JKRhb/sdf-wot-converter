use sdf_wot_converter::{converter, ConverterResult};

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgGroup,
};
use std::{env, fs};
use url::Url;

const SDF_INPUT_NAME: &str = "SDF input file";
const SDF_OUTPUT_NAME: &str = "SDF output file";
const TM_INPUT_NAME: &str = "TM input file";
const TM_OUTPUT_NAME: &str = "TM output file";
const TD_INPUT_NAME: &str = "TD input file";

type ConversionFunction<'a> = &'a dyn Fn(String) -> ConverterResult<String>;
type PrintFunction<'a> = &'a dyn Fn(String) -> ConverterResult<()>;

type MatchSubcommandFunction<'a> = &'a dyn Fn(&&clap::ArgMatches) -> ConverterResult<()>;

#[derive(Debug, PartialEq)]
enum InputPathType {
    File,
    ValidUrl,
    InvalidUrl,
}

fn determine_path_type(path: &str) -> InputPathType {
    match Url::from_file_path(path) {
        Ok(_) => InputPathType::File,
        Err(_) => match Url::parse(path) {
            Ok(url) => {
                if url.scheme() == "http" || url.scheme() == "https" {
                    InputPathType::ValidUrl
                } else if url.scheme() == "file" {
                    InputPathType::File
                } else {
                    InputPathType::InvalidUrl
                }
            }
            Err(_) => InputPathType::File,
        },
    }
}

fn write_to_file(path: &str, content: String) -> ConverterResult<()> {
    fs::write(path, content).map_err(|e| e.into())
}

fn write_to_another_file(input_path: &str, output_path: &str) -> ConverterResult<()> {
    let content = get_json(input_path)?;
    write_to_file(output_path, content)
}

fn get_json_from_file(path: &str) -> ConverterResult<String> {
    fs::read_to_string(&path).map_err(|e| e.into())
}

fn get_json_from_url(url: &str) -> ConverterResult<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn get_json(path: &str) -> ConverterResult<String> {
    let path_type = determine_path_type(path);
    match path_type {
        InputPathType::File => get_json_from_file(path),
        InputPathType::ValidUrl => get_json_from_url(path),
        InputPathType::InvalidUrl => Err("Invalid URL or file path!".into()),
    }
}

fn print_model_from_file(path: &str, print_function: PrintFunction) -> ConverterResult<()> {
    let json_string = get_json(path)?;
    print_function(json_string)
}

fn convert(
    input_path: &str,
    output_path: &str,
    conversion_function: ConversionFunction,
) -> ConverterResult<()> {
    let input_string = get_json(input_path)?;
    let output_string = conversion_function(input_string)?;
    write_to_file(output_path, output_string)
}

fn match_print_arguments(print_command: &&clap::ArgMatches) -> ConverterResult<()> {
    if let Some(input_path) = print_command.value_of(SDF_INPUT_NAME) {
        print_model_from_file(input_path, &converter::print_sdf_definition)
    } else if let Some(input_path) = print_command.value_of(TD_INPUT_NAME) {
        print_model_from_file(input_path, &converter::print_wot_td_definition)
    } else if let Some(input_path) = print_command.value_of(TM_INPUT_NAME) {
        print_model_from_file(input_path, &converter::print_wot_tm_definition)
    } else {
        Err("No legal argument for print subcommand found!".into())
    }
}

fn match_convert_arguments(convert_command: &&clap::ArgMatches) -> ConverterResult<()> {
    let output_error_message = "No legal output path argument given!";
    if let Some(input_path) = convert_command.value_of(SDF_INPUT_NAME) {
        if let Some(output_path) = convert_command.value_of(TM_OUTPUT_NAME) {
            convert(input_path, output_path, &converter::convert_sdf_to_wot_tm)
        } else if let Some(output_path) = convert_command.value_of(SDF_OUTPUT_NAME) {
            write_to_another_file(input_path, output_path)
        } else {
            Err(output_error_message.into())
        }
    } else if let Some(input_path) = convert_command.value_of(TM_INPUT_NAME) {
        if let Some(output_path) = convert_command.value_of(SDF_OUTPUT_NAME) {
            convert(input_path, output_path, &converter::convert_wot_tm_to_sdf)
        } else if let Some(output_path) = convert_command.value_of(TM_OUTPUT_NAME) {
            write_to_another_file(input_path, output_path)
        } else {
            Err(output_error_message.into())
        }
    } else {
        Err("No legal input path argument given!".into())
    }
}

fn match_arguments(
    app: clap::ArgMatches,
    match_print_command_function: MatchSubcommandFunction,
    match_convert_command_function: MatchSubcommandFunction,
) -> ConverterResult<()> {
    if let Some(ref matches) = app.subcommand_matches("print") {
        match_print_command_function(matches)
    } else if let Some(ref matches) = app.subcommand_matches("convert") {
        match_convert_command_function(matches)
    } else {
        Err("No known subcommand found!".into())
    }
}

fn create_app() -> clap::App<'static, 'static> {
    app_from_crate!()
        .subcommand(
            App::new("print")
                .about("Reads in an SDF or WoT file and prints it in the terminal.")
                .arg(
                    Arg::with_name(SDF_INPUT_NAME)
                        .long("sdf")
                        .help("Reads in an SDF file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(TM_INPUT_NAME)
                        .long("tm")
                        .help("Reads in a WoT Thing Model file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(TD_INPUT_NAME)
                        .long("td")
                        .help("Reads in a WoT Thing Description file.")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("input")
                        .args(&[SDF_INPUT_NAME, TM_INPUT_NAME, TD_INPUT_NAME])
                        .required(true),
                ),
        )
        .subcommand(
            App::new("convert")
                .about("Reads in an SDF or WoT file and converts it into another format.")
                .arg(
                    Arg::with_name(SDF_INPUT_NAME)
                        .long("from-sdf")
                        .help("Reads in an SDF file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(TM_INPUT_NAME)
                        .long("from-tm")
                        .help("Reads in a WoT Thing Model file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(TM_OUTPUT_NAME)
                        .long("to-tm")
                        .help("Converts to a WoT Thing Model and writes it to a file.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(SDF_OUTPUT_NAME)
                        .long("to-sdf")
                        .help("Converts to a WoT Thing Model and writes it to a file.")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("from")
                        .args(&[SDF_INPUT_NAME, TM_INPUT_NAME])
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("to")
                        .args(&[TM_OUTPUT_NAME, SDF_OUTPUT_NAME])
                        .required(true),
                ),
        )
}

fn main() -> ConverterResult<()> {
    let app = create_app().get_matches();

    match_arguments(app, &match_print_arguments, &match_convert_arguments)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_dir() {
        let _ = fs::create_dir_all("test_output");
    }

    fn successful_print_function(_input: String) -> ConverterResult<()> {
        Ok(())
    }

    fn failing_print_function(_input: String) -> ConverterResult<()> {
        Err("This is an error".into())
    }

    fn successful_converter_function(_input: String) -> ConverterResult<String> {
        Ok(String::new())
    }

    fn failing_converter_function(_input: String) -> ConverterResult<String> {
        Err("This is an error".into())
    }

    #[test]
    fn write_to_another_file_test() {
        create_test_dir();
        assert!(
            write_to_another_file("examples/sdf/example.sdf.json", "test_output/barfoo.json")
                .is_ok()
        );
    }

    #[test]
    fn print_model_from_path_test() {
        assert!(
            print_model_from_file("examples/sdf/example.sdf.json", &successful_print_function)
                .is_ok()
        );
        assert!(
            print_model_from_file("examples/sdf/example.sdf.json", &failing_print_function)
                .is_err()
        );
        assert!(print_model_from_file("foobar.json", &successful_print_function).is_err());
    }

    #[test]
    fn convert_test() {
        create_test_dir();
        let working_input_path = "examples/sdf/example.sdf.json";
        let failing_input_path = "foobar.json";
        let output_path = "test_output/foobar.tm.json";
        assert!(convert(
            working_input_path,
            output_path,
            &successful_converter_function
        )
        .is_ok());
        assert!(convert(working_input_path, output_path, &failing_converter_function).is_err());
        assert!(convert(
            failing_input_path,
            output_path,
            &successful_converter_function
        )
        .is_err());
    }

    #[test]
    fn determine_path_type_test() {
        assert_eq!(
            InputPathType::ValidUrl,
            determine_path_type("https://example.org")
        );
        assert_eq!(
            InputPathType::InvalidUrl,
            determine_path_type("coap://example.org")
        );
        assert_eq!(InputPathType::File, determine_path_type("file://foobar"));
        assert_eq!(InputPathType::File, determine_path_type("foobar"));
        assert_eq!(InputPathType::File, determine_path_type("./foobar"));
        assert_eq!(InputPathType::File, determine_path_type("~/foobar"));
        if cfg!(windows) {
            assert_eq!(InputPathType::File, determine_path_type("C:\\foobar"));
        }
    }
}
