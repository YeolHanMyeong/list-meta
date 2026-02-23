# list-meta (lm)

리눅스와 유닉스에서 `ls -al` 또는 `ll` 명령어를 통해 디렉토리의 내용을 확인할 수 있지만, 각 폴더나 파일에 대한 간단한 설명은 알 수 없다. `lm`은 `.meta.toml` 파일을 기반으로 파일과 폴더의 설명을 함께 출력하는 CLI 도구이다.

## 빌드 및 설치

```bash
# 빌드
cargo build --release

# 설치 (PATH에 추가)
cargo install --path .
```

> Rust 2024 에디션을 사용하므로 최신 Rust 툴체인이 필요하다.

## 사용법

```bash
# 현재 디렉토리의 파일/폴더 목록과 설명 출력
lm

# 특정 경로 지정
lm /path/to/dir

# .meta.toml 파일도 목록에 표시
lm --show-meta

# 출력 너비를 직접 지정 (기본: 터미널 너비, 폴백: 80)
lm --width 120
```

## .meta.toml 파일 규칙

각 디렉토리에 하나의 `.meta.toml` 파일을 두어 해당 디렉토리와 그 안의 파일들에 대한 설명을 관리한다.

- `.meta.toml` 파일은 **UTF-8 인코딩의 TOML** 형식이다.
- `[folder]` 섹션: 현재 디렉토리 자신에 대한 설명을 작성한다.
- `[file]` 섹션: 현재 디렉토리에 있는 **파일들만** 설명한다. (하위 디렉토리는 포함하지 않는다)
- 하위 디렉토리의 설명은 해당 디렉토리의 `.meta.toml`의 `[folder]`를 참조한다.
- `.meta.toml`에 등록되지 않은 항목은 설명 출력을 생략한다.
- `.meta.toml` 파일이 없는 디렉토리는 모든 설명이 생략된다.
- 설명은 **터미널 너비에 맞춰** 자동으로 잘린다. 터미널 너비를 감지할 수 없는 경우 기본 80 컬럼이 적용된다. (`--width` 옵션으로 직접 지정 가능)
- `.meta.toml` 파일은 기본적으로 목록에서 **숨김 처리**된다. (`--show-meta` 옵션으로 표시 가능)

### .meta.toml 파일 예시

```toml
# project/.meta.toml
[folder]
description = "프로젝트 루트 폴더"

[file]
"README.md" = "프로젝트 설명 문서"
"Cargo.toml" = "Rust 프로젝트 설정"
```

```toml
# project/src/.meta.toml
[folder]
description = "소스 코드 디렉토리"

[file]
"main.rs" = "Rust 엔트리포인트"
```

## 예시

### 디렉토리 구조

```
project/
├── .meta.toml
├── src/
│   ├── .meta.toml
│   └── main.rs
├── README.md
└── Cargo.toml
```

### 출력 (`project/`에서 `lm` 실행)

```
drwxr-xr-x  src/        | 소스 코드 디렉토리
-rw-r--r--  README.md   | 프로젝트 설명 문서
-rw-r--r--  Cargo.toml  | Rust 프로젝트 설정
```

- 디렉토리(`src/`)의 설명: `src/.meta.toml`의 `[folder]`에서 참조
- 파일(`README.md`, `Cargo.toml`)의 설명: `project/.meta.toml`의 `[file]`에서 참조

> 실제 터미널에서는 `|` 뒤의 설명 부분이 색상으로도 구분된다.

## 라이선스

MIT License
