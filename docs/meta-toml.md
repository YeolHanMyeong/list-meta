# .meta.toml 명세

각 디렉토리에 하나의 `.meta.toml` 파일을 두어 해당 디렉토리와 그 안의 파일들에 대한 설명을 관리한다.

## 파일 형식

- `.meta.toml` 파일은 **UTF-8 인코딩의 TOML** 형식이다.
- `[folder]` 섹션: 현재 디렉토리 자신에 대한 설명을 작성한다.
- `[file]` 섹션: 현재 디렉토리에 있는 **파일들만** 설명한다. (하위 디렉토리는 포함하지 않는다)
- 하위 디렉토리의 설명은 해당 디렉토리의 `.meta.toml`의 `[folder]`를 참조한다.

## 규칙

- `.meta.toml`에 등록되지 않은 항목은 설명 출력을 생략한다.
- `.meta.toml` 파일이 없는 디렉토리는 모든 설명이 생략된다.
- 설명은 **터미널 너비에 맞춰** 자동으로 잘린다. 터미널 너비를 감지할 수 없는 경우 기본 80 컬럼이 적용된다. (`--width` 옵션으로 직접 지정 가능)
- `.meta.toml` 파일은 기본적으로 목록에서 **숨김 처리**된다. (`--show-meta` 옵션으로 표시 가능)

## 예시

```toml
# project/.meta.toml
[folder]
"description" = "프로젝트 루트 폴더"

[file]
"README.md" = "프로젝트 설명 문서"
"Cargo.toml" = "Rust 프로젝트 설정"
```

```toml
# project/src/.meta.toml
[folder]
"description" = "소스 코드 디렉토리"

[file]
"main.rs" = "Rust 엔트리포인트"
```
