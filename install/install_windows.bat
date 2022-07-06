cd %userprofile%
mkdir dbh
set src=%~dp0..\src
cd %src%
cargo build -r
cd target\release
move dbh.exe %userprofile%\dbh
setx path "%path%;%userprofile%\dbh"