!include "LogicLib.nsh"
ShowInstDetails show

!macro NSIS_HOOK_POSTINSTALL
  Call RemoveAllTemporaryDirectories
  Call ExtractAll
  Call CheckProcessesAndWait
  Call DoInstall
  Call Cleanup
!macroend

Function RemoveAllTemporaryDirectories
  ${If} ${FileExists} "$INSTDIR\.python"
    Push "$INSTDIR\.python"
    Call RemoveDirectory
  ${EndIf}
FunctionEnd

Function ExtractAll
  CreateDirectory "$INSTDIR\.python\interpreter"
  DetailPrint "Extracting python..."
  nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "try { Expand-Archive -Path \"$INSTDIR\python\interpreter.zip\" -DestinationPath \"$INSTDIR\.python\interpreter\" -Force; exit 0 } catch { Write-Host $_.Exception.Message; exit 1 }"'
  Pop $0
  ${If} $0 != "0"
    Abort "Failed to extract python into $INSTDIR\.python\interpreter\."
  ${EndIf}

  DetailPrint "All ZIP files were extracted successfully."
FunctionEnd

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

Function DoInstall
  ; remove python\interpreter
  ${If} ${FileExists} "$INSTDIR\python\interpreter"
    Push "$INSTDIR\python\interpreter"
    Call RemoveDirectory
  ${EndIf}

  ; rename .python\interpreter to python\interpreter
  Rename "$INSTDIR\.python\interpreter" "$INSTDIR\python\interpreter"
  ${If} ${Errors}
    Abort "Rename $INSTDIR\.python\interpreter to $INSTDIR\python\interpreter failed!"
  ${EndIf}
FunctionEnd

Function Cleanup
  ${If} ${FileExists} "$INSTDIR\.python"
    Push "$INSTDIR\.python"
    Call RemoveDirectory
  ${EndIf}

  ${If} ${FileExists} "$INSTDIR\python\interpreter.zip"
    Push "$INSTDIR\python\interpreter.zip"
    Call RemoveDirectory
  ${EndIf}
  DetailPrint "Installation completed successfully!"
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