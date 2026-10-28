<div align="center">

<img src="assets/hero-logo-player.jpg" alt="Drøm — a calm, local music player for Windows" width="900">

[**Downloads**](https://github.com/so1fr1d/droem/releases) · [**Features**](#features) · [**Installation**](#installation)

</div>

## Meet Drøm

Drøm is a local music player for Windows, built around your own collection. Keep your library, playback queue and listening controls close — or switch to a smaller player when you need the space.

No streaming account required. Your music stays local.

<p align="center">
  <img src="assets/full-dark.jpg" alt="Drøm in dark mode, showing the library, floating left-side controls, playback queue and player bar" width="900">
</p>

## Three window modes

Go from a complete music library to an album-focused player, then to a little squircle that stays out of your way.

<p align="center">
  <img src="assets/three-modes.jpg" alt="Drøm's three window layouts side by side: Full, Compact and Mini" width="900">
</p>

## Colors, your way

Choose **light or dark**, pick a custom accent color, or turn on **Dynamic Cover Color** to let your album artwork influence the interface.

<p align="center">
  <img src="assets/color-parts.jpg" alt="Four accent colors in Drøm: amber, rose, slate and sky blue" width="900">
</p>

<p align="center">
  <img src="assets/full-light.webp" alt="The full library in Drøm's light theme" width="440">
  <img src="assets/full-dynamic.jpg" alt="Drøm's album-reactive theme on a colorful Windows desktop" width="440">
</p>

## Features

- **Left island controls.** A floating control strip beside your player.
- **Three window modes.** Full, Compact and Mini Squircle, each designed for a different way of listening.
- **Personal themes.** Light, dark, custom accent colors and album-reactive color.
- **10 audio formats.** MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF and AIFF.
- **Library and playback queue.** Browse your collection, manage the play queue and return to the library after playing external files.
- **Windows media controls (SMTC).** See the current track and control playback from Windows.
- **21 interface languages.** Choose your language or use automatic system-language detection.

<details>
<summary><strong>A look at the settings</strong></summary>

<p align="center">
  <img src="assets/settings.jpg" alt="Drøm's settings: appearance, theme, audio output, interface language and Windows file associations" width="600">
</p>

</details>

### Windows integration

Control Drøm from the Windows media panel without switching back to the player.

<p align="center">
  <img src="assets/smtc-indicate.jpg" alt="Drøm track details and playback controls shown in the Windows 11 media panel" width="350">
</p>

## Installation

Download **`droem-setup-1.0.0.exe`** from the [Releases page](https://github.com/so1fr1d/droem/releases).

**Windows 11 x64 is tested.** Windows 10 x64 has not been verified on a physical machine for this release.

The installer checks for **Microsoft Edge WebView2 Runtime** and may need an internet connection to obtain it if it is missing. Playing local music does not require a streaming account.

Drøm does **not** overwrite existing Windows default audio-app choices. To associate files with Drøm, open **Windows Settings → Apps → Default apps** and select the extensions you want.

See the [installation guide](INSTALL_GUIDE.md) for details.

## Notes

- **Unsigned installer.** Verify its SHA-256 against `SHA256SUMS.txt` in the release, and keep Windows security protections enabled.
- Windows Settings may not find Drøm when searching for the ASCII spelling `dro`. Searching for `drø` may work instead.
- All 21 interface languages passed automated translation-key and placeholder checks; not every translation has received native-speaker review.
- Playback support for certain WMA variants can depend on codecs; DRM-protected audio may not play.

## Credits & licenses

Made by **so1fr1d**.

Drøm's interface design draws inspiration from [AIMP](https://aimp.su/) and the AIMP custom skins **Mei** and **Wyn**. Drøm is an independent project, not affiliated with AIMP or the skin creators.

Wallpaper featured in selected screenshots: **[Johannes Plenio — Gray and White Wallpaper, Pexels](https://www.pexels.com/photo/gray-and-white-wallpaper-1103970/)**.

Drøm is open source under the [MIT No Attribution (MIT-0) license](LICENSE). Third-party rights remain separate; see the [notices](THIRD_PARTY_NOTICES.md) and [license information](THIRD_PARTY_LICENSES.txt).
