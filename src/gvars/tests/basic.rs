#[test]
fn basic() {
    #[gvars::gvar(alias = "test-alias", alias = "test-alias2")]
    const TEST: u32 = 2;
    assert_eq!(*TEST, 2);

    if gvars::ENABLED {
        gvars::set("basic::TEST", "3").unwrap();
        assert_eq!(*TEST, 3);

        gvars::set("TEST", "4").unwrap();
        assert_eq!(*TEST, 4);

        gvars::set("test-alias", "7").unwrap();
        assert_eq!(*TEST, 7);

        gvars::set("test-alias2", "13").unwrap();
        assert_eq!(*TEST, 13);

        let metadata = gvars::metadata();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].unique_name, "basic::TEST");
    }
}