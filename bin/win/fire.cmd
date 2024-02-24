@echo off
setlocal

if "%~1"=="" (
    echo Usage: fire <rust_workspace_member_name> [additional_args]
    exit /b 1
)

set WORKSPACE_MEMBER=%~1
shift

set CMD_LINE=
:parse_args
if "%~1"=="" goto execute_cmd
set CMD_LINE=%CMD_LINE% %1
shift
goto parse_args

:execute_cmd
cargo run --package %WORKSPACE_MEMBER% --bin %WORKSPACE_MEMBER% %CMD_LINE%

endlocal