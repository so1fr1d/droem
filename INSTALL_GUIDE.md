# Drøm v1.0.0 — Installation guide

## Requirements

- **Platform:** Windows 11 x64 (tested). Windows 10 x64 is a supported installation target but was **not physically verified** for this release.
- **Runtime:** Microsoft Edge WebView2 Runtime. The setup program checks for it and, if needed, attempts to obtain and install it. This step may require an internet connection.
- **Installer:** Inno Setup 6.7.3, per-machine installation with administrator permission.

## Installing Drøm

1. Download `droem-setup-1.0.0.exe` and `SHA256SUMS.txt` from the [v1.0.0 release](https://github.com/so1fr1d/droem/releases).
2. Verify the downloaded installer's checksum against the checksum file.
3. Run the installer and follow the setup wizard.
4. Start Drøm from the **Drøm** shortcut on your desktop or in the Start menu.

Default installation directory: `C:\Program Files\Droem`.

**Important:** The application and installer are currently **unsigned**. Keep Windows security protections enabled.

If a legacy **NSIS-based** Drøm test installation is detected, uninstall that older installation through **Windows Settings → Apps → Installed apps** before continuing with this Inno Setup installer.

## Supported formats and default apps

Drøm registers the following 10 audio-file extensions:

`.mp3`, `.flac`, `.wav`, `.m4a`, `.aac`, `.ogg`, `.opus`, `.wma`, `.aif`, `.aiff`.

The installer does **not** take over your current default app selections. To set up file associations:

1. Open **Windows Settings → Apps → Default apps**.
2. Find **Drøm** and assign it to the extensions you want.
3. You can also use Drøm's shortcut to open Windows default-app settings.

Previously chosen default apps are intended to be preserved during installation and uninstall. Windows may retain a user's own default-app choices across reinstalls.

The unsupported extensions `.wv`, `.ape` and `.mpc` are **not** included in Drøm's official 10-format support policy.

## Languages

The player offers **21 selectable interface languages** plus automatic system-language detection. The installer has a separate, smaller language selection. Not every interface translation has been reviewed by a native speaker.

## Uninstall

Open **Windows Settings → Apps → Installed apps**, find **Drøm**, and choose **Uninstall**.

Your personal music files are not part of the installation and should not be deleted by uninstalling Drøm.

## Licenses

The project's source code is licensed under [MIT-0](LICENSE). Review [third-party notices](THIRD_PARTY_NOTICES.md) and [third-party license information](THIRD_PARTY_LICENSES.txt) for component-specific terms.
