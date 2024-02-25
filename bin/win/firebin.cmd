@echo off
setlocal

if "%~1"=="" (
    echo "Usage: fire.cmd <package> <binary> [additional_args]"
    exit /b 1
)

set RUST_PACKAGE=%1
if "%2"=="" (
    echo "Usage: fire.cmd <package> <binary> [additional_args]"
    exit /b 1
)

set RUST_BINARY=%2
shift
shift

set CMD_LINE=

:parse_args
if "%~1"=="" goto execute_cmd
set CMD_LINE=%CMD_LINE% %1
shift
goto parse_args

:execute_cmd
cargo run --package %RUST_PACKAGE% --bin %RUST_BINARY% %CMD_LINE%

endlocal