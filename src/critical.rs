use critical_section::RawRestoreState;

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

// No, this isn't safe to have empty, but it's only here to allow using OnceCell when loading the OpenGL function
// addresses, and we're only running one thread anyway, so leaving it empty to save a few bytes seems reasonable.

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {}

    unsafe fn release(_: RawRestoreState) {}
}
