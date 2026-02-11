use tracing::{debug, error, info, instrument, warn};

// 애플리케이션 전체 로거 초기화
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🛒 추적 로깅 시스템 초기화 완료");
}

#[instrument]
pub fn log_request_start(path: &str, method: &str) {
    info!("📥 {} {} - 요청 시작", method, path);
}

#[instrument]
pub fn log_request_complete(path: &str, method: &str, status: u16) {
    info!("📤 {} {} - 완료 (상태: {})", method, path, status);
}

#[instrument]
pub fn log_user_creation(username: &str, email: &str) {
    info!("👤 사용자 생성 - 사용자: {}, 이메일: {}", username, email);
}

#[instrument]
pub fn log_user_lookup(user_id: i64) {
    info!("🔍 사용자 조회 - ID: {}", user_id);
}

#[instrument]
pub fn log_user_not_found(user_id: i64) {
    warn!("❌ 사용자를 찾을 수 없음 - ID: {}", user_id);
}

pub fn log_server_start(bind_addr: &str) {
    info!("🌐 서버 시작 - 주소: {}", bind_addr);
}

#[allow(dead_code)]
#[instrument]
pub fn log_server_error(error: &str) {
    error!("🔥 서버 에러: {}", error);
}

#[instrument]
pub fn log_template_render(template_name: &str) {
    debug!("🎨 템플릿 렌더링 - {}", template_name);
}

#[allow(dead_code)]
#[instrument]
pub fn log_database_operation(operation: &str, table: &str) {
    debug!("💾 데이터베이스 작업 - {} on {}", operation, table);
}

#[allow(dead_code)]
#[instrument]
pub fn log_cors_request(origin: &str, method: &str) {
    debug!("🌍 CORS 요청 - 출처: {}, 메서드: {}", origin, method);
}
