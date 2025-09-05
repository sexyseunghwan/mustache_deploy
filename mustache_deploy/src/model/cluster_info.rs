use crate::common::*;

/// Elasticsearch 클러스터 연결 정보를 담는 데이터 구조체
/// 
/// TOML 파일에서 로드되는 Elasticsearch 클러스터의 연결 설정을 포함합니다.
/// Serialize/Deserialize를 지원하여 TOML 파싱이 가능하며,
/// Getters derive를 통해 필드 접근자를 자동 생성합니다.
#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct ClusterInfo {
    /// Elasticsearch 클러스터의 이름
    pub cluster_name: String,
    /// 클러스터를 구성하는 호스트 목록 (host:port 형식)
    pub hosts: Vec<String>,
    /// Elasticsearch 인증을 위한 사용자 ID
    pub es_id: String,
    /// Elasticsearch 인증을 위한 비밀번호
    pub es_pw: String,
}
