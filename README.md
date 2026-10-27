# drøm

Windows 10 / 11 (x64) 전용 네이티브 데스크톱 오디오 플레이어입니다.  
Tauri v2, Rust, WebView2 기반으로 동작하며 원격 분석·광고·계정 연동 없이 로컬 음원 감상에만 집중합니다.

![Version](https://img.shields.io/badge/version-0.1.0-38bdf8)
![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11%20x64-1e293b)
![License](https://img.shields.io/badge/license-MIT--0-10b981)

---

## 주요 특징

- **3단계 창 모드 전환 (`Alt+M`)**
  - **1차 창 (Full Mode)**: 폴더 트리 · 라이브러리 테이블 · 앨범/아티스트 그리드 뷰, 실시간 가사 패널(`Alt+L`), 우측 재생 대기열 및 하단 미러 파형 바 제공
  - **2차 창 (Compact Mode)**: 앨범 아트 중심의 모놀리스 뷰와 보더리스 재생목록 뷰 전환 지원
  - **3차 창 (Mini Squircle Mode)**: `296×296` 스쿼클 미니 위젯, 우측 상단 항상 위(Always-on-Top) 핀 고정 지원
- **샘플 정확도를 보장하는 듀얼 엔진 Gapless Playback**
  - WebAudio `AudioBufferSourceNode.start(when, offset, duration)` 사전 스케줄링과 듀얼 `<audio>` 데크를 결합해 트랙 경계 0ms 무음 전환 지원
  - `MP3` (Xing/LAME `enc_delay`/`enc_padding`), `M4A/AAC` (`iTunSMPB`), `Opus` (`pre_skip`), 무손실 포맷(`FLAC`, `WAV`, `AIF`, `AIFF`) 샘플 정확도 유지
- **정돈된 고밀도 반투명 프로스티드 표면 & 동적 커버 컬러**
  - 다크/라이트 테마, 표면 불투명도(`90%~100%`), 앨범 커버 기반 동적 포인트 컬러 추출 지원
- **비파괴적 Windows 10종 파일 연결 및 조형이 통일된 포맷별 전용 아이콘**
  - 설치 시 기존 기본 앱(`UserChoice`)이나 타사 `OpenWithProgids`를 덮어쓰지 않고 `OpenWithProgids` 및 `RegisteredApplications`에 안전하게 등록
  - 공식 지원 10종 파일 아이콘이 승인된 **동일한 초승달 + 원형 별점 1개 + 2줄 웨이브 마스크**를 공유하며 포맷별 전용 매트 색상 팔레트(`WAV #7A9299`, `OGG #6878B4` 포함)로 구분됩니다.

---

## 공식 지원 오디오 포맷 (10종)

| 지원 분류 | 공식 지원 확장자 | 내부 처리 방식 및 비고 |
| :--- | :--- | :--- |
| **1. 기본 네이티브 직접 재생 (6종)** | `.mp3`, `.flac`, `.wav`, `.m4a`, `.ogg`, `.opus` | WebView2 / WebAudio 네이티브 디코더로 즉시 직접 재생 (추가 의존성 없음) |
| **2. 앱 내부 자동 변환·전처리 후 재생 (4종)** | `.aif`, `.aiff`, `.aac`, `.wma` | - **`.aif` / `.aiff`**: 순수 Rust 내장 `AIFF/AIFC → RIFF WAV` 무손실 변환기(`transcode_aiff_to_wav_cached`)로 즉시 재생<br>- **`.aac`**: 순수 Rust `ftyp`(MP4) 리먹싱 및 선두 `ID3v2` 태그 스트리핑(`prepare_aac_playable_path`) 후 재생<br>- **`.wma`**: Windows 10/11 내장 Media Foundation(`mfplat.dll` + `mfreadwrite.dll` `IMFSourceReader`, 타입 안전 `#[repr(C)]` COM vtable 바인딩 및 RAII `CoInitializeEx`/`MFShutdown` 수명 관리) 디코더 연동 및 순수 Rust ASF/WMA 태그·길이·커버 파서 탑재 *(WMA Standard 기준 지원; DRM 보호 파일이나 일부 특수 WMA 변형은 코덱 환경에 따라 제한될 수 있음)* |

> **미지원 포맷 처리 안내**: `v0.1.0` 공식 지원 범위에서 제외된 형식(`.wv`, `.ape`, `.mpc` 등)을 드래그하거나 실행한 경우 앱이 충돌하지 않으며, 미지원 형식 안내 토스트를 표시한 뒤 다음 곡으로 안전하게 건너뜁니다.

---

## 설치 및 빌드 안내

- 설치 프로그램 기본 경로: `C:\Program Files\Droem`
- 상세 설치 및 파일 연결 안내: [`INSTALL_GUIDE.md`](./INSTALL_GUIDE.md)
- 서드파티 오픈소스 고지문: 제3자 오픈소스 라이선스 고지문은 배포 패키지 및 설치 폴더 내 [`THIRD_PARTY_NOTICES.md`](./THIRD_PARTY_NOTICES.md) 파일에서 확인할 수 있습니다.

## 라이선스

- **소스 코드 라이선스**: [MIT No Attribution (MIT-0)](./LICENSE)
- **Copyright**: © 2026 so1fr1d
