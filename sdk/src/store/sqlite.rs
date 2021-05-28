// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

use super::StoreFactory;

/// A `StoryFactory` backed by a SQLite database.
pub struct SqliteStoreFactory {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl SqliteStoreFactory {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }
}

impl StoreFactory for SqliteStoreFactory {
    fn get_grid_commit_store(&self) -> Box<dyn crate::commits::CommitStore> {
        Box::new(crate::commits::DieselCommitStore::new(self.pool.clone()))
    }

    #[cfg(feature = "pike")]
    fn get_grid_pike_store(&self) -> Box<dyn crate::pike::PikeStore> {
        Box::new(crate::pike::DieselPikeStore::new(self.pool.clone()))
    }

    #[cfg(all(feature = "diesel", feature = "location-store-sqlite"))]
    fn get_grid_location_store(&self) -> Box<dyn crate::location::LocationStore> {
        Box::new(crate::location::DieselLocationStore::new(self.pool.clone()))
    }

    #[cfg(all(feature = "diesel", feature = "product-store-sqlite"))]
    fn get_grid_product_store(&self) -> Box<dyn crate::product::ProductStore> {
        Box::new(crate::product::DieselProductStore::new(self.pool.clone()))
    }

    #[cfg(all(feature = "diesel", feature = "schema-store-sqlite"))]
    fn get_grid_schema_store(&self) -> Box<dyn crate::schemas::SchemaStore> {
        Box::new(crate::schemas::DieselSchemaStore::new(self.pool.clone()))
    }

    #[cfg(feature = "track-and-trace")]
    fn get_grid_track_and_trace_store(
        &self,
    ) -> Box<dyn crate::track_and_trace::TrackAndTraceStore> {
        Box::new(crate::track_and_trace::DieselTrackAndTraceStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "batch-store")]
    fn get_batch_store(&self) -> Box<dyn crate::batches::BatchStore> {
        Box::new(crate::batches::DieselBatchStore::new(self.pool.clone()))
    }
}
