; =============================================================================
; Drøm v1.0.0 — Inno Setup 6.7.3 Installer Script (Final Release Build)
; Target Architecture: Windows x64 (perMachine: C:\Program Files\Droem)
; Publisher: so1fr1d
; =============================================================================

#define MyAppName "Drøm"
#define MyAppLegacySearchName "Drøm (Drom)"
#define MyAppAsciiName "Droem"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "so1fr1d"
#define MyAppExeName "droem.exe"
#define MyAppId "{{8E6A5D40-3B2F-4C91-9F1A-7D5E2C8B0100}"

; Standard Inno Setup 6.7.3 ships Default.isl + Korean/Japanese/German/French/Spanish/Russian.isl
; in compiler:Languages\. ChineseSimplified.isl is included when present in installer\inno\languages\.
#define HasChineseSimpLocalIsl FileExists(AddBackslash(SourcePath) + "languages\ChineseSimplified.isl")

[Setup]
AppId={#MyAppId}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
UninstallDisplayName={#MyAppName}
DefaultDirName={autopf}\{#MyAppAsciiName}
DefaultGroupName={#MyAppName}
DisableWelcomePage=no
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyMemo=yes
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
ChangesAssociations=yes
AllowNoIcons=yes
ShowLanguageDialog=yes
LanguageDetectionMethod=uilanguage
SetupLogging=yes
UninstallLogging=yes
CloseApplications=no
RestartApplications=no
OutputDir=..\..\dist-installer
OutputBaseFilename=droem-setup-{#MyAppVersion}
SetupIconFile=..\..\src-tauri\icons\icon.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
WizardImageFile=wizard.bmp
WizardSmallImageFile=wizard-small.bmp
WizardImageStretch=yes
VersionInfoVersion=1.0.0.0
VersionInfoProductVersion=1.0.0.0
VersionInfoProductTextVersion=1.0.0
VersionInfoProductName=Drøm
VersionInfoCompany=so1fr1d
VersionInfoDescription=Drøm Audio Player Setup

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
Name: "korean"; MessagesFile: "compiler:Languages\Korean.isl"; InfoBeforeFile: "RELEASE-NOTICE-ko.txt"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
#if HasChineseSimpLocalIsl
Name: "chinesesimplified"; MessagesFile: "languages\ChineseSimplified.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
#endif
Name: "german"; MessagesFile: "compiler:Languages\German.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"; InfoBeforeFile: "RELEASE-NOTICE-en.txt"

[Messages]
; Keep the message short. Real uninstall failures remain in the Inno log.
english.UninstalledMost=%1 uninstallation finished.
korean.UninstalledMost=%1 제거 작업을 마쳤습니다.
japanese.UninstalledMost=%1 のアンインストール処理が終了しました。
german.UninstalledMost=Deinstallation von %1 abgeschlossen.
french.UninstalledMost=Désinstallation de %1 terminée.
spanish.UninstalledMost=Desinstalación de %1 finalizada.
russian.UninstalledMost=Удаление %1 завершено.
#if HasChineseSimpLocalIsl
chinesesimplified.UninstalledMost=%1 卸载操作已结束。
#endif
english.WelcomeLabel1=Welcome to the Drøm Setup Wizard
english.WelcomeLabel2=This will install Drøm {#MyAppVersion} on your computer.%n%nIt is recommended that you close all other applications before continuing.
english.WizardInfoBefore=Credits & licenses
english.InfoBeforeLabel=Please review the credits and license summary for Drøm before continuing.
english.InfoBeforeClickLabel=When you are ready to continue with Setup, click Next.
english.FinishedHeadingLabel=Completing the Drøm Setup Wizard
english.FinishedLabelNoIcons=Setup has finished installing Drøm on your computer.
english.FinishedLabel=Setup has finished installing Drøm on your computer. The application may be launched by selecting the installed icons.

korean.WelcomeLabel1=Drøm 설치 마법사에 오신 것을 환영합니다
korean.WelcomeLabel2=컴퓨터에 Drøm {#MyAppVersion}을(를) 설치합니다.%n%n설치를 계속하기 전에 실행 중인 다른 모든 프로그램을 종료하는 것이 좋습니다.
korean.WizardInfoBefore=Credits & licenses (크레딧 및 라이선스 안내)
korean.InfoBeforeLabel=설치를 계속하기 전에 Drøm의 크레딧 및 라이선스 안내를 확인해 주십시오.
korean.InfoBeforeClickLabel=준비가 되었으면 [다음]을 클릭하여 설치를 계속하십시오.
korean.ClickNext=계속하려면 [다음]을 클릭하고, 설치 프로그램을 종료하려면 [취소]를 클릭하십시오.
korean.FinishedHeadingLabel=Drøm 설치 완료
korean.FinishedLabelNoIcons=컴퓨터에 Drøm 설치를 완료했습니다.
korean.FinishedLabel=컴퓨터에 Drøm 설치를 완료했습니다. 설치된 아이콘을 선택하여 프로그램을 실행할 수 있습니다.
korean.ClickFinish=설치 마법사를 종료하려면 [마침]을 클릭하십시오.

[CustomMessages]
english.AppSummary=Native high-resolution audio player (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
english.LaunchApp=Launch Drøm
english.OpenDefaultApps=Open Windows Default Apps settings
english.NsisConflictDetected=An existing NSIS installation of Drøm was detected on this system.%n%nTo prevent duplicate Uninstall entries or file conflicts, please uninstall the existing Drøm via Windows Settings > Apps > Installed apps before running this installer.
english.WebView2Installing=Checking and installing Microsoft Edge WebView2 Runtime...
english.WebView2DownloadFailed=Failed to download the Microsoft Edge WebView2 Runtime bootstrapper. Setup cannot continue because Drøm requires WebView2 Runtime. Please check your internet connection or install Microsoft Edge WebView2 Runtime manually.
english.WebView2InstallFailed=Microsoft Edge WebView2 Runtime installation failed or could not be verified. Setup cannot complete without WebView2 Runtime.

korean.AppSummary=고해상도 네이티브 오디오 플레이어 (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
korean.LaunchApp=Drøm 실행하기
korean.OpenDefaultApps=Windows 기본 앱 설정 열기
korean.NsisConflictDetected=기존 NSIS 방식으로 설치된 Drøm이 감지되었습니다.%n%n설치기 엔진 전환에 따른 중복 등록 및 파일 충돌을 방지하기 위해, 먼저 [Windows 설정 > 앱 > 설치된 앱]에서 기존 Drøm을 제거한 뒤 다시 실행해 주십시오.
korean.WebView2Installing=Microsoft Edge WebView2 런타임을 확인하고 설치하는 중입니다...
korean.WebView2DownloadFailed=Microsoft Edge WebView2 런타임 설치 파일 다운로드에 실패했습니다. Drøm 실행에는 WebView2 런타임이 필수이므로 설치를 중단합니다. 인터넷 연결을 확인하거나 WebView2 런타임을 수동 설치한 후 다시 시도해 주십시오.
korean.WebView2InstallFailed=Microsoft Edge WebView2 런타임 설치에 실패했거나 설치 결과를 확인할 수 없어 Drøm 설치를 중단했습니다.

japanese.AppSummary=ネイティブ高解像度オーディオプレーヤー (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
japanese.LaunchApp=Drøm を起動する
japanese.OpenDefaultApps=Windows の既定のアプリ設定を開く
japanese.NsisConflictDetected=既存の NSIS 版 Drøm が検出されました。%n%n競合を防ぐため、先に [Windows の設定 > アプリ > インストールされているアプリ] から既存の Drøm をアンインストールしてから再実行してください。
japanese.WebView2Installing=Microsoft Edge WebView2 ランタイムを確認・インストールしています...
japanese.WebView2DownloadFailed=Microsoft Edge WebView2 ランタイムのダウンロードに失敗したため、セットアップを中止しました。
japanese.WebView2InstallFailed=Microsoft Edge WebView2 ランタイムのインストールを確認できなかったため、セットアップを中止しました。

#if HasChineseSimpLocalIsl
chinesesimplified.AppSummary=原生高解析度音频播放器 (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
chinesesimplified.LaunchApp=启动 Drøm
chinesesimplified.OpenDefaultApps=打开 Windows 默认应用设置
chinesesimplified.NsisConflictDetected=检测到系统中已安装旧版 NSIS 安装包创建的 Drøm。%n%n为避免重复注册或文件冲突，请先在“Windows 设置 > 应用 > 已安装的应用”中卸载现有的 Drøm，然后再运行此安装程序。
chinesesimplified.WebView2Installing=正在检查并安装 Microsoft Edge WebView2 运行时...
chinesesimplified.WebView2DownloadFailed=下载 Microsoft Edge WebView2 运行时失败，安装已中止。
chinesesimplified.WebView2InstallFailed=未能安装或验证 Microsoft Edge WebView2 运行时，安装已中止。
#endif

german.AppSummary=Nativer Hi-Res-Audioplayer (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
german.LaunchApp=Drøm starten
german.OpenDefaultApps=Windows-Standard-Apps-Einstellungen öffnen
german.NsisConflictDetected=Eine bestehende NSIS-Installation von Drøm wurde erkannt.%n%nBitte deinstallieren Sie zuerst die vorhandene Drøm-Version über Windows-Einstellungen > Apps > Installierte Apps, bevor Sie dieses Setup ausführen.
german.WebView2Installing=Microsoft Edge WebView2 Runtime wird überprüft und installiert...
german.WebView2DownloadFailed=Download der Microsoft Edge WebView2 Runtime fehlgeschlagen. Das Setup wurde abgebrochen.
german.WebView2InstallFailed=Microsoft Edge WebView2 Runtime konnte nicht installiert oder verifiziert werden. Das Setup wurde abgebrochen.

french.AppSummary=Lecteur audio haute résolution natif (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
french.LaunchApp=Lancer Drøm
french.OpenDefaultApps=Ouvrir les paramètres des applications par défaut de Windows
french.NsisConflictDetected=Une installation NSIS existante de Drøm a été détectée.%n%nVeuillez d'abord désinstaller la version existante de Drøm via Paramètres Windows > Applications > Applications installées avant de relancer cet installateur.
french.WebView2Installing=Vérification et installation du runtime Microsoft Edge WebView2...
french.WebView2DownloadFailed=Échec du téléchargement du runtime Microsoft Edge WebView2. L'installation est annulée.
french.WebView2InstallFailed=Impossible d'installer ou de vérifier le runtime Microsoft Edge WebView2. L'installation est annulée.

spanish.AppSummary=Reproductor de audio nativo de alta resolución (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
spanish.LaunchApp=Iniciar Drøm
spanish.OpenDefaultApps=Abrir la configuración de Aplicaciones predeterminadas de Windows
spanish.NsisConflictDetected=Se detectó una instalación NSIS anterior de Drøm.%n%nPara evitar conflictos, desinstale primero la versión existente de Drøm desde Configuración de Windows > Aplicaciones > Aplicaciones instaladas antes de ejecutar este instalador.
spanish.WebView2Installing=Comprobando e instalando Microsoft Edge WebView2 Runtime...
spanish.WebView2DownloadFailed=Error al descargar Microsoft Edge WebView2 Runtime. La instalación se ha cancelado.
spanish.WebView2InstallFailed=No se pudo instalar o verificar Microsoft Edge WebView2 Runtime. La instalación se ha cancelado.

russian.AppSummary=Нативный аудиоплеер высокого разрешения (MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF)
russian.LaunchApp=Запустить Drøm
russian.OpenDefaultApps=Открыть параметры приложений по умолчанию Windows
russian.NsisConflictDetected=Обнаружена существующая установка Drøm (NSIS).%n%nВо избежание конфликтов сначала удалите текущую версию Drøm через «Параметры Windows > Приложения > Установленные приложения», а затем запустите этот установщик снова.
russian.WebView2Installing=Проверка и установка среды выполнения Microsoft Edge WebView2...
russian.WebView2DownloadFailed=Не удалось загрузить среду выполнения Microsoft Edge WebView2. Установка прервана.
russian.WebView2InstallFailed=Не удалось установить или проверить среду выполнения Microsoft Edge WebView2. Установка прервана.

[InstallDelete]
; Remove prior subfolder shortcuts created by test builds; keep one official-name shortcut.
Type: files; Name: "{autoprograms}\{#MyAppAsciiName}\{#MyAppName}.lnk"
Type: files; Name: "{autoprograms}\{#MyAppAsciiName}\{#MyAppLegacySearchName}.lnk"
Type: files; Name: "{autoprograms}\{#MyAppAsciiName}\Drøm (droem).lnk"
Type: dirifempty; Name: "{autoprograms}\{#MyAppAsciiName}"

[Files]
; Core Application Binary & Legal Resources (kept in {app} for installed reference)
Source: "Payload\droem.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "Payload\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "Payload\THIRD_PARTY_NOTICES.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "Payload\THIRD_PARTY_LICENSES.txt"; DestDir: "{app}"; Flags: ignoreversion

; Approved 10 Audio Format Icons + App Icons
Source: "Payload\icons\*"; DestDir: "{app}\icons"; Flags: ignoreversion recursesubdirs createallsubdirs

; 21 Multi-Language UI Locales
Source: "Payload\locales\*"; DestDir: "{app}\locales"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
; Single official-name shortcut. Desktop shortcut created by default (no extra Tasks page).
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icons\icon.ico"; AppUserModelID: "drøm.MusicPlayer"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icons\icon.ico"; AppUserModelID: "drøm.MusicPlayer"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchApp}"; Flags: nowait postinstall skipifsilent
Filename: "ms-settings:defaultapps"; Description: "{cm:OpenDefaultApps}"; Flags: shellexec nowait postinstall skipifsilent unchecked

[Code]
const
  SHCNE_ASSOCCHANGED   = $08000000;
  SHCNF_IDLIST         = $0000;
  WEBVIEW2_CLIENT_GUID = '{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}';

{ Native finish-page checkboxes: avoids left-edge clipping of Inno's RunList. }
var
  LaunchCheck, DefaultsCheck: TNewCheckBox;

procedure SHChangeNotify(wEventId: Integer; uFlags: Cardinal; dwItem1: Integer; dwItem2: Integer);
  external 'SHChangeNotify@shell32.dll stdcall';

function GetOfficialExtensionsCount(): Integer;
begin
  Result := 10;
end;

function GetOfficialExtension(Index: Integer): String;
begin
  case Index of
    0: Result := 'mp3';
    1: Result := 'flac';
    2: Result := 'wav';
    3: Result := 'm4a';
    4: Result := 'aac';
    5: Result := 'ogg';
    6: Result := 'opus';
    7: Result := 'wma';
    8: Result := 'aif';
    9: Result := 'aiff';
  else
    Result := '';
  end;
end;

function GetOfficialExtensionLabel(Ext: String): String;
begin
  if CompareText(Ext, 'wav') = 0 then
    Result := 'Drøm WAVE Audio'
  else if CompareText(Ext, 'ogg') = 0 then
    Result := 'Drøm OGG Vorbis Audio'
  else if CompareText(Ext, 'opus') = 0 then
    Result := 'Drøm Opus Audio'
  else if CompareText(Ext, 'wma') = 0 then
    Result := 'Drøm Windows Media Audio'
  else if (CompareText(Ext, 'aif') = 0) or (CompareText(Ext, 'aiff') = 0) then
    Result := 'Drøm AIFF Audio'
  else
    Result := 'Drøm ' + Uppercase(Ext) + ' Audio';
end;

function GetLegacyExcludedExtensionsCount(): Integer;
begin
  Result := 3;
end;

function GetLegacyExcludedExtension(Index: Integer): String;
begin
  case Index of
    0: Result := 'wv';
    1: Result := 'ape';
    2: Result := 'mpc';
  else
    Result := '';
  end;
end;

{ ---------------------------------------------------------------------------
  1. NSIS Conflict Guard
  --------------------------------------------------------------------------- }
function HasNsisUninstallKey(RootKey: Integer; const SubKey: String): Boolean;
var
  UninstallStr: String;
begin
  Result := False;
  if RegQueryStringValue(RootKey, SubKey, 'UninstallString', UninstallStr) then
  begin
    if (Pos('uninstall.exe', Lowercase(UninstallStr)) > 0) and
       (Pos('unins000.exe', Lowercase(UninstallStr)) = 0) then
    begin
      Result := True;
    end;
  end;
end;

function IsLegacyNsisDroemInstalled(): Boolean;
var
  ProgramFilesDir: String;
begin
  Result := False;
  ProgramFilesDir := ExpandConstant('{autopf}');

  if FileExists(ProgramFilesDir + '\Droem\uninstall.exe') or
     FileExists(ProgramFilesDir + '\Drøm\uninstall.exe') or
     FileExists(ProgramFilesDir + '\Drøm\droem.exe') then
  begin
    Result := True;
    Exit;
  end;

  if HasNsisUninstallKey(HKLM, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Drøm') or
     HasNsisUninstallKey(HKLM, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\droem') or
     HasNsisUninstallKey(HKLM, 'SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Drøm') or
     HasNsisUninstallKey(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Drøm') or
     HasNsisUninstallKey(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\droem') then
  begin
    Result := True;
    Exit;
  end;
end;

{ ---------------------------------------------------------------------------
  2. WebView2 Runtime Detection & Strict Pre-Install Enforcement
  --------------------------------------------------------------------------- }
function IsWebView2Installed(): Boolean;
var
  Pv: String;
begin
  Result := False;
  if RegQueryStringValue(HKLM, 'SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\' + WEBVIEW2_CLIENT_GUID, 'pv', Pv) then
    if (Pv <> '') and (Pv <> '0.0.0.0') then
    begin
      Result := True;
      Exit;
    end;

  if RegQueryStringValue(HKLM, 'SOFTWARE\Microsoft\EdgeUpdate\Clients\' + WEBVIEW2_CLIENT_GUID, 'pv', Pv) then
    if (Pv <> '') and (Pv <> '0.0.0.0') then
    begin
      Result := True;
      Exit;
    end;

  if RegQueryStringValue(HKCU, 'Software\Microsoft\EdgeUpdate\Clients\' + WEBVIEW2_CLIENT_GUID, 'pv', Pv) then
    if (Pv <> '') and (Pv <> '0.0.0.0') then
    begin
      Result := True;
      Exit;
    end;
end;

function EnsureWebView2BeforeInstall(): String;
var
  BootstrapperPath: String;
  PsCmd: String;
  DownloadExitCode: Integer;
  InstallExitCode: Integer;
begin
  Result := '';
  if IsWebView2Installed() then
    Exit;

  Log('WebView2 Runtime not detected. Downloading Microsoft Edge WebView2 Evergreen Bootstrapper...');
  BootstrapperPath := ExpandConstant('{tmp}\MicrosoftEdgeWebview2Setup.exe');
  if FileExists(BootstrapperPath) then
    DeleteFile(BootstrapperPath);

  PsCmd := '-NoProfile -ExecutionPolicy Bypass -Command "try { [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri ''https://go.microsoft.com/fwlink/p/?LinkId=2124703'' -OutFile ' + #39 + BootstrapperPath + #39 + ' -UseBasicParsing -ErrorAction Stop; exit 0 } catch { exit 1 }"';
  if (not Exec(ExpandConstant('{sys}\WindowsPowerShell\v1.0\powershell.exe'), PsCmd, '', SW_HIDE, ewWaitUntilTerminated, DownloadExitCode)) or
     (DownloadExitCode <> 0) or
     (not FileExists(BootstrapperPath)) then
  begin
    Log('WebView2 bootstrapper download failed. ExitCode=' + IntToStr(DownloadExitCode));
    Result := CustomMessage('WebView2DownloadFailed') + ' (Code: ' + IntToStr(DownloadExitCode) + ')';
    Exit;
  end;

  Log('Executing WebView2 Evergreen Bootstrapper (/silent /install)...');
  if (not Exec(BootstrapperPath, '/silent /install', '', SW_HIDE, ewWaitUntilTerminated, InstallExitCode)) or
     (InstallExitCode <> 0) then
  begin
    Log('WebView2 bootstrapper execution failed. ExitCode=' + IntToStr(InstallExitCode));
    Result := CustomMessage('WebView2InstallFailed') + ' (Code: ' + IntToStr(InstallExitCode) + ')';
    Exit;
  end;

  if not IsWebView2Installed() then
  begin
    Log('WebView2 registry verification failed after running bootstrapper.');
    Result := CustomMessage('WebView2InstallFailed');
    Exit;
  end;

  Log('WebView2 Runtime successfully installed and verified.');
end;

{ ---------------------------------------------------------------------------
  3. Ownership Check & Value-Level Safe Cleanup Helpers
  --------------------------------------------------------------------------- }
function IsDroemOwnedProgId(RootKey: Integer; const ProgIdKey: String): Boolean;
var
  Marker, CmdVal, ExpectedCmdQuoted, ExpectedCmdUnquoted, LegacyNsisCmd: String;
begin
  Result := False;
  if RegQueryStringValue(RootKey, ProgIdKey, 'DroemCreated', Marker) then
    if Marker = '1' then
    begin
      Result := True;
      Exit;
    end;

  if RegQueryStringValue(RootKey, ProgIdKey + '\shell\open\command', '', CmdVal) then
  begin
    ExpectedCmdQuoted   := '"' + ExpandConstant('{app}\{#MyAppExeName}') + '" "%1"';
    ExpectedCmdUnquoted := ExpandConstant('{app}\{#MyAppExeName}') + ' "%1"';
    LegacyNsisCmd       := '"' + ExpandConstant('{autopf}\Drøm\{#MyAppExeName}') + '" "%1"';
    if (CompareText(CmdVal, ExpectedCmdQuoted) = 0) or
       (CompareText(CmdVal, ExpectedCmdUnquoted) = 0) or
       (CompareText(CmdVal, LegacyNsisCmd) = 0) then
    begin
      Result := True;
      Exit;
    end;
  end;
end;

procedure CleanupSingleExtensionInRoot(RootKey: Integer; const Ext: String; RemoveCandidateValues: Boolean);
var
  ProgId, ProgIdKey, OpenWithKey: String;
begin
  ProgId      := 'Droem.Audio.' + Ext;
  ProgIdKey   := 'Software\Classes\' + ProgId;
  OpenWithKey := 'Software\Classes\.' + Ext + '\OpenWithProgids';

  { 1. Remove ONLY Drøm's own value from shared OpenWithProgids, SupportedTypes, and Capabilities }
  if RemoveCandidateValues then
  begin
    RegDeleteValue(RootKey, OpenWithKey, ProgId);
    RegDeleteKeyIfEmpty(RootKey, OpenWithKey);
    RegDeleteValue(RootKey, 'Software\Classes\Applications\{#MyAppExeName}\SupportedTypes', '.' + Ext);
    RegDeleteValue(RootKey, 'Software\Droem\Capabilities\FileAssociations', '.' + Ext);
  end;

  { 2. Verify Drøm ownership before removing individual values and empty subkeys }
  if RegKeyExists(RootKey, ProgIdKey) and IsDroemOwnedProgId(RootKey, ProgIdKey) then
  begin
    RegDeleteValue(RootKey, ProgIdKey + '\shell\open\command', '');
    RegDeleteKeyIfEmpty(RootKey, ProgIdKey + '\shell\open\command');

    RegDeleteValue(RootKey, ProgIdKey + '\shell\open', '');
    RegDeleteValue(RootKey, ProgIdKey + '\shell\open', 'FriendlyAppName');
    RegDeleteValue(RootKey, ProgIdKey + '\shell\open', 'Icon');
    RegDeleteKeyIfEmpty(RootKey, ProgIdKey + '\shell\open');

    RegDeleteValue(RootKey, ProgIdKey + '\shell', '');
    RegDeleteKeyIfEmpty(RootKey, ProgIdKey + '\shell');

    RegDeleteValue(RootKey, ProgIdKey + '\DefaultIcon', '');
    RegDeleteKeyIfEmpty(RootKey, ProgIdKey + '\DefaultIcon');

    RegDeleteValue(RootKey, ProgIdKey, '');
    RegDeleteValue(RootKey, ProgIdKey, 'FriendlyTypeName');
    RegDeleteValue(RootKey, ProgIdKey, 'DroemCreated');
    RegDeleteKeyIfEmpty(RootKey, ProgIdKey);
  end;
end;

procedure CleanupStaleHkcuDroemRoot();
var
  I: Integer;
  Ext, Marker, RegAppVal: String;
begin
  { Ensure no stale per-user HKCU Capabilities or NoOpenWith shadows HKLM for MP3/FLAC }
  for I := 0 to GetOfficialExtensionsCount() - 1 do
  begin
    Ext := GetOfficialExtension(I);
    CleanupSingleExtensionInRoot(HKCU, Ext, True);
  end;

  RegDeleteValue(HKCU, 'Software\Classes\Applications\{#MyAppExeName}', 'NoOpenWith');

  if RegQueryStringValue(HKCU, 'Software\RegisteredApplications', '{#MyAppName}', RegAppVal) then
    if CompareText(RegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKCU, 'Software\RegisteredApplications', '{#MyAppName}');

  if RegQueryStringValue(HKCU, 'Software\RegisteredApplications', '{#MyAppAsciiName}', RegAppVal) then
    if CompareText(RegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKCU, 'Software\RegisteredApplications', '{#MyAppAsciiName}');

  if RegQueryStringValue(HKCU, 'Software\Droem\Capabilities', 'DroemCreated', Marker) and (Marker = '1') then
  begin
    RegDeleteKeyIfEmpty(HKCU, 'Software\Droem\Capabilities\FileAssociations');
    RegDeleteValue(HKCU, 'Software\Droem\Capabilities', 'ApplicationName');
    RegDeleteValue(HKCU, 'Software\Droem\Capabilities', 'ApplicationDescription');
    RegDeleteValue(HKCU, 'Software\Droem\Capabilities', 'ApplicationIcon');
    RegDeleteValue(HKCU, 'Software\Droem\Capabilities', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKCU, 'Software\Droem\Capabilities');
    RegDeleteValue(HKCU, 'Software\Droem', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKCU, 'Software\Droem');
  end;
end;

procedure CleanupLegacyExcludedFormats();
var
  I: Integer;
  Ext, LegacyIconPath: String;
begin
  for I := 0 to GetLegacyExcludedExtensionsCount() - 1 do
  begin
    Ext := GetLegacyExcludedExtension(I);
    CleanupSingleExtensionInRoot(HKLM, Ext, True);
    CleanupSingleExtensionInRoot(HKCU, Ext, True);

    LegacyIconPath := ExpandConstant('{app}\icons\audio_') + Ext + '.ico';
    if FileExists(LegacyIconPath) then
      DeleteFile(LegacyIconPath);
  end;
end;

{ ---------------------------------------------------------------------------
  4. Register Official 10 Audio Formats (OpenWithProgids / Default Apps only)
     - NEVER modifies Software\Classes\.<ext> default value or UserChoice.
     - Removes NoOpenWith from Applications\droem.exe so Windows 11 Default Apps
       and OpenWith enumeration never suppress MP3/FLAC or any of the 10 formats.
  --------------------------------------------------------------------------- }
procedure RegisterDroemAssociations();
var
  I: Integer;
  Ext, ProgId, ProgIdKey, DescText, AppExePath, AppIconSpec, ExtIconFile, ExtIconSpec, OpenCmd, OldRegAppVal: String;
begin
  AppExePath := ExpandConstant('{app}\{#MyAppExeName}');
  if not FileExists(AppExePath) then
  begin
    Log('RegisterDroemAssociations skipped because droem.exe is missing at: ' + AppExePath);
    Exit;
  end;

  OpenCmd := '"' + AppExePath + '" "%1"';

  if FileExists(ExpandConstant('{app}\icons\icon.ico')) then
    AppIconSpec := ExpandConstant('{app}\icons\icon.ico') + ',0'
  else
    AppIconSpec := AppExePath + ',0';

  { Clear any stale HKCU shadow entries or lingering NoOpenWith flags first }
  CleanupStaleHkcuDroemRoot();

  { Application root & Capabilities }
  RegWriteStringValue(HKLM, 'Software\Droem', 'DroemCreated', '1');
  RegWriteStringValue(HKLM, 'Software\Droem\Capabilities', 'DroemCreated', '1');
  RegWriteStringValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationName', '{#MyAppName}');
  RegWriteStringValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationDescription', 'Drøm — Native High-Resolution Music Player');
  RegWriteStringValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationIcon', AppIconSpec);
  { One searchable Windows Default Apps entry. Retire earlier display-name registration
    only when it points to this same Drøm-owned capabilities key. }
  RegWriteStringValue(HKLM, 'Software\RegisteredApplications', '{#MyAppName}', 'Software\Droem\Capabilities');
  if RegQueryStringValue(HKLM, 'Software\RegisteredApplications', '{#MyAppLegacySearchName}', OldRegAppVal) then
    if CompareText(OldRegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKLM, 'Software\RegisteredApplications', '{#MyAppLegacySearchName}');

  { Applications\droem.exe & App Paths — explicitly ensure NoOpenWith is ABSENT }
  RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'NoOpenWith');
  RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'DroemCreated', '1');
  RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'FriendlyAppName', '{#MyAppName}');
  RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\DefaultIcon', '', AppIconSpec);
  RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open', '', 'Play with Drøm');
  RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open\command', '', OpenCmd);

  RegWriteStringValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', 'DroemCreated', '1');
  RegWriteStringValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', '', AppExePath);
  RegWriteStringValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', 'Path', ExpandConstant('{app}'));

  { Per-extension ProgID & non-destructive OpenWithProgids / Capabilities registration }
  for I := 0 to GetOfficialExtensionsCount() - 1 do
  begin
    Ext         := GetOfficialExtension(I);
    DescText    := GetOfficialExtensionLabel(Ext);
    ProgId      := 'Droem.Audio.' + Ext;
    ProgIdKey   := 'Software\Classes\' + ProgId;
    ExtIconFile := ExpandConstant('{app}\icons\audio_') + Ext + '.ico';

    if FileExists(ExtIconFile) then
      ExtIconSpec := ExtIconFile + ',0'
    else
      ExtIconSpec := AppExePath + ',0';

    RegWriteStringValue(HKLM, ProgIdKey, '', DescText);
    RegWriteStringValue(HKLM, ProgIdKey, 'FriendlyTypeName', DescText);
    RegWriteStringValue(HKLM, ProgIdKey, 'DroemCreated', '1');
    RegWriteStringValue(HKLM, ProgIdKey + '\DefaultIcon', '', ExtIconSpec);
    RegWriteStringValue(HKLM, ProgIdKey + '\shell', '', 'open');
    RegWriteStringValue(HKLM, ProgIdKey + '\shell\open', '', 'Play with Drøm');
    RegWriteStringValue(HKLM, ProgIdKey + '\shell\open', 'FriendlyAppName', '{#MyAppName}');
    RegWriteStringValue(HKLM, ProgIdKey + '\shell\open', 'Icon', ExtIconSpec);
    RegWriteStringValue(HKLM, ProgIdKey + '\shell\open\command', '', OpenCmd);

    { Register as candidate only — never overwrite Software\Classes\.<ext> default value }
    RegWriteStringValue(HKLM, 'Software\Classes\.' + Ext + '\OpenWithProgids', ProgId, '');
    RegWriteStringValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\SupportedTypes', '.' + Ext, '');
    RegWriteStringValue(HKLM, 'Software\Droem\Capabilities\FileAssociations', '.' + Ext, ProgId);
  end;

  { Safely clean up any legacy WV/APE/MPC Drøm registrations from earlier builds }
  CleanupLegacyExcludedFormats();

  SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, 0, 0);
end;

{ ---------------------------------------------------------------------------
  5. Ownership-Verified Uninstall Cleanup
  --------------------------------------------------------------------------- }
procedure UnregisterDroemAssociations();
var
  I: Integer;
  Ext, Marker, RegAppVal, AppPathDefault: String;
begin
  for I := 0 to GetOfficialExtensionsCount() - 1 do
  begin
    Ext := GetOfficialExtension(I);
    CleanupSingleExtensionInRoot(HKLM, Ext, True);
  end;

  CleanupStaleHkcuDroemRoot();
  CleanupLegacyExcludedFormats();

  if RegQueryStringValue(HKLM, 'Software\RegisteredApplications', '{#MyAppLegacySearchName}', RegAppVal) then
    if CompareText(RegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKLM, 'Software\RegisteredApplications', '{#MyAppLegacySearchName}');

  if RegQueryStringValue(HKLM, 'Software\RegisteredApplications', '{#MyAppName}', RegAppVal) then
    if CompareText(RegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKLM, 'Software\RegisteredApplications', '{#MyAppName}');

  if RegQueryStringValue(HKLM, 'Software\RegisteredApplications', '{#MyAppAsciiName}', RegAppVal) then
    if CompareText(RegAppVal, 'Software\Droem\Capabilities') = 0 then
      RegDeleteValue(HKLM, 'Software\RegisteredApplications', '{#MyAppAsciiName}');

  if RegQueryStringValue(HKLM, 'Software\Droem\Capabilities', 'DroemCreated', Marker) and (Marker = '1') then
  begin
    RegDeleteKeyIfEmpty(HKLM, 'Software\Droem\Capabilities\FileAssociations');
    RegDeleteValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationName');
    RegDeleteValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationDescription');
    RegDeleteValue(HKLM, 'Software\Droem\Capabilities', 'ApplicationIcon');
    RegDeleteValue(HKLM, 'Software\Droem\Capabilities', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Droem\Capabilities');
    RegDeleteValue(HKLM, 'Software\Droem', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Droem');
  end;

  if IsDroemOwnedProgId(HKLM, 'Software\Classes\Applications\{#MyAppExeName}') then
  begin
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\SupportedTypes');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open\command', '');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open\command');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open', '');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell\open');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\shell');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\DefaultIcon', '');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}\DefaultIcon');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'FriendlyAppName');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'NoOpenWith');
    RegDeleteValue(HKLM, 'Software\Classes\Applications\{#MyAppExeName}', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Classes\Applications\{#MyAppExeName}');
  end;

  if (RegQueryStringValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', 'DroemCreated', Marker) and (Marker = '1')) or
     (RegQueryStringValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', '', AppPathDefault) and
      (CompareText(AppPathDefault, ExpandConstant('{app}\{#MyAppExeName}')) = 0)) then
  begin
    RegDeleteValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', '');
    RegDeleteValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', 'Path');
    RegDeleteValue(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}', 'DroemCreated');
    RegDeleteKeyIfEmpty(HKLM, 'Software\Microsoft\Windows\CurrentVersion\App Paths\{#MyAppExeName}');
  end;

  SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, 0, 0);
end;

{ ---------------------------------------------------------------------------
  6. Wizard Lifecycle Hooks
  --------------------------------------------------------------------------- }
function InitializeSetup(): Boolean;
begin
  if IsLegacyNsisDroemInstalled() then
  begin
    if not WizardSilent() then
      MsgBox(CustomMessage('NsisConflictDetected'), mbError, MB_OK);
    Result := False;
    Exit;
  end;
  Result := True;
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  NeedsRestart := False;
  Result := EnsureWebView2BeforeInstall();
end;


{ Keep Inno's original [Run] selection flags as the source of truth. }
procedure FinishChoiceChanged(Sender: TObject);
begin
  WizardForm.RunList.Checked[0] := LaunchCheck.Checked;
  WizardForm.RunList.Checked[1] := DefaultsCheck.Checked;
end;

procedure CurPageChanged(CurPageID: Integer);
var
  H, LeftPadding: Integer;
begin
  if (CurPageID = wpFinished) and (not WizardSilent()) then
  begin
    if LaunchCheck = nil then
    begin
      LaunchCheck := TNewCheckBox.Create(WizardForm);
      DefaultsCheck := TNewCheckBox.Create(WizardForm);
      LaunchCheck.Parent := WizardForm.RunList.Parent;
      DefaultsCheck.Parent := WizardForm.RunList.Parent;
      LaunchCheck.Font := WizardForm.RunList.Font;
      DefaultsCheck.Font := WizardForm.RunList.Font;
      LaunchCheck.Caption := CustomMessage('LaunchApp');
      DefaultsCheck.Caption := CustomMessage('OpenDefaultApps');
      LaunchCheck.OnClick := @FinishChoiceChanged;
      DefaultsCheck.OnClick := @FinishChoiceChanged;
    end;

    H := ScaleY(28);
    LeftPadding := ScaleX(6);
    LaunchCheck.SetBounds(
      WizardForm.RunList.Left + LeftPadding,
      WizardForm.RunList.Top,
      WizardForm.RunList.Width - LeftPadding,
      H);
    DefaultsCheck.SetBounds(
      LaunchCheck.Left,
      LaunchCheck.Top + H,
      LaunchCheck.Width,
      H);

    LaunchCheck.Checked := WizardForm.RunList.Checked[0];
    DefaultsCheck.Checked := WizardForm.RunList.Checked[1];
    WizardForm.RunList.Visible := False;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    if not IsWebView2Installed() then
      RaiseException(CustomMessage('WebView2InstallFailed'));
    RegisterDroemAssociations();
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    UnregisterDroemAssociations();
  end;
end;
