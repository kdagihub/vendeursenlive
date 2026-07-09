CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(255),
    avatar_url VARCHAR(500),
    status VARCHAR(50) NOT NULL DEFAULT 'Active',
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT users_status_check CHECK (status IN ('Active', 'Disabled'))
);

CREATE TABLE user_auth_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_subject VARCHAR(255),
    email VARCHAR(255),
    phone_number VARCHAR(50),
    password_hash VARCHAR(255),
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    phone_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT user_auth_identities_provider_check CHECK (
        provider IN ('Email', 'Phone', 'Google', 'TikTok')
    ),
    CONSTRAINT user_auth_identities_required_identifier_check CHECK (
        (provider = 'Email' AND email IS NOT NULL)
        OR (provider = 'Phone' AND phone_number IS NOT NULL)
        OR (provider IN ('Google', 'TikTok') AND provider_subject IS NOT NULL)
    )
);

CREATE TABLE auth_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR(255) NOT NULL,
    user_agent VARCHAR(500),
    ip_address VARCHAR(64),
    revoked_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE customer_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    default_location VARCHAR(255),
    preferred_payment_method VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE seller_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    shop_name VARCHAR(255) NOT NULL,
    payment_link VARCHAR(255),
    trial_ends_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE live_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_profile_id UUID NOT NULL REFERENCES seller_profiles(id) ON DELETE CASCADE,
    tiktok_url VARCHAR(500) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    CONSTRAINT live_sessions_status_check CHECK (status IN ('Ongoing', 'Ended'))
);

CREATE TABLE ephemeral_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    live_session_id UUID NOT NULL REFERENCES live_sessions(id) ON DELETE CASCADE,
    image_url VARCHAR(500) NOT NULL,
    price_fcfa INTEGER NOT NULL,
    description VARCHAR(255) NOT NULL,
    is_retained BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ephemeral_products_price_check CHECK (price_fcfa > 0)
);

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES ephemeral_products(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    customer_name VARCHAR(255) NOT NULL,
    customer_phone VARCHAR(50) NOT NULL,
    customer_location VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    proof_image_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT orders_status_check CHECK (
        status IN ('Pending', 'ProofSubmitted', 'Accepted', 'Rejected')
    )
);

CREATE UNIQUE INDEX idx_user_auth_identities_provider_subject
    ON user_auth_identities(provider, provider_subject)
    WHERE provider_subject IS NOT NULL;
CREATE UNIQUE INDEX idx_user_auth_identities_email
    ON user_auth_identities(LOWER(email))
    WHERE email IS NOT NULL;
CREATE UNIQUE INDEX idx_user_auth_identities_phone_number
    ON user_auth_identities(phone_number)
    WHERE phone_number IS NOT NULL;
CREATE INDEX idx_user_auth_identities_user_id ON user_auth_identities(user_id);
CREATE INDEX idx_auth_sessions_user_id ON auth_sessions(user_id);
CREATE INDEX idx_auth_sessions_expires_at ON auth_sessions(expires_at);
CREATE INDEX idx_customer_profiles_user_id ON customer_profiles(user_id);
CREATE INDEX idx_seller_profiles_user_id ON seller_profiles(user_id);
CREATE INDEX idx_live_sessions_seller_profile_id ON live_sessions(seller_profile_id);
CREATE INDEX idx_live_sessions_status ON live_sessions(status);
CREATE INDEX idx_ephemeral_products_live_session_id ON ephemeral_products(live_session_id);
CREATE INDEX idx_ephemeral_products_created_at ON ephemeral_products(created_at);
CREATE INDEX idx_orders_product_id ON orders(product_id);
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
