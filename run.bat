@ECHO OFF
SET "cp=%CD%"
cd .\dll\
START /b CMD /c RLBot_Injector.exe
cd /d C:\Program Files (x86)\Steam\
START /b CMD /c Steam.exe -applaunch 252950 -rlbot
cd /d %cp%
TIMEOUT /T 10
cargo run
pause