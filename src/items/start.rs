use crate::items::termination::Termination;

#[lang = "start"]
fn start<T: Termination>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _sigpipe: u8,
) -> isize {
    let result = main();
    result.into_code()
}
