# Stage 1: 디렉토리 목록 출력

## 학습 목표

현재 디렉토리의 파일과 폴더 목록을 읽어서 출력한다.

## 핵심 개념

### std::fs 모듈
- `std::fs::read_dir(path)` → `Result<ReadDir>`를 반환
- `ReadDir`는 이터레이터: `DirEntry`를 하나씩 꺼낸다
- `DirEntry`에서 얻을 수 있는 것: `file_name()`, `path()`, `file_type()`, `metadata()`

### std::path 모듈
- `Path`와 `PathBuf`의 차이: `&str` vs `String`과 비슷한 관계
- `Path::new(".")` — 현재 디렉토리
- `.display()` — 경로를 화면에 출력할 때 사용

### Result와 에러 처리
- `Result<T, E>` — 성공(`Ok(T)`) 또는 실패(`Err(E)`)
- `unwrap()` — 간단하지만 패닉 위험
- `?` 연산자 — 에러를 호출자에게 전파 (main에서 쓰려면 반환 타입 변경 필요)
- `match` 표현식으로 에러 분기 처리

### 이터레이터 기초
- `for entry in read_dir(path)?` 패턴
- `.filter()`, `.map()`, `.collect()` 체이닝
- `entry?` — `Result<DirEntry>`에서 `DirEntry` 꺼내기

## 이 단계에서 구현할 것

1. `std::fs::read_dir(".")`로 현재 디렉토리 읽기
2. 각 엔트리의 파일명을 출력
3. `Result`를 사용한 에러 처리
4. 숨김 파일(`.`으로 시작) 필터링

## 검색 키워드

- `rust std::fs::read_dir`
- `rust DirEntry`
- `rust Result error handling`
- `rust iterator basics`

## 체크리스트

- [ ] `cargo run` 시 현재 디렉토리의 파일/폴더 목록이 출력되는가?
- [ ] `.`으로 시작하는 숨김 파일이 기본적으로 제외되는가?
- [ ] 에러 발생 시 패닉 대신 적절한 메시지를 출력하는가?
- [ ] `main` 함수의 반환 타입이 `Result`인가?
