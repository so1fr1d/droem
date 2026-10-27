; =============================================================================
; src-tauri/windows/hooks.nsh
; Drøm (drøm) v0.1.0 — Phase 38-F Final NSIS Hooks (10 Official Audio Formats)
;
; Architecture & Safety Guarantees:
; 1. Official 10 Supported Formats (`mp3, flac, wav, m4a, aac, ogg, opus, wma, aif, aiff`):
;    - Registers one canonical ProgID per extension (`Droem.Audio.<ext>`), `OpenWithProgids`,
;      `Capabilities\FileAssociations`, `RegisteredApplications`, `SupportedTypes`, and
;      per-format icons (`audio_<ext>.ico`).
; 2. Safe Upgrade Cleanup of Excluded Formats (`wv, ape, mpc`):
;    - When upgrading from an earlier 13-format build, `NSIS_HOOK_POSTINSTALL` runs
;      `!insertmacro DroemUnregisterAudioExt` for `wv`, `ape`, and `mpc`.
;    - Removes ONLY Drøm-owned (`"DroemCreated"=="1"`) registry values and Drøm's own
;      entry under `OpenWithProgids`, `SupportedTypes`, and `Capabilities\FileAssociations`.
;    - NEVER touches `Software\Classes\.<ext>` `(Default)` value, `HKCU`, `UserChoice`,
;      other programs' `OpenWithProgids`, or shared ProgIDs not owned by Drøm.
; 3. Install Failure & Reinstall Resilience:
;    - `NSIS_HOOK_PREINSTALL` performs zero registry writes (only normalizes `$InstDir` to
;      `C:\Program Files\Droem`).
;    - `NSIS_HOOK_POSTINSTALL` verifies `IfFileExists "$INSTDIR\${MAINBINARYNAME}.exe"` before
;      writing any registry entries, and checks `IfFileExists "$INSTDIR\icons\audio_<ext>.ico"`
;      with fallback to `$INSTDIR\${MAINBINARYNAME}.exe,0`.
; =============================================================================

!ifndef DROEM_NSIS_HOOKS_INCLUDED
!define DROEM_NSIS_HOOKS_INCLUDED

!macro DroemRegisterAudioExt EXT_LOWER DESC_TEXT
  ; Resolve per-extension icon path with safe fallback if icon file is missing
  StrCpy $R0 "$INSTDIR\icons\audio_${EXT_LOWER}.ico,0"
  IfFileExists "$INSTDIR\icons\audio_${EXT_LOWER}.ico" +2 0
    StrCpy $R0 "$INSTDIR\${MAINBINARYNAME}.exe,0"

  ; 1. Register single canonical ProgID (`Droem.Audio.<ext>`) under SHCTX (HKLM)
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "" "${DESC_TEXT}"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "FriendlyTypeName" "${DESC_TEXT}"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "DroemCreated" "1"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\DefaultIcon" "" "$R0"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell" "" "open"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" "" "Play with Drøm"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" "FriendlyAppName" "Drøm"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" "Icon" "$R0"
  WriteRegStr SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open\command" "" "$\"$INSTDIR\${MAINBINARYNAME}.exe$\" $\"%1$\""

  ; 2. Register non-destructive OpenWithProgids value, SupportedTypes, and Capabilities
  ;    Note: `Software\Classes\.${EXT_LOWER}` default value ("") is NEVER modified!
  WriteRegStr SHCTX "Software\Classes\.${EXT_LOWER}\OpenWithProgids" "Droem.Audio.${EXT_LOWER}" ""
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\SupportedTypes" ".${EXT_LOWER}" ""
  WriteRegStr SHCTX "Software\Droem\Capabilities\FileAssociations" ".${EXT_LOWER}" "Droem.Audio.${EXT_LOWER}"
!macroend

