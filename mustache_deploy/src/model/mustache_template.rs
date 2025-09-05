use crate::common::*;

/// Mustache 템플릿의 정보를 담는 데이터 구조체
/// 
/// Elasticsearch에 저장될 mustache 스크립트의 이름과 내용을 포함합니다.
/// Serialize/Deserialize를 지원하여 JSON 변환이 가능하며,
/// Getters derive를 통해 필드 접근자를 자동 생성합니다.
#[derive(Serialize, Deserialize, Debug, Getters, new)]
#[getset(get = "pub")]
pub struct MustacheTemplate {
    /// 템플릿의 이름 (Elasticsearch script ID로 사용됨)
    pub script_name: String,
    /// 템플릿의 실제 mustache 스크립트 내용
    pub script: String,
}
