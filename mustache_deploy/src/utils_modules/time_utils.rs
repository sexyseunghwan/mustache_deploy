#![allow(dead_code)]

use crate::common::*;

#[doc = "시스템에 호환되도록 날짜 타입을 변환해주는 함수"]
/// 
/// # Arguments
/// * `dt` - 변환할 NaiveDateTime 객체
/// 
/// # Returns
/// * `Ok(String)` - ISO 8601 형식의 날짜 문자열 ("YYYY-MM-DDTHH:MM:SSZ")
/// * `Err(anyhow::Error)` - 날짜 포맷팅 실패
pub fn format_datetime(dt: NaiveDateTime) -> Result<String, anyhow::Error> {
    get_str_from_naivedatetime(dt, "%Y-%m-%dT%H:%M:%SZ")
}

#[doc = "Functions that return the current UTC time -> NaiveDatetime"]
/// 
/// # Returns
/// * `NaiveDateTime` - 현재 UTC 시간을 NaiveDateTime 형식으로 반환
/// 
/// # Note
/// 함수 이름에 오타가 있음 (currnet -> current)
pub fn get_currnet_utc_naivedatetime() -> NaiveDateTime {
    let utc_now: DateTime<Utc> = Utc::now();
    utc_now.naive_local()
}

#[doc = "Function that converts the date data 'naivedate' format to the string format"]
/// 
/// # Arguments
/// * `naive_date` - 변환할 NaiveDateTime 객체
/// * `fmt` - 사용할 날짜 포맷 문자열 (strftime 형식)
/// 
/// # Returns
/// * `Ok(String)` - 지정된 포맷으로 변환된 날짜 문자열
/// * `Err(anyhow::Error)` - 날짜 포맷팅 실패
/// 
/// # Format Examples
/// * "%Y-%m-%d" -> "2025-09-05"
/// * "%Y-%m-%dT%H:%M:%SZ" -> "2025-09-05T14:30:15Z"
pub fn get_str_from_naivedatetime(
    naive_date: NaiveDateTime,
    fmt: &str,
) -> Result<String, anyhow::Error> {
    let result_date = naive_date.format(fmt).to_string();
    Ok(result_date)
}