!macro DroemUnregisterAudioExt EXT_LOWER
  ; 1. Remove ONLY Drøm's own value from shared OpenWithProgids, SupportedTypes, and Capabilities.
  ;    `Software\Classes\.${EXT_LOWER}` default value ("") and shared keys are NEVER modified or deleted.
  DeleteRegValue SHCTX "Software\Classes\.${EXT_LOWER}\OpenWithProgids" "Droem.Audio.${EXT_LOWER}"
  DeleteRegKey /ifempty SHCTX "Software\Classes\.${EXT_LOWER}\OpenWithProgids"
  DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\SupportedTypes" ".${EXT_LOWER}"
  DeleteRegValue SHCTX "Software\Droem\Capabilities\FileAssociations" ".${EXT_LOWER}"

  ; 2. Verify Drøm ownership before removing `Droem.Audio.<ext>` individual values and empty subkeys
  ClearErrors
  ReadRegStr $R0 SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "DroemCreated"
  ReadRegStr $R1 SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open\command" ""
  ${If} $R0 == "1"
  ${OrIf} $R1 == "$\"$INSTDIR\${MAINBINARYNAME}.exe$\" $\"%1$\""
  ${OrIf} $R1 == "$INSTDIR\${MAINBINARYNAME}.exe $\"%1$\""
    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open\command" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open\command"

    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" ""
    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" "FriendlyAppName"
    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open" "Icon"
    DeleteRegKey /ifempty SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell\open"

    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\shell"

    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\DefaultIcon" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}\DefaultIcon"

    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" ""
    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "FriendlyTypeName"
    DeleteRegValue SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}" "DroemCreated"
    DeleteRegKey /ifempty SHCTX "Software\Classes\Droem.Audio.${EXT_LOWER}"
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREINSTALL
  ; Normalize installation directory to ASCII "C:\Program Files\Droem" while keeping display name "Drøm".
  ; Performs ZERO registry writes so aborted/failed installations leave no registry state behind.
  ${If} $InstDir == "$PROGRAMFILES64\${PRODUCTNAME}"
  ${OrIf} $InstDir == "$PROGRAMFILES64\Drøm"
  ${OrIf} $InstDir == "$PROGRAMFILES64\drøm"
  ${OrIf} $InstDir == "$LOCALAPPDATA\${PRODUCTNAME}"
  ${OrIf} $InstDir == "$LOCALAPPDATA\Drøm"
  ${OrIf} $InstDir == "$LOCALAPPDATA\drøm"
  ${OrIf} $InstDir == "$LOCALAPPDATA\droem"
  ${OrIf} $InstDir == "$LOCALAPPDATA\Droem"
    StrCpy $InstDir "$PROGRAMFILES64\Droem"
  ${EndIf}
  ; Restore the output directory after normalizing $INSTDIR, as in the tested 38-D PathFix.
  ; Otherwise the main resources can be extracted under the old Unicode folder.
  SetOutPath "$INSTDIR"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  SetShellVarContext all

  ; 1. Desktop Shortcut Control:
  ;    By default on perMachine installs, do NOT leave a Public Desktop shortcut
  ;    unless the user explicitly passed /DESKTOP=1 on the installer command line.
  ${GetParameters} $R9
  ClearErrors
  ${GetOptions} $R9 "/DESKTOP=" $R8
  ${If} $R8 != "1"
    Delete "$DESKTOP\${PRODUCTNAME}.lnk"
    Delete "$DESKTOP\Drøm.lnk"
    Delete "$DESKTOP\drøm.lnk"
    Delete "$DESKTOP\Droem.lnk"
  ${EndIf}

  ; 2. Guard against incomplete extraction: only register associations if main binary exists in $INSTDIR
  IfFileExists "$INSTDIR\${MAINBINARYNAME}.exe" 0 droem_postinstall_skip_reg

  StrCpy $R2 "$INSTDIR\icons\icon.ico,0"
  IfFileExists "$INSTDIR\icons\icon.ico" +2 0
    StrCpy $R2 "$INSTDIR\${MAINBINARYNAME}.exe,0"

  ; 3. Register Drøm Application Capabilities & App Paths under SHCTX (HKLM) with ownership markers
  WriteRegStr SHCTX "Software\Droem" "DroemCreated" "1"
  WriteRegStr SHCTX "Software\Droem\Capabilities" "DroemCreated" "1"
  WriteRegStr SHCTX "Software\Droem\Capabilities" "ApplicationName" "Drøm"
  WriteRegStr SHCTX "Software\Droem\Capabilities" "ApplicationDescription" "Drøm — Native High-Resolution Music Player"
  WriteRegStr SHCTX "Software\Droem\Capabilities" "ApplicationIcon" "$R2"

  WriteRegStr SHCTX "Software\RegisteredApplications" "Drøm" "Software\Droem\Capabilities"

  ; Set FriendlyAppName & NoOpenWith on generic Applications\droem.exe so Windows Default Apps / Open With
  ; routes through per-format Capabilities\FileAssociations ProgIDs (`Droem.Audio.<ext>`)
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "DroemCreated" "1"
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "FriendlyAppName" "Drøm"
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "NoOpenWith" ""
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\DefaultIcon" "" "$R2"
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open" "" "Play with Drøm"
  WriteRegStr SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open\command" "" "$\"$INSTDIR\${MAINBINARYNAME}.exe$\" $\"%1$\""

  WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "DroemCreated" "1"
  WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "" "$INSTDIR\${MAINBINARYNAME}.exe"
  WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "Path" "$INSTDIR"

  ; 4. Register the 10 official v0.1.0 supported audio extensions around canonical ProgID (`Droem.Audio.<ext>`)
  !insertmacro DroemRegisterAudioExt "mp3"  "Drøm MP3 Audio"
  !insertmacro DroemRegisterAudioExt "flac" "Drøm FLAC Audio"
  !insertmacro DroemRegisterAudioExt "wav"  "Drøm WAVE Audio"
  !insertmacro DroemRegisterAudioExt "m4a"  "Drøm M4A Audio"
  !insertmacro DroemRegisterAudioExt "aac"  "Drøm AAC Audio"
  !insertmacro DroemRegisterAudioExt "ogg"  "Drøm OGG Vorbis Audio"
  !insertmacro DroemRegisterAudioExt "opus" "Drøm Opus Audio"
  !insertmacro DroemRegisterAudioExt "wma"  "Drøm Windows Media Audio"
  !insertmacro DroemRegisterAudioExt "aif"  "Drøm AIFF Audio"
  !insertmacro DroemRegisterAudioExt "aiff" "Drøm AIFF Audio"

  ; 5. Upgrade Cleanup: If upgrading from an earlier 13-format install, safely clean up ONLY Drøm-owned
  ;    registrations and residual icon files for excluded formats (`wv`, `ape`, `mpc`) without touching
  ;    third-party default apps, UserChoice, or other programs' OpenWithProgids.
  !insertmacro DroemUnregisterAudioExt "wv"
  !insertmacro DroemUnregisterAudioExt "ape"
  !insertmacro DroemUnregisterAudioExt "mpc"
  Delete "$INSTDIR\icons\audio_wv.ico"
  Delete "$INSTDIR\icons\audio_ape.ico"
  Delete "$INSTDIR\icons\audio_mpc.ico"

  ; 6. Notify Windows Shell of association update
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'

