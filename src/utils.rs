pub mod id {
    use nanoid::nanoid;

    pub fn new() -> String {
        nanoid!(14)
    }
}
