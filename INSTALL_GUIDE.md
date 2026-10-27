# Drøm v0.1.0 — 설치 및 사용 안내 (INSTALL_GUIDE.md)

## 1. 시스템 요구 사항
- **운영체제**: Windows 10 (1809 이상) 또는 Windows 11 (64-bit / x64)
- **런타임**: Microsoft Edge WebView2 런타임 (Windows 11 기본 내장, 미설치 시 설치기가 자동 구성)

## 2. 설치 경로 및 NSIS 설치기 구성
- **표준 설치 경로**: `C:\Program Files\Droem` (`hooks.nsh`가 한글·특수문자 경로 충돌 방지를 위해 ASCII 표준 폴더로 정규화합니다)
- **설치기 디자인**: 시작(Welcome) 및 완료(Finish) 페이지 좌측에 소형 패널 전용으로 최적화된 Drøm 브랜드 패널(`src-tauri/windows/nsis-sidebar.bmp`, `164×314` 24-bit 비압축 BMP — 달+웨이브 아이콘 및 `drøm` 워드마크)이 선명하게 표시되며, 우측 설치 UI와 진행(Installing) 화면은 Tauri/NSIS 표준 구성을 유지합니다.
- **시작 메뉴**: `Drøm` 시작 메뉴 바로가기가 기본 생성됩니다.
- **바탕화면 바로가기**: 기본 생성하지 않으며, 필요한 경우 설치 시 `/DESKTOP=1` 옵션을 전달해 생성할 수 있습니다.

## 3. 공식 지원 10종 파일 연결 및 포맷별 전용 아이콘
- **공식 지원 포맷 (10종)**: `MP3`, `FLAC`, `WAV`, `M4A`, `AAC`, `OGG`, `OPUS`, `WMA`, `AIF`, `AIFF`
- **기본 앱 지정 방법**:
  1. 설치가 완료되면 Drøm을 실행하고 **`F10` (설정) → `일반` 탭** 하단의 **`Windows 기본 앱 설정 열기 ↗`** 버튼을 클릭합니다.
  2. Windows 설정 → 앱 → 기본 앱 목록에서 **Drøm**을 선택하고 원하는 확장자의 기본 앱으로 지정합니다.
  3. 지정 즉시 승인된 **초승달 + 원형 별점 1개 + 2줄 웨이브**의 100% 동일한 조형 마스크와 포맷별 고유 매트 컬러(`WAV #7A9299`, `OGG #6878B4` 포함)가 적용된 **10종 전용 파일 아이콘(`audio_<ext>.ico`)** 이 표시됩니다.
- **기존 13종 설치본에서 업데이트 시 정리 동작**:
  - 이전 테스트 버전에서 등록된 `WV`, `APE`, `MPC`의 Drøm 소유 레지스트리 값(`DroemCreated="1"`, `Droem.Audio.wv/ape/mpc` 등)만 안전하게 정리하며, 사용자의 `UserChoice`나 타사 프로그램의 `OpenWithProgids`는 절대 변경하지 않습니다.

## 4. 포맷별 재생 및 미지원 형식 안내
- **기본 직접 재생 (6종)**: `.mp3`, `.flac`, `.wav`, `.m4a`, `.ogg`, `.opus`
- **앱 내장 자동 변환·전처리 재생 (4종)**:
  - `.aif`, `.aiff` (앱 내장 순수 Rust 무손실 WAV 변환)
  - `.aac` (앱 내장 순수 Rust ADTS/MP4 전처리)
  - `.wma` (Windows Media Foundation `IMFSourceReader` COM 디코더 및 순수 Rust ASF 태그/커버 파서 연동 — WMA Standard 기준)
- **미지원 포맷(`.wv`, `.ape`, `.mpc` 등) 드래그/실행 시**:
  - 앱이 멈추거나 충돌하지 않고 하단에 미지원 형식 안내 토스트를 표시한 뒤 정상적으로 다음 트랙으로 건너뜁니다.

## 5. 오픈소스 라이선스 및 제3자 고지문 확인
- **소스 코드 라이선스**: `MIT-0` (루트 `LICENSE` 파일)
- **제3자 오픈소스 고지문**: 제3자 고지문은 배포 패키지 및 설치 폴더(`C:\Program Files\Droem` 리소스)의 `THIRD_PARTY_NOTICES.md` 파일에서 확인할 수 있습니다.
