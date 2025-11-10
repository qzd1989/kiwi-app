!include "LogicLib.nsh"
ShowInstDetails show

!macro NSIS_HOOK_PREINSTALL
  Call CheckProcessesAndWait
!macroend

; Only check python process
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