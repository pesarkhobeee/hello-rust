use super::*;

#[tokio::test]
async fn test_generate_output() {
    let opt = Opt {
        urls: vec![
            "https://github.com/Restfulness/Restfulness-flutter-app/pull/36".to_string(),
            "https://github.com/Restfulness/Restfulness-flutter-app/pull/35".to_string(),
        ],
    };
    let output = generate_output(opt)
        .await
        .expect("expect success unwrap output in test");

    assert_eq!(output.failed_qty, 1);
    assert_eq!(output.reports.len(), 2);
}
