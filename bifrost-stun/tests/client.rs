use std::time::Duration;

use bifrost_stun::client::ClientBuilder;

#[test]
fn test_binding() {
    tokio_test::block_on(async {
        let mut client = ClientBuilder::new()
            .bind("0.0.0.0:0")
            // .connect("127.0.0.1:3478")
            .connect("stun.l.google.com:19302")
            .rto(Duration::from_secs(5))
            .build()
            .await
            .unwrap();

        let addr = client.binding().await.unwrap();
        println!("My addr: {:?}", addr);
    });
}
