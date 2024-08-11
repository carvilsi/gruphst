use gruphst::logger::enable_logging;

#[test]
fn should_enable_logging() {
    enable_logging(log::Level::Debug);
}
