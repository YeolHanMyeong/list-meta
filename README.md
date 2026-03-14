# list-meta (`lm`)

`.meta.toml` 파일을 기반으로 파일과 폴더의 설명을 `ls -al` 스타일로 함께 출력하는 Rust CLI 도구.

## 설치

```bash
cargo install --path .
```

## 사용법

```bash
lm                    # 현재 디렉토리 목록 + 설명 출력
lm --show-meta        # .meta.toml 파일도 목록에 표시
lm --width 120        # 출력 너비 지정 (기본: 터미널 너비, 폴백: 80)
```

## 출력 예시

```
drwxr-xr-x  3  elevn  staff  src         | 소스 코드 디렉토리
-rw-r--r--  1  elevn  staff  Cargo.toml  |
-rw-r--r--  1  elevn  staff  README.md   | 프로젝트 소개 문서
```

파일 정보와 설명은 `|` 구분자로 분리되며, 터미널에서는 설명 부분에 색상이 적용된다.

## `.meta.toml` 형식

각 디렉토리에 `.meta.toml` 파일을 만들어 메타데이터를 관리한다.

```toml
[folder]
description = "소스 코드 디렉토리"

[file]
"main.rs" = "Rust 엔트리포인트"
"lib.rs" = "라이브러리 모듈"
```

- `[folder]` — 현재 디렉토리 자체의 설명 (`description` 키)
- `[file]` — 현재 디렉토리 내 파일들의 설명 (`"파일명" = "설명"` 형식)
- 하위 디렉토리의 설명은 해당 디렉토리의 `.meta.toml`에 있는 `[folder]`를 참조

## 빌드

```bash
cargo build --release
```

Rust 2024 에디션을 사용하므로 최신 Rust 툴체인이 필요하다.
