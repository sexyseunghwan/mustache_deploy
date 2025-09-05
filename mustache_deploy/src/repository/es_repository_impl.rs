use crate::common::*;

use crate::traits::repository::es_repository::*;

use crate::utils_modules::io_utils::*;

use crate::model::cluster_info::*;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct EsRepositoryImpl {
    pub es_clients: Vec<Arc<EsClient>>,
}

#[derive(Debug, Getters, Clone, new)]
pub struct EsClient {
    es_conn: Elasticsearch,
}

impl EsRepositoryImpl {
    #[doc = "Elasticsearch connection 인스턴스를 초기화 해주는 함수"]
    /// # Arguments
    /// * `path`    - Elasticsearch connection 정보가 존재하는 경로
    ///
    /// # Returns
    /// * anyhow::Result<Self>
    pub fn new() -> anyhow::Result<Self> {
        /* 프로그램 실행환경 설정 -> 해당 환경에 따라서 어떤 elasticsearch 를 바라볼건지 정해짐 */
        let args: Vec<String> = std::env::args().collect();
        let mut env_type: &str = DEFAULT_ENV; /* 환경 기본값 */
        if args.len() >= 5 && args[1] == "--env" {
            env_type = &args[2];
        }

        let es_info_path: &str = match env_type {
            "prod" => ENV_PROD_PATH,
            "dev" => ENV_DEV_PATH,
            _ => {
                error!(
                    "[WARN][EsRepositoryImpl->new] The execution case must be specified as either prod or dev. Because other arguments are provided, it will be executed in the 'dev' environment."
                );
                ENV_DEV_PATH
            }
        };

        let es_info: String = env::var(es_info_path).map_err(|e| {
            error!("[ERROR][EsRepositoryImpl->new] {:?}", e);
            anyhow!("Failed to get environment variable {}: {:?}", es_info_path, e)
        })?;

        let copy_es_info: ClusterInfo = read_toml_from_file::<ClusterInfo>(&es_info)
            .map_err(|e| {
                error!("[ERROR][EsRepositoryImpl->new] {:?}", e);
                anyhow!("Failed to read cluster info from {}: {:?}", es_info, e)
            })?;

        let mut es_clients: Vec<Arc<EsClient>> = Vec::new();

        for url in copy_es_info.hosts() {
            
            let parse_url: String = if copy_es_info.es_id().is_empty() && copy_es_info.es_pw().is_empty() {
                format!("{}{}", HTTP_PROTOCOL, url)
            } else {
                format!(
                    "{}{}:{}@{}",
                    HTTP_PROTOCOL,
                    encode(copy_es_info.es_id()),
                    encode(copy_es_info.es_pw()),
                    url
                )
            };

            let es_url: Url = Url::parse(&parse_url).map_err(|e| {
                error!("[ERROR][EsRepositoryImpl->new] {:?}", e);
                anyhow!("Failed to parse URL {}: {:?}", parse_url, e)
            })?;

            let conn_pool: SingleNodeConnectionPool = SingleNodeConnectionPool::new(es_url);
            let transport: Transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(CONNECTION_TIMEOUT_SECS, 0))
                .build()
                .map_err(|e| {
                    error!("[ERROR][EsRepositoryImpl->new] {:?}", e);
                    anyhow!("Failed to build transport: {:?}", e)
                })?;

            let elastic_conn: Elasticsearch = Elasticsearch::new(transport);
            let es_client: Arc<EsClient> = Arc::new(EsClient::new(elastic_conn));
            es_clients.push(es_client);
        }

        Ok(EsRepositoryImpl {
            es_clients,
        })
    }

    #[doc = "특정 노드의 부하를 줄이기 위해 request를 각 노드로 분산시켜주는 함수"]
    async fn execute_on_any_node<F, Fut>(&self, operation: F) -> Result<Response, anyhow::Error>
    where
        F: Fn(Elasticsearch) -> Fut + Send + Sync,
        Fut: Future<Output = Result<Response, anyhow::Error>> + Send,
    {
        let mut last_error: Option<anyhow::Error> = None;

        let mut rng: StdRng = StdRng::from_entropy(); /* 랜덤 시드로 생성 */

        /* 클라이언트 목록을 셔플 -> StdRng를 사용하여 셔플*/
        let mut shuffled_clients: Vec<Arc<EsClient>> = self.es_clients.clone();
        shuffled_clients.shuffle(&mut rng);

        /* 셔플된 클라이언트들에 대해 순차적으로 operation 수행 */
        for es_client in shuffled_clients {
            let es_conn: Elasticsearch = es_client.es_conn.clone();

            match operation(es_conn).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    last_error = Some(err);
                }
            }
        }

        /* 모든 노드에서 실패했을 경우 에러 반환 */
        Err(anyhow::anyhow!(
            "All Elasticsearch nodes failed. Last error: {:?}",
            last_error
        ))
    }
}

#[async_trait]
impl EsRepository for EsRepositoryImpl {

    #[doc = "배포 대상이 되는 mustache template 을 실제 elasticsearch server에 배포해주는 함수"]
    async fn put_mustache_template(
        &self,
        template_name: &str,
        template_content: &str,
    ) -> anyhow::Result<bool> {
        let endpoint: String = format!("/_scripts/{}", template_name);

        let script_body: Value = serde_json::json!({
            "script": {
                "lang": "mustache",
                "source": template_content
            }
        });

        let body_string: String = serde_json::to_string(&script_body)?;

        let response: Response = self
            .execute_on_any_node(move |es_client| {
                let endpoint: String = endpoint.clone();
                let body_string: String = body_string.clone();

                async move {
                    let response: Response = es_client
                        .transport()
                        .send(
                            Method::Put,
                            endpoint.as_str(),
                            HeaderMap::new(),
                            None::<&str>,
                            Some(body_string.as_bytes()),
                            None::<Duration>,
                        )
                        .await?;

                    Ok(response)
                }
            })
            .await?;

        if response.status_code().is_success() {
            Ok(true)
        } else {
            let error_body: String = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!(
                "[ERROR][EsRepositoryImpl->put_mustache_template] Failed to PUT template '{}': {}",
                template_name,
                error_body
            ))
        }
    }
}
