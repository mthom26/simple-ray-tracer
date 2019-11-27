use clap::{App, Arg};

pub fn get_config() -> (u64, u64, String) {
    let matches = App::new("Simple Ray Tracer")
        .arg(
            Arg::with_name("dimensions")
                .help("The image width and height")
                .short("d")
                .long("dimensions")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["width, height"]),
        )
        .arg(
            Arg::with_name("scene")
                .help("The scene to render")
                .short("s")
                .long("scene")
                .takes_value(true)
                .value_name("scene"),
        )
        .get_matches();

    let (width, height): (u64, u64) = match matches.values_of("dimensions") {
        Some(mut vals) => (
            // These unwraps are a bit ugly but they should be ok, the outer
            // match on `matches.values_of()` should catch any errors.
            vals.next().unwrap().parse().unwrap(),
            vals.next().unwrap().parse().unwrap(),
        ),
        None => (200, 100),
    };

    let scene = match matches.value_of("scene") {
        Some(val) => val.to_owned(),
        None => "default".to_owned(),
    };

    (width, height, scene)
}