droem_postinstall_skip_reg:
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  SetShellVarContext all

  ; Unregister 10 official formats + 3 legacy excluded formats (if present) using ownership verification
  !insertmacro DroemUnregisterAudioExt "mp3"
  !insertmacro DroemUnregisterAudioExt "flac"
  !insertmacro DroemUnregisterAudioExt "wav"
  !insertmacro DroemUnregisterAudioExt "m4a"
  !insertmacro DroemUnregisterAudioExt "aac"
  !insertmacro DroemUnregisterAudioExt "ogg"
  !insertmacro DroemUnregisterAudioExt "opus"
  !insertmacro DroemUnregisterAudioExt "wma"
  !insertmacro DroemUnregisterAudioExt "aif"
  !insertmacro DroemUnregisterAudioExt "aiff"
  !insertmacro DroemUnregisterAudioExt "wv"
  !insertmacro DroemUnregisterAudioExt "ape"
  !insertmacro DroemUnregisterAudioExt "mpc"

  ; Remove RegisteredApplications entry ONLY if it points to Drøm Capabilities
  ClearErrors
  ReadRegStr $R0 SHCTX "Software\RegisteredApplications" "Drøm"
  ${If} $R0 == "Software\Droem\Capabilities"
    DeleteRegValue SHCTX "Software\RegisteredApplications" "Drøm"
  ${EndIf}

  ClearErrors
  ReadRegStr $R0 SHCTX "Software\RegisteredApplications" "Droem"
  ${If} $R0 == "Software\Droem\Capabilities"
    DeleteRegValue SHCTX "Software\RegisteredApplications" "Droem"
  ${EndIf}

  ; Clean up Software\Droem\Capabilities only if owned by Drøm
  ClearErrors
  ReadRegStr $R0 SHCTX "Software\Droem\Capabilities" "DroemCreated"
  ${If} $R0 == "1"
    DeleteRegKey /ifempty SHCTX "Software\Droem\Capabilities\FileAssociations"
    DeleteRegValue SHCTX "Software\Droem\Capabilities" "ApplicationName"
    DeleteRegValue SHCTX "Software\Droem\Capabilities" "ApplicationDescription"
    DeleteRegValue SHCTX "Software\Droem\Capabilities" "ApplicationIcon"
    DeleteRegValue SHCTX "Software\Droem\Capabilities" "DroemCreated"
    DeleteRegKey /ifempty SHCTX "Software\Droem\Capabilities"
    DeleteRegValue SHCTX "Software\Droem" "DroemCreated"
    DeleteRegKey /ifempty SHCTX "Software\Droem"
  ${EndIf}

  ; Clean up Applications\droem.exe only if owned by Drøm
  ClearErrors
  ReadRegStr $R0 SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "DroemCreated"
  ${If} $R0 == "1"
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\SupportedTypes"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open\command" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open\command"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell\open"
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\shell"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\DefaultIcon" ""
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe\DefaultIcon"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "FriendlyAppName"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "NoOpenWith"
    DeleteRegValue SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe" "DroemCreated"
    DeleteRegKey /ifempty SHCTX "Software\Classes\Applications\${MAINBINARYNAME}.exe"
  ${EndIf}

  ; Clean up App Paths\droem.exe only if owned by Drøm
  ClearErrors
  ReadRegStr $R0 SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "DroemCreated"
  ReadRegStr $R1 SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" ""
  ${If} $R0 == "1"
  ${OrIf} $R1 == "$INSTDIR\${MAINBINARYNAME}.exe"
    DeleteRegValue SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" ""
    DeleteRegValue SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "Path"
    DeleteRegValue SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe" "DroemCreated"
    DeleteRegKey /ifempty SHCTX "Software\Microsoft\Windows\CurrentVersion\App Paths\${MAINBINARYNAME}.exe"
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ${If} $DeleteAppDataCheckboxState == 1
    SetShellVarContext current
    RMDir /r "$LOCALAPPDATA\droem"
    Delete "$TEMP\droem_native_smtc_cover.jpg"
    SetShellVarContext all
  ${EndIf}
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
!macroend

!endif
