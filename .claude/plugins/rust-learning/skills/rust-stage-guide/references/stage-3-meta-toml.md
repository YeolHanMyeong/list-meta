# Stage 3: .meta.toml 파싱

## 학습 목표

`.meta.toml` 파일을 읽고 파싱하여 파일/폴더 설명을 출력에 추가한다.

## 핵심 개념

### serde
- **Ser**ialize / **De**serialize의 줄임말
- `#[derive(Deserialize)]` — 구조체에 역직렬화 기능 부여
- `serde`는 프레임워크, 실제 포맷 변환은 `toml` 크레이트가 담당

### toml 크레이트
- `toml::from_str::<T>(content)` — TOML 문자열을 구조체 `T`로 변환
- `Cargo.toml`에 추가: `toml = "0.8"`, `serde = { version = "1", features = ["derive"] }`

### .meta.toml 구조

```toml
[folder]
description = "소스 코드 디렉토리"

[file]
"main.rs" = "Rust 엔트리포인트"
"lib.rs" = "라이브러리 모듈"
```

### 이 구조를 표현하는 Rust 타입
- `[folder]` → 구조체의 `folder` 필드
- `[file]` → `HashMap<String, String>` 으로 표현 가능
- 두 섹션 모두 없을 수 있으므로 `Option`으로 감싸기

### HashMap
- `std::collections::HashMap<K, V>`
- `.get(key)` → `Option<&V>`
- `.contains_key(key)` → `bool`
- TOML의 `[file]` 섹션은 키-값 쌍이므로 `HashMap`이 자연스러움

### Option
- `Option<T>` — 값이 있거나(`Some(T)`) 없거나(`None`)
- `.unwrap_or_default()` — `None`이면 기본값
- `.map(|v| ...)` — `Some`이면 변환
- `if let Some(v) = option { ... }` — 패턴 매칭
- `.meta.toml`이 없을 수도 있으므로 파일 읽기 결과도 `Option`으로 처리

## 이 단계에서 구현할 것

1. `Cargo.toml`에 `toml`과 `serde` 의존성 추가
2. `.meta.toml` 구조를 표현하는 구조체 정의
3. 파일 읽기 → 파싱 → 구조체 변환
4. 파일 목록 출력 시 설명 컬럼 추가
5. 하위 디렉토리의 `.meta.toml`에서 폴더 설명 읽기

## 검색 키워드

- `rust serde derive Deserialize`
- `rust toml crate parse`
- `rust HashMap get Option`
- `rust Option pattern matching`

## 체크리스트

- [ ] `.meta.toml`이 없어도 프로그램이 정상 동작하는가?
- [ ] `[folder]` 섹션의 설명이 출력되는가?
- [ ] `[file]` 섹션의 파일별 설명이 출력되는가?
- [ ] 하위 디렉토리의 `.meta.toml`에서 폴더 설명을 읽는가?
- [ ] TOML 파싱 에러 시 적절한 처리를 하는가?
