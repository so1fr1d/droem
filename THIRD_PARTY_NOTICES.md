# THIRD-PARTY OPEN SOURCE NOTICES

**drøm v1.0.0** (`Windows x64`)의 자체 소스 코드(`index.html`, `src-tauri/src/main.rs`, `vite.config.ts`, 빌드·설치 스크립트)와 수학적 SDF/벡터 알고리즘으로 직접 생성한 자체 아이콘 자산(`src-tauri/icons/*`)은 **so1fr1d**가 개발하였으며 **MIT-0 (MIT No Attribution, `SPDX-License-Identifier: MIT-0`)** 라이선스로 공개됩니다.

단, 본 소프트웨어가 사용하는 **제3자 오픈소스 라이브러리(`src-tauri/Cargo.lock`, `package-lock.json`) 및 런타임·설치 도구**에는 각 원저작자의 라이선스가 그대로 적용됩니다. 또한 외부 저작물(타인의 소스 코드, 외부 폰트 파일, 제3자 아이콘, 사용자의 음원 및 앨범아트)에는 MIT-0가 자동 적용되지 않으며, 권한이 확인되지 않은 외부 자산은 공개 패키지에 일절 포함하지 않습니다.

---

## 1. 직접 의존성 및 핵심 프레임워크 (`src-tauri/Cargo.toml` / `package.json`)

