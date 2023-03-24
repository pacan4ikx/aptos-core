-- Your SQL goes here
-- objects, basically normalizing ObjectCore
CREATE TABLE objects (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,
  object_address VARCHAR(66) NOT NULL,
  owner_address VARCHAR(66),
  state_key_hash VARCHAR(66) NOT NULL,
  guid_creation_num NUMERIC,
  allow_ungated_transfer BOOLEAN,
  is_deleted BOOLEAN NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX o_owner_idx ON objects (owner_address);
CREATE INDEX o_object_skh_idx ON objects (object_address, state_key_hash);
CREATE INDEX o_skh_idx ON objects (state_key_hash);
CREATE INDEX o_insat_idx ON objects (inserted_at);
-- latest instance of objects
CREATE TABLE current_objects (
  object_address VARCHAR(66) UNIQUE PRIMARY KEY NOT NULL,
  owner_address VARCHAR(66),
  state_key_hash VARCHAR(66) NOT NULL,
  allow_ungated_transfer BOOLEAN,
  last_guid_creation_num NUMERIC,
  last_transaction_version BIGINT NOT NULL,
  is_deleted BOOLEAN NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX co_owner_idx ON current_objects (owner_address);
CREATE INDEX co_object_skh_idx ON current_objects (object_address, state_key_hash);
CREATE INDEX co_skh_idx ON current_objects (state_key_hash);
CREATE INDEX co_insat_idx ON current_objects (inserted_at);
ALTER TABLE move_resources
ADD COLUMN state_key_hash VARCHAR(66) NOT NULL DEFAULT '';
CREATE INDEX mr_skh_idx ON move_resources (state_key_hash);