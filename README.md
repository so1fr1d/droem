<div align="center">

# Drøm

**A calm, local music player for Windows, built around your own music.**

[**Releases & downloads**](https://github.com/so1fr1d/droem/releases) · [Features](#features) · [Installation](#installation)

</div>

## Meet Drøm

Drøm brings your local music library, playback queue and listening space together in one flexible desktop player. Browse your collection in a full library window, move to a compact cover or list view, or keep a mini player nearby.

No streaming account required — your music stays local.

## Features

- **Left island controls** — a floating control strip alongside the main player.
- **Three window modes** — switch between **Full**, **Compact** and **Mini Squircle** to suit your workspace.
- **Light, dark and album-reactive themes** — choose a theme and let album artwork shape the accent colors.
- **10 audio formats** — MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF and AIFF.
- **Library, queue and playback lists** — browse folders, manage what's playing and return to your library after opening music externally.
- **Windows media controls (SMTC)** — view track information and control playback from Windows.
- **21 interface languages** — choose your language or use system-language detection.

## Installation

Download **`droem-setup-1.0.0.exe`** from the [Releases page](https://github.com/so1fr1d/droem/releases).

**Windows 11 x64 is the tested platform.** Windows 10 x64 has not been verified on a physical machine for this release.

Drøm requires Microsoft Edge WebView2 Runtime. The installer checks for it and may need an internet connection to install it if missing. Once installed, Drøm plays local music without a streaming account.

The installer does **not** override existing default audio-app choices. To associate supported file formats with Drøm, open **Windows Settings → Apps → Default apps** and select Drøm for the extensions you want.

See the [installation guide](INSTALL_GUIDE.md) for more information.

## Notes

- **Unsigned installer:** check the installer against `SHA256SUMS.txt` from the release, and keep Windows security protections enabled.
- Windows Settings may not find Drøm when searching for the ASCII spelling `dro`. Searching for `Drø` may work instead.
- All 21 selectable languages passed automated translation-key and placeholder checks; not every language has received native-speaker review.
- Some WMA variants and DRM-protected audio files may not play, depending on the codec and file.

## Credits & licenses

Made by **so1fr1d**.

Drøm is open source under the [MIT No Attribution (MIT-0) license](LICENSE). Third-party rights remain separate; see the [notices](THIRD_PARTY_NOTICES.md) and [license information](THIRD_PARTY_LICENSES.txt).
