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

## 문서

- [.meta.toml 명세](docs/meta-toml.md)

## 라이선스

MIT License
