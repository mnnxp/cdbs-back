// use crate::cli_args::Opt;
// use crate::database::PooledConnection;
// use crate::jwt::model::DecodedToken;
// use crate::models::user::model::LoggedUser;
//
// use std::sync::Arc;
//
// #[derive(Clone)]
// pub struct Context {
//     pub opt: Opt,
//     pub db: Arc<PooledConnection>,
//     pub user: LoggedUser,
//     pub token: DecodedToken,
// }
//
// impl Context {
//     pub async fn new(token: DecodedToken, user: LoggedUser, pool: PooledConnection, opt: Opt) -> Self {
//         Self {
//             opt,
//             token,
//             user,
//             db: Arc::new(pool),
//         }
//     }
// }
