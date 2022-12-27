use custom_error::custom_error;

custom_error! {pub EventStoreError
    MissingTargetAggregateIdentifier = "Target Aggregate Identifier must be provided",
}

pub trait Command: Send + Sync {
    fn target_aggregate_identifier(&self) -> Option<&String>;
}
