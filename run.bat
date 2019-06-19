SET "cp=%CD%"
cd .\dll\
RLBot_Injector.exe
cd /d C:\Program Files (x86)\Steam\
Steam.exe -applaunch 252950 -rlbot
cd /d %cp%
cargo run
pause