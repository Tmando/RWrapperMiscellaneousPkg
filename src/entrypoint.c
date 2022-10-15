// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_RWrapperMiscellaneousPkg_extendr(void *dll);

void R_init_RWrapperMiscellaneousPkg(void *dll) {
    R_init_RWrapperMiscellaneousPkg_extendr(dll);
}
