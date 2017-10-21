CREATE TABLE events (
    source_id UUID NOT NULL,
    sequence_number BIGINT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (source_id, sequence_number)
);