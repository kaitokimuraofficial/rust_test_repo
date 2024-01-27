mod cell_and_refcell;

// mod chapter1;
// mod chapter2;
// mod chapter4;
// mod chapter5;
mod chapter6;

fn main() {
    //     // chapter1::thread_use();
    //     // chapter2::atomic();
    //     // chapter4::do_chapter();
    //     // chapter5::do_channel();
    chapter6::do_arc();
    cell_and_refcell::use_cell_and_refcell();
}
