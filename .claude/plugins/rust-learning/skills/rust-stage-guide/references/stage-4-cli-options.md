# Stage 4: CLI 옵션 처리

## 학습 목표

`clap` 크레이트를 사용하여 커맨드라인 인자를 처리한다.

## 핵심 개념

### clap derive API
- `#[derive(Parser)]` — 구조체를 CLI 파서로 변환
- `Cargo.toml`에 추가: `clap = { version = "4", features = ["derive"] }`
- `use clap::Parser;`

### 구조체 필드와 CLI 옵션 매핑
- `#[arg(long)]` — `--옵션명` 플래그
- `#[arg(short, long)]` — `-o`, `--옵션명` 둘 다 사용 가능
- `#[arg(default_value_t = 80)]` — 기본값 지정
- `bool` 타입 → 플래그 (있으면 `true`)
- `Option<T>` → 선택적 인자
- `Vec<T>` → 여러 값 받기

### derive 매크로
- Rust의 `#[derive(...)]`는 컴파일 타임에 코드를 자동 생성
- `clap`의 `Parser`는 proc-macro: 구조체 정의를 분석하여 파싱 코드 생성
- `serde`의 `Deserialize`도 같은 원리

### Parser 트레이트
- `Args::parse()` — 커맨드라인에서 인자 파싱
- 잘못된 인자 시 자동으로 에러 메시지 + 도움말 출력
- `--help` 자동 생성

## list-meta에 필요한 옵션

1. `--show-meta` (bool): `.meta.toml` 파일을 목록에 표시
2. `--width <N>` (Option<usize>): 출력 너비 직접 지정
3. 위치 인자 (Option<PathBuf>): 대상 디렉토리 경로 (기본: 현재 디렉토리)

## 이 단계에서 구현할 것

1. `Cargo.toml`에 `clap` 의존성 추가
2. CLI 옵션 구조체 정의 (`#[derive(Parser)]`)
3. `main()`에서 `Args::parse()` 호출
4. 파싱된 옵션을 기존 로직에 연결
5. `--show-meta` 플래그로 `.meta.toml` 표시/숨김 제어

## 검색 키워드

- `rust clap derive Parser`
- `rust clap arg attribute`
- `rust derive macro`
- `rust clap positional argument`

## 체크리스트

- [ ] `lm --help`가 도움말을 출력하는가?
- [ ] `lm --show-meta`가 `.meta.toml`을 목록에 포함하는가?
- [ ] `lm --width 120`이 출력 너비를 지정하는가?
- [ ] `lm /some/path`로 다른 디렉토리를 지정할 수 있는가?
- [ ] 잘못된 옵션 입력 시 에러 메시지가 출력되는가?
