use actix_service::forward_ready;
use actix_web::body::{self, EitherBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::Error;
use actix_web::HttpResponse;
use futures::future::{ok, LocalBoxFuture, Ready};
use crate::models::UserClaim;

const IGNORE_ROUTES: [&str; 1] = ["/login"]; 

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);


    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut validation_auth = false;


        let headers = req.headers().clone();
        let path = req.path().to_owned();
    
        // Bypass routes that don't need authentication
        for routes in IGNORE_ROUTES {
            if path.starts_with(routes) {
                validation_auth = true;
            }
        }
    
        // check if the request has a valid token or header not present in the request
        if !validation_auth {

            // get the header
            let header_get_authorization = headers.get(header::AUTHORIZATION);
            match header_get_authorization {
                Some(token_bearer) => {
                    let token = token_bearer.to_str().unwrap();
                    // extract the token from the header 
                    if token.to_lowercase().starts_with("bearer") {
                        // remove the bearer from the token
                        let jwt_token = token[7..].trim();
                        // validate the token
                        let validate = UserClaim::validate_token(jwt_token);
                        match validate {
                            Ok(_) => {
                                validation_auth = true;
                            },
                            Err(_) => {
                                // if the token is not valid, return 401
                                let res = HttpResponse::Unauthorized()
                                    .body("Invalid Token")
                                    .map_into_right_body();

                                return Box::pin(async { Ok(req.into_response(res)) });
                            }
                        }
                        
                    }else{
                      
                        // If bearer argument is not present, return 401
                        // Authorization: <Token> -- Bearer is not present

                        let res = HttpResponse::Unauthorized()
                        .insert_header(("WWW-Authenticate", "Bearer"))
                        .body("Unauthorized")
                        .map_into_right_body();

                        eprintln!("Bearer argument not found");

                        return Box::pin(async { Ok(req.into_response(res)) });

                    }
                }
                None => {

                    // If header Authorization not found, return 401
                    let res = HttpResponse::Unauthorized()
                        .insert_header(("WWW-Authenticate", "Bearer"))
                        .body("Unauthorized")
                        .map_into_right_body();
                    eprintln!("Header Authorization not found");
                    return Box::pin(async { Ok(req.into_response(res)) });
                }
            }
        }
    
        let res = self.service.call(req);
    
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }    
}
