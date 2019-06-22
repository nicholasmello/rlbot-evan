@ECHO OFF
pip install rlbot
cargo build && python -c "from rlbot import runner; runner.main()"
ECHO Bot no longer running... press any key to shutdown.
TIMEOUT /T 120 > NUL