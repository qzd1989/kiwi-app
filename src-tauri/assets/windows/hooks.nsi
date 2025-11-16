!include "LogicLib.nsh"
ShowInstDetails show

!macro NSIS_HOOK_PREINSTALL
  Call CheckProcessesAndWait
  ; Call RemoveProjectTemplate
!macroend

Function CheckProcessesAndWait
  CheckPython:
    nsExec::ExecToStack 'powershell.exe -ExecutionPolicy Bypass -Command "Get-CimInstance Win32_Process | Where-Object { $_.ExecutablePath -and $_.ExecutablePath -like \"*python\interpreter\python.exe\" } | Select-Object -ExpandProperty ExecutablePath"'
    Pop $R0
    Pop $R1
    DetailPrint "check python process exit code: $R0"
    DetailPrint "check python process output: $R1"
    StrCmp $R1 "" PythonIsNotRunning PythonIsRunning

    PythonIsNotRunning:
      Goto ProcessesCleared

    PythonIsRunning:
      MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION "Python is running.$\r$\nPlease stop it and click Retry to continue, or Cancel to abort installation." IDRETRY CheckPython
      Goto Canceled

  ProcessesCleared:
    DetailPrint "All processes cleared. Continuing with installation..."
    Return

  Canceled:
    Abort "Installation cancelled by user."

FunctionEnd

Function RemoveProjectTemplate
  ${If} ${FileExists} "$INSTDIR\python\project_template"
    Push "$INSTDIR\python\project_template"
    Call RemoveDirectory
  ${EndIf}
  DetailPrint "Project template folder removed successfully!"
FunctionEnd

Function RemoveDirectory
  Exch $0
  ${If} ${FileExists} "$0"
    DetailPrint "Removing '$0', Please wait."
    nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "Remove-Item -Path \"$0\" -Recurse -Force"'
    Pop $1
    ${If} $1 != "0"
      Abort "Failed to remove directory '$0' with PowerShell Remove-Item."
    ${EndIf}
  ${EndIf}
FunctionEnd