| 컴포넌트 / 패키지 | 기준 버전 (`Cargo.toml` / `package.json`) | 라이선스 (SPDX) | 저작권자 / 출처 | 용도 및 배포 포함 여부 |
| :--- | :--- | :--- | :--- | :--- |
| **`tauri`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | 데스크톱 애플리케이션 코어 프레임워크 (런타임 포함) |
| **`tauri-build`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | Windows 리소스·매니페스트 빌드 스크립트 (빌드 시점 사용) |
| **`tauri-plugin-single-instance`** | `2.x` (`Cargo.lock` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | 단일 인스턴스 실행 및 탐색기 다중 파일 인자 전달 (런타임 포함) |
| **`serde` / `serde_json`** | `1.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Erickson, David Tolnay & Serde Contributors | 직렬화 및 JSON 데이터 파싱 (런타임 포함) |
| **`lofty`** | `0.21.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Serial-ATA & Lofty Contributors | 오디오 메타데이터(ID3v2, Vorbis, MP4, AIFF 등) 및 커버 파싱 (런타임 포함) |
| **`walkdir`** | `2.5.x` (`Cargo.lock` 고정) | `Unlicense OR MIT` | Copyright (c) Andrew Gallant (BurntSushi) | 고속 로컬 음악 디렉터리 순회 (런타임 포함) |
| **`rayon`** | `1.10.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Niko Matsakis, Josh Stone & Rayon Contributors | 멀티코어 병렬 오디오 파일 스캔 (런타임 포함) |
| **`base64`** | `0.22.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Alice Maz, Marshall Pierce & Contributors | 앨범 커버 이미지 Base64 인코딩 (런타임 포함) |
| **`rfd` (Rusty File Dialogs)** | `0.15.x` (`Cargo.lock` 고정) | `MIT` | Copyright (c) PolyMeilex & RFD Contributors | Windows 네이티브 파일/폴더 열기 대화상자 (런타임 포함) |
| **`trash`** | `5.x` (`Cargo.lock` 고정) | `MIT` | Copyright (c) Byron (Sebastian Thiel), Artururas & Contributors | Windows 시스템 휴지통(Recycle Bin) 이동 (런타임 포함) |
| **`drag`** | `2.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) CrabNebula / Tauri Contributors | 외부 프로그램으로 네이티브 파일 드래그 아웃 (런타임 포함) |
| **`windows` / `windows-core`** | `0.58.x` (`Cargo.lock` 고정) | `MIT OR Apache-2.0` | Copyright (c) Microsoft Corporation | Windows WinRT SMTC, DWM 및 Win32 API 바인딩 (런타임 포함) |
| **`vite` (빌드 전용)** | `8.3.4` (`package-lock.json` 고정) | `MIT` | Copyright (c) 2019-present VoidZero Inc., Yuxi (Evan) You and Vite contributors | 프론트엔드 번들러 (`vite.config.ts`, `tsc && vite build` — 최종 바이너리 미포함) |
| **`typescript` (빌드 전용)** | `6.0.3` (`package-lock.json` 고정) | `Apache-2.0` | Copyright (c) Microsoft Corporation | TypeScript 타입 검사기 (`tsc && vite build` — 최종 바이너리 미포함) |
| **`@tauri-apps/cli` / `@tauri-apps/api`** | `2.x` (`package-lock.json` 고정) | `Apache-2.0 OR MIT` | Copyright (c) 2019-present Tauri Programme within The Commons Conservancy | Tauri CLI 번들링 도구 및 IPC 타입 바인딩 |

---

## 2. 잠금 파일(`src-tauri/Cargo.lock` & `package-lock.json`) 기반 전이 의존성 대조

`src-tauri/Cargo.lock` 및 `package-lock.json`에 의해 해석되는 하위 전이(Transitive) 구성요소들과 적용 라이선스는 다음과 같습니다:

| 분류 | 주요 구성요소 (`Cargo.lock` / `package-lock.json`) | 적용 라이선스 (SPDX) | 저작권자 및 비고 |
| :--- | :--- | :--- | :--- |
| **Tauri 창·WebView2 런타임 바인딩** | `tao`, `wry`, `webview2-com`, `webview2-com-sys`, `webview2-com-macros`, `dpi`, `muda`, `tray-icon`, `window-vibrancy` | `Apache-2.0 OR MIT` | Copyright (c) Tauri Programme within The Commons Conservancy |
| **Windows Win32 / WinRT 시스템 바인딩** | `windows`, `windows-core`, `windows-result`, `windows-strings`, `windows-sys`, `windows-targets`, `windows-implement`, `windows-interface`, `windows_x86_64_msvc` | `MIT OR Apache-2.0` | Copyright (c) Microsoft Corporation |
| **비동기 런타임 & 동시성** | `tokio`, `tokio-macros`, `mio`, `bytes`, `pin-project-lite`, `parking_lot`, `parking_lot_core`, `lock_api`, `scopeguard`, `smallvec`, `once_cell` | `MIT` 또는 `MIT OR Apache-2.0` | Copyright (c) Tokio Contributors, Amanieu d'Antras & Contributors |
| **멀티코어 병렬 처리 (`rayon` 하위)** | `rayon-core`, `crossbeam-deque`, `crossbeam-epoch`, `crossbeam-utils`, `either` | `MIT OR Apache-2.0` | Copyright (c) Crossbeam & Rayon Contributors |
| **오디오 태그 & 이미지/압축 처리 (`lofty` 하위)** | `lofty_attr`, `ogg_pager`, `byteorder`, `flate2`, `miniz_oxide`, `adler2`, `crc32fast`, `png`, `fdeflate`, `simd-adler32`, `bitflags`, `cfg-if` | `MIT OR Apache-2.0` / `Unlicense OR MIT` / `Zlib` | Copyright (c) Serial-ATA, BurntSushi, Image-rs & Contributors |
| **파일시스템 & 문자열/정규식 유틸리티** | `same-file`, `memchr`, `aho-corasick`, `regex`, `regex-automata`, `regex-syntax`, `itoa`, `ryu`, `log` | `Unlicense OR MIT` / `MIT OR Apache-2.0` | Copyright (c) Andrew Gallant (BurntSushi), David Tolnay & Contributors |
| **URL · 유니코드 · 윈도우 핸들 표준** | `url`, `idna`, `form_urlencoded`, `percent-encoding`, `unicode-bidi`, `unicode-normalization`, `raw-window-handle`, `unicode-ident` | `MIT OR Apache-2.0` / `Unicode-3.0` / `BSD-2-Clause` / `BSD-3-Clause` | Copyright (c)The Servo Project Developers, Unicode, Inc. & Contributors |
| **프로시저 매크로 & 컴파일러 보조** | `syn`, `quote`, `proc-macro2`, `thiserror`, `thiserror-impl`, `anyhow`, `serde_derive`, `serde_spanned`, `toml` | `MIT OR Apache-2.0` | Copyright (c) David Tolnay & Contributors |
| **Vite 8.3.4 / TypeScript 빌드 체인 (`package-lock.json`)** | `vite` (`8.3.4`), `typescript`, `esbuild`, `rollup`, `postcss`, `picocolors`, `nanoid`, `source-map-js` | `MIT` / `Apache-2.0` / `ISC` / `BSD-2-Clause` | 빌드 전용(`devDependencies`) — 최종 `droem.exe` 바이너리에 미포함 |

<!-- BEGIN_LOCKFILE_INVENTORY -->
### 2.1 실제 잠금 파일(`src-tauri/Cargo.lock` 및 `package-lock.json`) 대조 요약

- **`package-lock.json` 파싱**: 완료 (lockfileVersion 3; 루트 프로젝트 제외 **58개** 잠금 패키지)
- **프론트엔드 잠금 버전**: `vite 8.3.4`, `typescript 6.0.3`, `@tauri-apps/cli 2.12.2`, `@tauri-apps/api 2.12.3`
- **`src-tauri/Cargo.lock` 파싱**: 완료 (잠금 패키지 항목 **501개**; 동일 이름의 서로 다른 버전을 별도로 계산)
- **개별 Rust 크레이트 라이선스 메타데이터 전수 확인**: **미완료**. `Cargo.lock`에는 라이선스 필드가 없어 501개 항목에 대한 라이선스 준수 여부를 잠금 파일만으로 확정할 수 없습니다. 릴리즈 공개 전 실제 `cargo metadata` 및 의존 크레이트 배포물의 라이선스·고지문을 추가로 확인해야 합니다.

**주의:** 위의 핵심·전이 구성요소 표는 대표 의존성에 대한 안내이며, 모든 잠금 패키지의 SPDX 라이선스를 독립 검증한 전체 목록은 아닙니다.
<!-- END_LOCKFILE_INVENTORY -->

---

## 3. 외부 자산·폰트·아이콘·런타임 구분 고지

1. **폰트 파일 미번들 (No Bundled Fonts)**:
   - drøm은 `.ttf`, `.otf`, `.woff2` 폰트 파일을 패키지나 실행 파일 내부에 포함·재배포하지 않습니다.
   - CSS `local(...)` 선언으로 사용자 OS에 설치된 시스템/로컬 폰트(`Inter`, `Noto Sans CJK`, `Segoe UI Variable Text`, `Malgun Gothic`, `Yu Gothic UI`, `Microsoft YaHei UI`, `Microsoft JhengHei UI`)만 호출합니다.
2. **자체 제작 아이콘 자산 (`src-tauri/icons/*`)**:
   - 다크 메인 아이콘(`icon.ico`, `1_main_ref_clean.ico`, `icon.svg`, `32x32.png`, `128x128.png`, `128x128@2x.png`), 라이트 보조 아이콘(`5_ui_return_light.ico`, `icon_light.*`), UI 회귀 아이콘(`4_ui_return_dark.ico`), 공식 배포 10종 오디오 파일 아이콘(`audio_mp3.ico`, `audio_flac.ico`, `audio_wav.ico`, `audio_m4a.ico`, `audio_aac.ico`, `audio_ogg.ico`, `audio_opus.ico`, `audio_wma.ico`, `audio_aif.ico`, `audio_aiff.ico`; 기존 WV/APE/MPC 아이콘은 개발용으로만 보존)은 타인의 그래픽 자산을 복제하지 않고 코드 내 수학적 벡터/SDF 수식으로 직접 생성한 자체 저작물이며 **MIT-0**가 적용됩니다.
3. **사용자 음원 및 앨범아트**:
   - 사용자가 앱에서 불러오는 로컬 오디오 파일, 가사, 앨범 커버 이미지의 저작권은 각 권리자에게 있으며, 공개 소스 및 배포 패키지에는 어떠한 개인 음원이나 테스트용 상업 앨범 커버도 포함되지 않습니다.
4. **런타임 및 설치 도구**:
   - **Microsoft Edge WebView2 Runtime**: Windows OS의 Evergreen WebView2 런타임을 동적 호출합니다 (Copyright (c) Microsoft Corporation).
   - **NSIS (Nullsoft Scriptable Install System)**: 설치 마법사 스텁은 `Zlib` / `CPL-1.0` 등 NSIS 라이선스가 적용됩니다.

---

## 4. 제3자 오픈소스 라이선스 전문 (Unabridged Full License Texts)

### 4.1 The MIT License (`MIT`)

```text
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### 4.2 Apache License, Version 2.0 (`Apache-2.0` — 전문)

```text
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

### 4.3 BSD 2-Clause "Simplified" License (`BSD-2-Clause`)

```text
BSD 2-Clause License

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### 4.4 BSD 3-Clause "New" or "Revised" License (`BSD-3-Clause`)

```text
BSD 3-Clause License

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

### 4.5 ISC License (`ISC`)

```text
ISC License

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```

### 4.6 Zlib / libpng License (`Zlib` — NSIS Stub & Compression Crates)

```text
zlib License

This software is provided 'as-is', without any express or implied
warranty. In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.
```

### 4.7 The Unlicense (`Unlicense`)

```text
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
```

### 4.8 Unicode License v3 (`Unicode-3.0`)

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2026 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS
INCLUDED IN THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT
OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THE DATA FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```

### 4.9 Mozilla Public License Version 2.0 (`MPL-2.0` — 조건부 전이 의존성 고지)

```text
Cargo.lock 전이 의존성 중 MPL-2.0이 적용되는 크레이트(예: option-ext 등)가 포함되는 경우,
해당 크레이트는 수정 없이 원본 상태 그대로 링크되며 MPL-2.0 제1.10조 및 제3.2조에 따라
원본 소스 코드는 https://crates.io 및 각 크레이트의 공식 저장소에서 제공됩니다.
전문: https://www.mozilla.org/en-US/MPL/2.0/
```
