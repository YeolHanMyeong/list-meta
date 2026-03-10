# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

list-meta (`lm`)는 `.meta.toml` 파일을 기반으로 파일과 폴더의 설명을 `ls -al` 스타일로 함께 출력하는 Rust CLI 도구이다.

## Build & Run

```bash
cargo build --release        # 빌드
cargo install --path .       # PATH에 설치
cargo run                    # 개발 중 실행
```

## Architecture

### .meta.toml 시스템

각 디렉토리에 하나의 `.meta.toml` 파일로 메타데이터를 관리한다.

- `[folder]` 섹션: 현재 디렉토리 자신에 대한 설명 (`description` 키)
- `[file]` 섹션: 현재 디렉토리의 파일들만 설명 (`"파일명" = "설명"` 형식)
- 하위 디렉토리의 설명은 해당 디렉토리의 `.meta.toml`의 `[folder]`를 참조

```toml
[folder]
description = "소스 코드 디렉토리"

[file]
"main.rs" = "Rust 엔트리포인트"
```

### CLI 옵션

- `--show-meta`: 기본적으로 숨겨진 `.meta.toml` 파일을 목록에 표시
- `--width <N>`: 출력 너비 직접 지정 (기본: 터미널 너비, 폴백: 80)

### 출력 형식

- 파일 정보와 설명을 `|` 구분자로 분리
- 터미널에서는 설명 부분에 색상 구분 적용

## Conventions

- Rust 2024 에디션 사용 — 최신 Rust 툴체인 필요
- 문서 및 주석은 한국어로 작성
