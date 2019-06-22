@ECHO OFF
SET "cp=%CD%"
cd .\dll\
START /b CMD /c RLBot_Injector.exe
ECHO Injecting DLLS
cd /d C:\Program Files (x86)\Steam\
START /b CMD /c Steam.exe -applaunch 252950 -rlbot
ECHO Starting Rocket League with -rlbot flag
cd /d %cp%
ECHO
ECHO Waiting for Rocket League to start and DLLS to be injected. Press any key when this is done or wait.
TIMEOUT /T 30 > NUL
cargo run
ECHO Bot no longer running... press any key to shutdown.
pause > NUL