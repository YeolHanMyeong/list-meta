# Stage 2: 파일 정보 표시 (ls -al 스타일)

## 학습 목표

각 파일/폴더에 대해 권한, 크기, 수정 시간 등 상세 정보를 `ls -al` 형식으로 출력한다.

## 핵심 개념

### Metadata
- `DirEntry::metadata()` 또는 `std::fs::metadata(path)` → `Result<Metadata>`
- `metadata.len()` — 파일 크기 (바이트)
- `metadata.modified()` — 수정 시간 (`SystemTime`)
- `metadata.is_dir()`, `metadata.is_file()`, `metadata.is_symlink()`
- `metadata.permissions()` — `Permissions` 타입

### Unix 권한 (std::os::unix::fs::PermissionsExt)
- `permissions.mode()` → `u32` (Unix 모드 비트)
- `0o755`, `0o644` 등 8진수 리터럴
- 비트 연산으로 `rwxrwxrwx` 문자열 생성:
  - `mode & 0o400` → owner read
  - `mode & 0o200` → owner write
  - `mode & 0o100` → owner execute
  - 이하 group, other도 같은 패턴

### 포맷팅
- `format!` 매크로의 정렬 옵션: `{:<10}` (좌측 정렬), `{:>10}` (우측 정렬)
- 파일 크기를 사람이 읽기 좋은 형태로 변환 (B, KB, MB)
- `SystemTime` → 날짜 문자열 변환

### 트레이트
- `Display` 트레이트 — `fmt::Display`를 구현하면 `{}` 포맷에 사용 가능
- 자신만의 구조체에 `Display`를 구현하면 출력이 깔끔해짐
- `impl fmt::Display for MyStruct { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { ... } }`

## 이 단계에서 구현할 것

1. 파일 타입 표시 (`d`, `-`, `l`)
2. 권한 문자열 생성 (`rwxr-xr-x`)
3. 파일 크기 포맷팅
4. 수정 시간 포맷팅
5. 전체를 정렬된 컬럼으로 출력

## 검색 키워드

- `rust std::fs::Metadata`
- `rust PermissionsExt mode`
- `rust format alignment`
- `rust Display trait implementation`
- `rust SystemTime to string`

## 체크리스트

- [ ] 파일 타입(d/-/l)이 표시되는가?
- [ ] 권한이 `rwxr-xr-x` 형식으로 표시되는가?
- [ ] 파일 크기가 표시되는가?
- [ ] 수정 시간이 표시되는가?
- [ ] 컬럼이 정렬되어 있는가?
- [ ] 디렉토리와 파일이 구분 가능한가?
