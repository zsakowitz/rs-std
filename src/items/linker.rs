#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {}

#[cfg(target_os = "macos")]
#[link(name = "System")]
extern "C" {}
