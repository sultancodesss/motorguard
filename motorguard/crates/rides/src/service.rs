use motorguard_core::{
    models::{Ride, RidePoint},
    types::{RideId, RidePointId, RideStatus, UserId},
    AppError, Result,
};
use motorguard_database::RideRepository;
use motorguard_location::LocationService;
use sqlx::SqlitePool;
use tracing::info;

use crate::{safety_score::calculate_safety_score, statistics::RideStatistics};

pub struct RideService {
    pool: SqlitePool,
    location_service: LocationService,
}

impl RideService {
    pub fn new(pool: SqlitePool, location_service: LocationService) -> Self {
        Self { pool, location_service }
    }

    /// Create a new pending ride for the user.
    pub async fn create_ride(&self, user_id: UserId, name: Option<String>) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = Ride::new(user_id, name);
        let created = repo.create(&ride).await?;
        info!("Created ride {} for user {}", created.id, user_id);
        Ok(created)
    }

    /// Transition a pending ride to active.
    pub async fn start_ride(&self, ride_id: RideId, user_id: UserId) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;

        if ride.status == RideStatus::Active {
            return Err(AppError::RideAlreadyActive);
        }
        if ride.status == RideStatus::Completed {
            return Err(AppError::RideAlreadyCompleted);
        }

        repo.start_ride(ride_id).await?;
        let updated = repo.find_by_id(ride_id).await?;
        info!("Started ride {}", ride_id);
        Ok(updated)
    }

    /// Pause an active ride.
    pub async fn pause_ride(&self, ride_id: RideId, user_id: UserId) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;
        self.assert_active(&ride)?;

        repo.set_status(ride_id, RideStatus::Paused).await?;
        repo.find_by_id(ride_id).await
    }

    /// Resume a paused ride.
    pub async fn resume_ride(&self, ride_id: RideId, user_id: UserId) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;

        if ride.status != RideStatus::Paused {
            return Err(AppError::RideNotActive);
        }

        repo.set_status(ride_id, RideStatus::Active).await?;
        repo.find_by_id(ride_id).await
    }

    /// Finish an active ride, compute statistics and safety score.
    pub async fn finish_ride(&self, ride_id: RideId, user_id: UserId) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;

        if ride.status != RideStatus::Active && ride.status != RideStatus::Paused {
            return Err(AppError::RideNotActive);
        }

        let points = repo.get_points(ride_id).await?;
        let stats = RideStatistics::from_points(&points);
        let safety_score = calculate_safety_score(&points, &stats);

        let started_at = ride.started_at.unwrap_or_else(chrono::Utc::now);
        let duration_seconds = (chrono::Utc::now() - started_at).num_seconds().max(0);

        repo.finish_ride(
            ride_id,
            stats.distance_miles,
            duration_seconds,
            stats.average_speed_mph,
            stats.max_speed_mph,
            safety_score,
        )
        .await?;

        let finished = repo.find_by_id(ride_id).await?;
        info!(
            "Finished ride {} — {:.1}mi in {}s, score: {:.0}",
            ride_id, stats.distance_miles, duration_seconds, safety_score
        );
        Ok(finished)
    }

    /// Add GPS points to an active ride.
    pub async fn add_points(
        &self,
        ride_id: RideId,
        user_id: UserId,
        locations: Vec<motorguard_core::models::Location>,
    ) -> Result<usize> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;
        self.assert_active(&ride)?;

        let mut count = 0;
        for loc in locations {
            if !loc.is_valid_coordinates() {
                continue; // Skip invalid GPS points silently
            }
            // Skip points with very poor accuracy (> 100m)
            if !loc.is_accurate_enough(100.0) {
                continue;
            }
            let point = RidePoint {
                id: RidePointId::new(),
                ride_id,
                latitude: loc.latitude,
                longitude: loc.longitude,
                altitude: loc.altitude,
                speed: loc.speed,
                accuracy: loc.accuracy,
                recorded_at: loc.timestamp,
            };
            repo.insert_point(&point).await?;
            count += 1;
        }
        Ok(count)
    }

    pub async fn get_ride(&self, ride_id: RideId, user_id: UserId) -> Result<Ride> {
        let repo = RideRepository::new(&self.pool);
        let ride = repo.find_by_id(ride_id).await?;
        self.assert_owner(&ride, user_id)?;
        Ok(ride)
    }

    pub async fn list_rides(
        &self,
        user_id: UserId,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<Ride>> {
        let repo = RideRepository::new(&self.pool);
        let offset = (page - 1) * per_page;
        repo.list_by_user(user_id, per_page, offset).await
    }

    // ── Private ──────────────────────────────────────────────────────────────

    fn assert_owner(&self, ride: &Ride, user_id: UserId) -> Result<()> {
        if ride.user_id != user_id {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }

    fn assert_active(&self, ride: &Ride) -> Result<()> {
        if ride.status != RideStatus::Active {
            Err(AppError::RideNotActive)
        } else {
            Ok(())
        }
    }
}
