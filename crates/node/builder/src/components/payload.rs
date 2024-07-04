//! Payload service component for the node builder.

use crate::{BuilderContext, FullNodeTypes};
use reth_payload_builder::PayloadBuilderHandle;
use reth_transaction_pool::TransactionPool;
use std::future::Future;

/// A type that knows how to spawn the payload service.
pub trait PayloadServiceBuilder<Node: FullNodeTypes, Pool: TransactionPool>: Send {
    /// Spawns the payload service and returns the handle to it.
    ///
    /// The [`BuilderContext`] is provided to allow access to the node's configuration.
    fn spawn_payload_service(
        self,
        ctx: &BuilderContext<Node>,
        pool: Pool,
    ) -> impl Future<Output = eyre::Result<PayloadBuilderHandle<Node::EngineTypes>>> + Send;
}

impl<Node, F, Fut, Pool> PayloadServiceBuilder<Node, Pool> for F
where
    Node: FullNodeTypes,
    Pool: TransactionPool,
    F: Fn(&BuilderContext<Node>, Pool) -> Fut + Send,
    Fut: Future<Output = eyre::Result<PayloadBuilderHandle<Node::EngineTypes>>> + Send,
{
    fn spawn_payload_service(
        self,
        ctx: &BuilderContext<Node>,
        pool: Pool,
    ) -> impl Future<Output = eyre::Result<PayloadBuilderHandle<Node::EngineTypes>>> + Send {
        self(ctx, pool)
    }
}
