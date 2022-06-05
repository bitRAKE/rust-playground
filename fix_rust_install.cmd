REM requires Admin or developer mode:
REM run this in C:\Users\%USER%\.cargo\bin
REM to remove duplicate files and use SYMLINKs

for %%G in (
  cargo-clippy
  cargo-fmt
  cargo-miri
  cargo
  clippy-driver
  rls
  rust-gdb
  rust-lldb
  rustdoc
  rustfmt
  rustup
) DO (
  del %%G.exe
  mklink %%G.exe rustc.exe
)
