fn main() {
    start();
}

use renderer::entry::ren_entry::split_chunk::CharChunksTrait;

fn start() {
    //testing split
    let t = "aaabbbcccdddeeefffggg";
    println!("{:?}", t.char_chunks(3).collect::<Vec<_>>());
    //caching
    let main_renderer = renderer::entry::ren_entry::MainNavigator;
    let url = String::from("http://127.0.0.1:5500/");
    //liverserver's url
    main_renderer.getter(url, false);

}
