fn main() {
    // Thin binary wrapper: delegate all logic to the library orchestration layer
    pingdown::app::App::default().run();
}
