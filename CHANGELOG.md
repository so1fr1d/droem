# Changelog

## [v0.1.0 — Phase 38-F Release Finalization] — 2026-10-23

### 1. 공식 지원 오디오 포맷 10종 확정 및 파일 연결 정리
- **공식 지원 10종 확정 (`MP3, FLAC, WAV, M4A, AAC, OGG, OPUS, WMA, AIF, AIFF`)**:
  - `WV`, `APE`, `MPC`를 v0.1.0 공식 지원 목록, 파일 열기 대화상자 필터(`main.rs`, `index.html`), 번들 아이콘 리소스(`tauri.conf.json`), NSIS 파일 연결(`hooks.nsh`), 문서(`README.md`, `INSTALL_GUIDE.md`)에서 제외했습니다.
  - 이전 13종 설치본 위에서 업데이트할 때 `NSIS_HOOK_POSTINSTALL`이 Drøm 소유(`DroemCreated="1"`)가 확인된 `wv`, `ape`, `mpc` 등록 값(`OpenWithProgids`, `SupportedTypes`, `Capabilities\FileAssociations`, `Droem.Audio.<ext>`)만 안전하게 정리하며, 타사 기본 앱·`UserChoice`·타 프로그램의 `OpenWithProgids`는 일절 건드리지 않습니다.
  - 사용자가 `.wv`, `.ape`, `.mpc` 파일을 직접 드래그하거나 실행 인자로 전달한 경우 앱이 충돌하지 않고 미지원 포맷 토스트 안내 후 다음 곡으로 안전하게 건너뜁니다.
  - 기존 WMA 재생 구현(`MF_SOURCE_READER_FIRST_AUDIO_STREAM = 0xFFFF_FFFD`, `MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED = 0x20`, `IMFSample::ConvertToContiguousBuffer` vtable slot `41`, RAII `MfComSessionGuard`, 원자적 WAV 캐시 잠금)은 그대로 보존했습니다.

### 2. 오디오 파일 아이콘 최종 색상 확정 (`OGG` → `#6878B4`)
- 기존 승인된 통일 조형(초승달 + 원형 별점 1개 + 2줄 웨이브)과 9종(`MP3, AAC, FLAC, WAV #7A9299, M4A, OPUS, WMA, AIF, AIFF`)의 색상은 그대로 유지하고, **`OGG`(`audio_ogg.ico`)만 Deep Indigo Slate(`#6878B4`, 테두리 `#48568A`)** 로 변경했습니다.
- `audio_wv.ico`, `audio_ape.ico`, `audio_mpc.ico`는 설치본 리소스(`bundle.resources`)에서 제외했습니다.

### 3. About 및 오픈소스 라이선스 마감
- 설정 하단 About 구성을 확정 사양(`drøm · v0.1.0` / `drøm — Version 0.1.0 · Windows x64`, `Developer: so1fr1d`, `License: MIT-0 · Open Source`, `GitHub: github.com/so1fr1d/droem`, `Third-party licenses: View notices`, `© 2026 so1fr1d`)으로 유지했습니다.
- NSIS 설치본에서도 `About → View notices` 클릭 시 `THIRD_PARTY_NOTICES.md`가 즉시 열리도록 리소스 번들 경로 및 내장 폴백(`EMBEDDED_THIRD_PARTY_NOTICES`)을 동기화했습니다.

### 4. NSIS 설치기 좌측 브랜드 패널(`nsis-sidebar.bmp`) 연결 확정
- 기존 승인된 `src-tauri/windows/nsis-sidebar.bmp` (`164×314`, 24-bit 비압축 BMP)를 그대로 재사용하고 `tauri.conf.json`의 `bundle.windows.nsis.sidebarImage`를 `"windows/nsis-sidebar.bmp"`로 지정하여 NSIS 시작(Welcome)·완료(Finish) 페이지에 연결되도록 설정했습니다.
