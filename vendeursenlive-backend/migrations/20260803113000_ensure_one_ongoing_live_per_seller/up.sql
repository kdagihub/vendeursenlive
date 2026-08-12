CREATE UNIQUE INDEX idx_live_sessions_one_ongoing_per_seller
    ON live_sessions(seller_profile_id)
    WHERE status = 'Ongoing';
