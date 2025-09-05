# Mustache Deploy

Elasticsearch Mustache 템플릿을 Git Flow를 통해 자동으로 배포하는 Rust 프로그램입니다.

## 개요

이 프로그램은 로컬 파일 시스템에 저장된 Mustache 템플릿을 읽어서 Elasticsearch 클러스터에 자동으로 배포합니다. 개발환경과 운영환경을 구분하여 관리할 수 있으며, 여러 Elasticsearch 노드에 부하 분산을 지원합니다.

## 주요 기능

- 📄 **템플릿 읽기**: 지정된 경로에서 배포 대상 템플릿 목록 읽기
- 🔍 **템플릿 스캔**: `.es` 파일에서 Mustache 템플릿 내용 추출
- 🚀 **자동 배포**: Elasticsearch 클러스터에 템플릿 자동 배포
- 🌐 **환경별 관리**: 개발환경(`dev`)과 운영환경(`prod`) 구분 지원
- ⚖️ **부하 분산**: 여러 ES 노드에 요청 분산 처리
- 📊 **로깅**: 상세한 실행 로그 및 에러 처리

## 프로젝트 구조

```
src/
├── main.rs                           # 프로그램 진입점
├── common/                           # 공통 모듈 및 의존성
├── config/                           # 설정 관련
├── controller/
│   └── main_controller.rs            # 메인 컨트롤러 (배포 작업 조율)
├── model/
│   ├── mustache_template.rs          # Mustache 템플릿 모델
│   └── cluster_info.rs               # ES 클러스터 정보 모델
├── repository/
│   └── es_repository_impl.rs         # Elasticsearch 저장소 구현
├── service/
│   ├── template_deployer_impl.rs     # 템플릿 배포 서비스
│   ├── template_reader_impl.rs       # 템플릿 읽기 서비스
│   └── template_scanner_impl.rs      # 템플릿 스캔 서비스
├── traits/                           # 트레이트 정의
│   ├── repository/
│   │   └── es_repository.rs
│   └── service/
│       ├── template_deployer.rs
│       ├── template_reader.rs
│       └── template_scanner.rs
└── utils_modules/                    # 유틸리티 모듈
    ├── logger_utils.rs
    ├── time_utils.rs
    └── io_utils.rs
```

## 설치 및 설정

### 1. 프로젝트 클론 및 빌드

```bash
git clone <repository-url>
cd mustache_deploy
cargo build --release
```

### 2. 환경 변수 설정

`.env` 파일을 생성하고 다음 변수들을 설정하세요:

```env
# 배포 대상 템플릿 목록 파일 경로
DEPLOY_TARGET_PATH=relative/path/to/deploy_list.txt

# Mustache 템플릿 파일들이 위치한 디렉토리 경로
MUSTACHE_TEMPLATE_LIST_INFO_PATH=relative/path/to/templates/

# 개발환경 Elasticsearch 설정 파일 경로
ES_DEV_PATH=/path/to/dev_cluster_info.toml

# 운영환경 Elasticsearch 설정 파일 경로
ES_PROD_PATH=/path/to/prod_cluster_info.toml
```

### 3. Elasticsearch 클러스터 설정

클러스터 정보 TOML 파일을 생성하세요:

```toml
# cluster_info.toml 예시
hosts = ["localhost:9200", "localhost:9201"]
es_id = "elastic"
es_pw = "password"
```

## 사용법

### 기본 실행

```bash
cargo run -- --env dev --path /path/to/base/directory
```

### 매개변수

- `--env`: 실행 환경 (`dev` 또는 `prod`)
- `--path`: 베이스 디렉토리 경로

### 템플릿 파일 형식

Mustache 템플릿 파일(`.es`)은 다음과 같은 형식이어야 합니다:

```
"""
{
  "query": {
    "match": {
      "{{field}}": "{{value}}"
    }
  }
}
"""
```

## 작동 원리

1. **템플릿 목록 읽기**: `DEPLOY_TARGET_PATH`에서 배포할 템플릿 목록을 읽습니다.
2. **템플릿 스캔**: 각 템플릿 파일을 읽어서 `"""` 구분자 사이의 내용을 추출합니다.
3. **Elasticsearch 연결**: 환경에 따라 적절한 ES 클러스터에 연결합니다.
4. **템플릿 배포**: 각 템플릿을 `/_scripts/{template_name}` 엔드포인트를 통해 배포합니다.

## 의존성

주요 사용된 크레이트들:

- `tokio`: 비동기 런타임
- `elasticsearch`: Elasticsearch 클라이언트
- `serde`: 직렬화/역직렬화
- `anyhow`: 에러 처리
- `flexi_logger`: 로깅
- `reqwest`: HTTP 클라이언트
- `regex`: 정규표현식

## 에러 처리

프로그램은 다음과 같은 상황에서 에러를 처리합니다:

- 잘못된 명령줄 인수
- 환경 변수 누락
- 파일 읽기 실패
- Elasticsearch 연결 실패
- 템플릿 배포 실패

각 에러는 상세한 로그와 함께 기록됩니다.

## 로깅

프로그램은 다음 레벨의 로그를 제공합니다:

- `INFO`: 일반적인 작업 진행 상황
- `ERROR`: 에러 발생 시 상세 정보
- `WARN`: 경고 상황

## 라이선스

MIT License

## 작성자

- **Author**: Seunghwan Shin
- **Created**: 2025-09-05
- **Version**: 1.0.0

## 기여하기

버그 리포트나 기능 제안은 GitHub Issues를 통해 제출해 주세요.