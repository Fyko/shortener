use r2d2::Pool;

pub type RedisPool = Pool<redis_async::client::PairedConnection>;