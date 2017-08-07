#[cfg(feature = "readme-testing")]
extern crate skeptic;

#[cfg(feature = "readme-testing")]
fn readme_testing() {
    skeptic::generate_doc_tests(&["README.md"]);
}

#[cfg(not(feature = "readme-testing"))]
fn readme_testing() {}

fn main() {
    readme_testing();
}
