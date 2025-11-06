!include "LogicLib.nsh"
ShowInstDetails show

; new structure
; python/.interpreter
; python/.project_template

; installed structure
; python/interpreter
; python/project_template


!macro NSIS_HOOK_POSTINSTALL
  ; Call RemoveAllTemporaryDirectories
  ; Call ExtractAll
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

; Function ExtractAll
;   CreateDirectory "$INSTDIR\.python\interpreter"
;   DetailPrint "Extracting python..."
;   nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "try { Expand-Archive -Path \"$INSTDIR\python\interpreter.zip\" -DestinationPath \"$INSTDIR\.python\interpreter\" -Force; exit 0 } catch { Write-Host $_.Exception.Message; exit 1 }"'
;   Pop $0
;   ${If} $0 != "0"
;     Abort "Failed to extract python into $INSTDIR\.python\interpreter\."
;   ${EndIf}

;   DetailPrint "All ZIP files were extracted successfully."
; FunctionEnd

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

  ; rename python\.interpreter to python\interpreter
  Rename "$INSTDIR\python\.interpreter" "$INSTDIR\python\interpreter"
  ${If} ${Errors}
    Abort "Rename $INSTDIR\python\.interpreter to $INSTDIR\python\interpreter failed!"
  ${EndIf}

  ; remove python\project_template
  ${If} ${FileExists} "$INSTDIR\python\project_template"
    Push "$INSTDIR\python\project_template"
    Call RemoveDirectory
  ${EndIf}

  ; rename python\.project_template to python\project_template
  Rename "$INSTDIR\python\.project_template" "$INSTDIR\python\project_template"
  ${If} ${Errors}
    Abort "Rename $INSTDIR\python\.project_template to $INSTDIR\python\project_template failed!"
  ${EndIf}

  ; install uv
  Call InstallUv
FunctionEnd

Function InstallUv
  DetailPrint "Checking for uv wheel file..."
  FindFirst $0 $1 "$INSTDIR\python\project_template\wheels\uv-*.whl"
  StrCmp $1 "" NoUvWheelFound

  ; Found uv wheel file
  StrCpy $R0 "$INSTDIR\python\project_template\wheels\$1"
  StrCpy $R1 "$INSTDIR\python\project_template\wheels"
  StrCpy $R2 "$INSTDIR\python\interpreter\python.exe"

  DetailPrint "Found uv wheel: $R0"
  DetailPrint "Installing uv via pip..."
  nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "& \"$R2\" -m pip install --no-index --find-links=\"$R1\" \"$R0\""'
  Pop $R3

  ${If} $R3 != "0"
    Abort "Failed to install uv from $R0"
  ${Else}
    DetailPrint "uv installed successfully."
  ${EndIf}

  FindClose $0
  Goto UvInstallDone

  NoUvWheelFound:
    DetailPrint "No uv wheel file found, skipping installation."
    FindClose $0

  UvInstallDone:
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
  
  ; Remove .tar files in wheels directory todo

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