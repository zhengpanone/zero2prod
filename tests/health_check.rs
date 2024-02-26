use std::net::TcpListener;
use zero2prod::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let url = format!("{}/health_check", &address);
    // Act
    
    let response = client
        // Use the returned application address
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    println!("  ===== {:?}",response)
    // assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}