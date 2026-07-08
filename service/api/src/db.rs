use lambda_http::Error;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

#[allow(dead_code)]
pub(crate) async fn connect(database_url: &str) -> Result<Client, Error> {
    let connector = TlsConnector::builder().build()?;
    let tls = MakeTlsConnector::new(connector);
    let (client, connection) = tokio_postgres::connect(database_url, tls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("database connection error: {error}");
        }
    });

    Ok(client)
}
