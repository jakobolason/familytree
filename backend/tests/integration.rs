use familyserver_backend::{
    app::App,
    views::auth::{LoginMethods, PasswordLoginParams},
};
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_health() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/_health").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn fail_at_login() {
    request::<App, _, _>(|request, _ctx| async move {
        let test = PasswordLoginParams {
            email: "bad@email.com".to_string(),
            password: "123".to_string(),
        };
        let body = LoginMethods::Password(test);
        let res = request.post("/api/auth/login").json(&body).await;
        assert_eq!(res.status_code(), 401);
    })
    .await;
}
