mod errors;
mod dataset;

fn main() {
    dataset::initialize().expect("");
}
