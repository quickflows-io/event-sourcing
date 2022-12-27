use crate::Error;

use crate::aggregate::Aggregate;
use crate::snapshot::envelope::SnapshotEnvelope;

#[async_trait::async_trait]
pub trait SnapshotStore<A>: Sized + Send + Sync + Clone
where
    A: Aggregate,
{
    async fn read(&self, aggregate_id: &String) -> Result<Option<SnapshotEnvelope<A>>, Error>;
    async fn persist(&self, snapshot_envelope: &SnapshotEnvelope<A>) -> Result<(), Error>;
}
