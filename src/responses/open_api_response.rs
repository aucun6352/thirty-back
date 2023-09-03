#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct C005Result {
    pub MSG: String,
    pub CODE: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct C005Row {
    // 폐업일자
    pub CLSBIZ_DT: String,
    // 주소
    pub SITE_ADDR: String,
    // 품목보고(신고)번호
    pub PRDLST_REPORT_NO: String,
    // 보고(신고일)
    pub PRMS_DT: String,
    // 제품명
    pub PRDLST_NM: String,
    // 유통바코드
    pub BAR_CD: String,
    // 유통/소비기한
    pub POG_DAYCNT: String,
    // 식품 유형
    pub PRDLST_DCNM: String,
    // 제조사명
    pub BSSH_NM: String,
    // 생산중단일
    pub END_DT: String,
    // 업종
    pub INDUTY_NM: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct C005Item {
    pub total_count: String,
    pub row: Option<Vec<C005Row>>,
    pub RESULT: C005Result,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct C005 {
    pub C005: C005Item,
}
