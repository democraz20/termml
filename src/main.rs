use debug::logger::Logger;

fn main() {
    start();
}

use renderer::entry::ren_entry::split_chunk::CharChunksTrait;


fn start() {
    //testing split
    let t = "aaabbbcccdddeeefffggg";
    println!("{:?}", t.char_chunks(6).collect::<Vec<_>>());

    //caching
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
