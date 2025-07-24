// mod sudoku;
// mod process;
// mod flow;
// mod sqlopt;
// mod color_paint;
// mod test_nom;
// mod img;
mod audio;
fn main() {
     let file = &std::env::args().nth(1).expect("Cannot open file.");
     let _ = audio::play_mp4(file);
}
