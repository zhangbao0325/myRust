use hyper::{Body, Client, Request, Uri};
use hyper_util::client::connect::{HttpConnector, HttpsConnectorBuilder};
use rustls::ClientConfig;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a standard HttpConnector
    let http_connector = HttpConnector::new();

    // Define TLS configuration using rustls
    let mut tls_config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();

    // Add root certificates (for production, use a proper root CA store)
    tls_config
        .root_store
        .add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

    // Create an HTTPS connector using hyper-util and rustls
    let https_connector = HttpsConnectorBuilder::new()
        .with_tls_config(Arc::new(tls_config))
        .https_only()
        .wrap_connector(http_connector)
        .enable_http1()
        .build();

    // Build the client using the connector
    let client: Client<_, hyper::Body> = Client::builder().build(https_connector);

    // Define the URI to request
    let uri: Uri = "https://127.0.0.1:443/test".parse()?;

    // Create the request, setting the Host header to specify SNI
    let request = Request::builder()
        .uri(uri)
        .header("Host", "example.com") // Set the SNI via Host header
        .body(Body::empty())?; // Empty body for GET request

    // Send the request
    let response = client.request(request).await?;

    // Output the response status
    println!("Response status: {}", response.status());

    Ok(())
}
