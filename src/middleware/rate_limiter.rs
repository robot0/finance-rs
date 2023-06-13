// use actix_service::Service;
// use actix_web::{Error, HttpResponse, dev::{ServiceRequest, ServiceResponse}};
// use governor::{Quota, RateLimiter, state::InMemoryState, clock::DefaultClock};
// use std::num::NonZeroU32;
// use std::time::Duration;
//
// pub fn rate_limiter() -> RateLimiter<(), InMemoryState, DefaultClock> {
//     let rate = NonZeroU32::new(1).unwrap(); // Allow 1 request...
//     let per = Duration::from_secs(1); // ...per second
//     let quota = Quota::per_second(rate).allow_burst(NonZeroU32::new(5).unwrap()); // Allow a burst of 5 requests
//     // Error : mismatched types
//     // Error : expected `RateLimiter<(), InMemoryState, DefaultClock>`, found `RateLimiter<NotKeyed, InMemoryState, DefaultClock>`
//     RateLimiter::direct(quota)
// }
//
// pub async fn rate_limiter_middleware<S>(
//     req: ServiceRequest,
//     // Error : this trait takes 1 generic argument but 0 generic arguments were supplied
//     // Error : The value of the associated type `Future` (from trait `Service`) must be specified
//     // Error : Wrong number of type arguments: expected 1, found 0
//     // srv: &dyn Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
//     srv: &S,
//     limiter: &RateLimiter<(), InMemoryState, DefaultClock>,
// ) -> Result<ServiceResponse, Error> {
//     if limiter.check_key(&()).is_ok() {
//         Ok(srv.call(req).await?)
//     } else {
//         Ok(req.error_response(HttpResponse::TooManyRequests()))
//     }
// }
