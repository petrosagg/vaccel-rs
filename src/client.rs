use std::marker::PhantomData;

use tarpc::client;
use tarpc::context;
use tarpc::server::{BaseChannel, Channel};
use tarpc::transport::channel;

use crate::resource::{Resource, ResourceRef};
use crate::server::{Server, Vaccel, VaccelClient};

pub enum ClientConfig {
    /// All RPC calls will be performed over an in-memory mpsc channel
    Local,
    /// All RPC calls will be performed over a vsock socket
    Remote,
}

pub struct Client {
    inner: VaccelClient,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        match config {
            ClientConfig::Local => {
                let (client_transport, server_transport) = channel::unbounded();
                let server = BaseChannel::with_defaults(server_transport);
                tokio::spawn(server.execute(Server::default().serve()));

                Self {
                    inner: VaccelClient::new(client::Config::default(), client_transport).spawn(),
                }
            }
            ClientConfig::Remote => {
                todo!("create vsock transport")
            }
        }
    }

    pub async fn register<T: Into<Resource>>(&self, data: T) -> ResourceRef<T> {
        let id = self
            .inner
            .register_resource(context::current(), data.into())
            .await
            .unwrap();
        ResourceRef {
            id,
            marker: PhantomData,
        }
    }

    pub async fn length(&self, model: ResourceRef<String>) -> usize {
        self.inner.length(context::current(), model).await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn basic_client() {
        let client = Client::new(ClientConfig::Local);

        let resource = client.register("cat".to_string()).await;
        let resp = client.length(resource).await;
        assert_eq!(resp, 3);

        let resource = client.register("foobar".to_string()).await;
        let resp = client.length(resource).await;
        assert_eq!(resp, 6);
    }
}
