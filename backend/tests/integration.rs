use familyserver_backend::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_magic_link() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/magic/").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
