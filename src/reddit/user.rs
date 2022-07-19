use crate::response::AccessToken;

///

#[derive(Debug)]
pub(crate) enum User {
    Anonymous {
        user_agent: String,
    },
    Authenticated {
        user_agent: String,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
        access_token: AccessToken,
    },
}

impl User {
    pub(crate) fn new_anonymous(user_agent: String) -> User {
        User::Anonymous { user_agent }
    }

    pub(crate) fn new_authenticated(
        user_agent: String,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
        access_token: AccessToken,
    ) -> User {
        User::Authenticated {
            user_agent,
            client_id,
            client_secret,
            username,
            password,
            access_token,
        }
    }

    pub(crate) fn user_agent(&self) -> String {
        match self {
            User::Anonymous { user_agent } => user_agent.to_string(),
            User::Authenticated {
                user_agent,
                client_id: _,
                client_secret: _,
                username: _,
                password: _,
                access_token: _,
            } => user_agent.to_string(),
        }
    }

    pub(crate) fn username(&self) -> Option<String> {
        match self {
            User::Anonymous { user_agent: _ } => None,
            User::Authenticated {
                user_agent: _,
                client_id: _,
                client_secret: _,
                username,
                password: _,
                access_token: _,
            } => Some(username).cloned(),
        }
    }

    pub(crate) fn access_token(&self) -> Option<AccessToken> {
        match self {
            User::Anonymous { user_agent: _ } => None,
            User::Authenticated {
                user_agent: _,
                client_id: _,
                client_secret: _,
                username: _,
                password: _,
                access_token,
            } => Some(access_token).cloned(),
        }
    }
}

// ///
// // TODO: Consider change to enum instead.
// pub trait User {
//     fn is_auth(&self) -> bool;
// }
//
// ///
// impl std::fmt::Debug for dyn User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let user = if self.is_auth() {
//             "Authenticated User"
//         } else {
//             "Anonymous User"
//         };
//         write!(f, "{}", user)
//     }
// }
//
// ///
// pub(crate) struct AnonymousUser;
//
// ///
// impl AnonymousUser {
//     ///
//     pub(crate) fn new_user() -> std::sync::Arc<parking_lot::Mutex<Box<dyn User>>> {
//         std::sync::Arc::new(parking_lot::Mutex::new(
//             Box::new(AnonymousUser) as Box<dyn User>
//         ))
//     }
// }
//
// ///
// impl User for AnonymousUser {
//     ///
//     fn is_auth(&self) -> bool {
//         false
//     }
// }
//
// ///
// pub(crate) struct AuthenticatedUser {
//     client_id: String,
//     client_secret: String,
//     username: String,
//     password: String,
//     access_token: AccessToken,
// }
//
// ///
// impl User for AuthenticatedUser {
//     ///
//     fn is_auth(&self) -> bool {
//         true
//     }
// }
//
// ///
// impl AuthenticatedUser {
//     ///
//     pub(crate) fn new_user(
//         client_id: String,
//         client_secret: String,
//         username: String,
//         password: String,
//         access_token: AccessToken,
//     ) -> std::sync::Arc<parking_lot::Mutex<Box<dyn User>>> {
//         let user = AuthenticatedUser {
//             client_id,
//             client_secret,
//             username,
//             password,
//             access_token,
//         };
//         // or type inference: std::sync::Arc::new(parking_lot::Mutex::new(Box::new(user) as _))
//         std::sync::Arc::new(parking_lot::Mutex::new(Box::new(user) as Box<dyn User>))
//     }
//
//
//
//
// }
