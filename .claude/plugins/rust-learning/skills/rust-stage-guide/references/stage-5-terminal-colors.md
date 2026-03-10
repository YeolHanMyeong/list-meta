# Stage 5: 터미널 너비 + 색상

## 학습 목표

터미널 너비에 맞춰 출력을 정렬하고, 색상으로 정보를 구분한다.

## 핵심 개념

### terminal_size 크레이트
- `Cargo.toml`에 추가: `terminal_size = "0.4"`
- `terminal_size::terminal_size()` → `Option<(Width, Height)>`
- `Width(w)` — 터미널 너비 (u16)
- 터미널이 아닌 환경(파이프, 리다이렉트)에서는 `None` 반환 → 폴백값 필요 (기본 80)

### colored 크레이트
- `Cargo.toml`에 추가: `colored = "2"`
- `use colored::Colorize;` — 문자열에 색상 메서드 추가
- `"text".green()`, `"text".bold()`, `"text".blue().bold()`
- 체이닝 가능: `"text".red().on_white().bold()`
- `NO_COLOR` 환경 변수 존중 (colored가 자동 처리)

### 유니코드 너비 처리
- 한글 등 전각 문자는 터미널에서 2칸 차지
- `unicode-width` 크레이트: `UnicodeWidthStr::width("한글")` → `4`
- 포맷팅 시 `{:>10}` 같은 정렬은 바이트/문자 수 기준이라 전각 문자에서 어긋남
- 직접 패딩을 계산해야 할 수 있음

### 출력 형식 설계

```
drwxr-xr-x  4096  2024-01-15 14:30  src/         | 소스 코드 디렉토리
-rw-r--r--   234  2024-01-15 14:30  Cargo.toml   | 프로젝트 설정 파일
-rw-r--r--    42  2024-01-15 14:30  main.rs      | Rust 엔트리포인트
```

- 파일 정보와 설명을 `|` 구분자로 분리
- 설명 부분에 색상 적용
- 터미널 너비에 맞춰 설명 영역 폭 조절

## 이 단계에서 구현할 것

1. `Cargo.toml`에 `terminal_size`, `colored` 의존성 추가
2. 터미널 너비 감지 (폴백: `--width` 옵션 또는 80)
3. 파일 정보 영역과 설명 영역의 너비 계산
4. `|` 구분자와 색상 적용
5. 유니코드 너비를 고려한 정렬

## 검색 키워드

- `rust terminal_size crate`
- `rust colored crate`
- `rust unicode width`
- `rust terminal color output`

## 체크리스트

- [ ] 터미널 너비에 맞춰 출력이 정렬되는가?
- [ ] `--width` 옵션이 터미널 너비를 오버라이드하는가?
- [ ] 파이프로 출력 시 폴백 너비(80)가 적용되는가?
- [ ] 설명 부분에 색상이 적용되는가?
- [ ] `|` 구분자로 파일 정보와 설명이 구분되는가?
- [ ] 한글 등 전각 문자가 정렬에서 어긋나지 않는가?
- [ ] `NO_COLOR` 환경 변수 설정 시 색상이 비활성화되는가?
