use debug::logger::Logger;

fn main() {
    start();
}

fn start() {
    //caching
    let mut test_logger = Logger::new("test logger", "logtest.log", true);
    for i in 0..10 {
        test_logger.add(&format!("test: {}", i));
    }
    test_logger.save().unwrap();
    let main_renderer = renderer::entry::ren_entry::MainNavigator;
    let url = String::from("http://127.0.0.1:5500/");
    //liverserver's url
    main_renderer.getter(url);

    // dbg!(&resized);
    // let termml_vec = construct_termml_vec(parsedml.clone(), hash.clone());
    // dbg!(termml_vec);
    // let renderer = renderer::debug::ren_debug::DebugRenderer;

    // renderer.entry(vec);
    // dbg!(styles_hash());
}
