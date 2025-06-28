#[actix_rt::test]
async fn test_health() {
    let resp = reqwest::get("http://localhost:8000/health").await.unwrap();
    assert_eq!(resp.status(), 200);
}
