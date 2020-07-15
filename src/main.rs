use clap::{Arg, App};

fn parse_args() -> clap::ArgMatches<'static> {
    return App::new("Audience calculator")
                .about("Helps to calculate audience for experiments")
                .arg(Arg::with_name("current")
                         .short("c")
                         .long("current")
                         .takes_value(true)
                         .required(true)
                         .help("Current conversion"))
                .arg(Arg::with_name("expected")
                         .short("e")
                         .long("expected")
                         .takes_value(true)
                         .required(true)
                         .help("Expected conversion"))
                .arg(Arg::with_name("step")
                         .short("step")
                         .long("step")
                         .takes_value(true)
                         .help("Pass current audience to calculate signigicant conversion"))
                .get_matches();
}

fn audience(current_conversion: f64, expected_conversion: f64) {
    let significance_level: f64 = 1.96; // p = 95, 1.654 (p = 90), 2.575 (p = 99)
    let statistical_power: f64 = 0.84;

    let audience = (
        (
            (significance_level + statistical_power).powi(2) *
            (current_conversion * (1.0 - current_conversion) + expected_conversion * (1.0 - expected_conversion))
        ) / (current_conversion - expected_conversion).powi(2)).ceil() * 2.0;

    println!("Audience size ({:.2}% -> {:.2}%): {}", current_conversion * 100.0, expected_conversion * 100.0, audience);
}

fn audience_with_step(mut current_conversion: f64, expected_conversion: f64, step: f64) {
    println!(
        "Calculating audiences for conversions between {:.2}% and {:.2}% with {:.2}% step\n",
        current_conversion * 100.0, expected_conversion * 100.0, step * 100.0
    );

    loop {
        audience(current_conversion, expected_conversion);
        current_conversion += step;

        if current_conversion > expected_conversion {
            break;
        }
    }
}

fn main() {
    let matches = parse_args();

    let current_conversion_arg = matches.value_of("current").unwrap();
    let current_conversion = current_conversion_arg.parse::<f64>().expect("Failed parse number!") / 100.0;

    let expected_conversion_arg = matches.value_of("expected").unwrap();
    let expected_conversion = expected_conversion_arg.parse::<f64>().expect("Failed parse number!") / 100.0;

    match matches.value_of("step") {
        Some(arg_value) => {
            let step = arg_value.parse::<f64>().expect("Failed parse number!") / 100.0;
            audience_with_step(current_conversion, expected_conversion, step);
        },
        None => {
            audience(current_conversion, expected_conversion);
        }
    }
}
