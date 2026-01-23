use actix_web::{test, web, App};
use rust_server_petstore::{config_app, db::MongoDb, petmodel::Pet, usermodel::User};
use std::env;

async fn get_test_db() -> MongoDb {
    let mongo_url = env::var("DATABASE_URI")
        .unwrap_or_else(|_| "mongodb://root:example@localhost:27017/?authSource=admin".to_string());
    
    let client = mongodb::Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB for testing");
    let db = client.database("petstore_test");

    MongoDb {
        pet_collection: db.collection::<Pet>("pets"),
        user_collection: db.collection::<User>("users"),
    }
}

#[actix_web::test]
async fn test_get_pets_empty() {
    let mongo_db = get_test_db().await;
    let app_state = web::Data::new(mongo_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(config_app)
    ).await;

    let req = test::TestRequest::get().uri("/v2/pet").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_add_pet() {
    let mongo_db = get_test_db().await;
    let app_state = web::Data::new(mongo_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(config_app)
    ).await;

    let new_pet = Pet {
        id: 999,
        name: "TestPet".to_string(),
        category: rust_server_petstore::petmodel::Category { id: 1, name: "TestCategory".to_string() },
        photo_urls: vec![],
        tags: vec![],
        status: "available".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/v2/pet")
        .set_json(&new_pet)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status() == 201);

    // Cleanup
    let _ = app_state.delete_pet_by_id("999").await;
}

#[actix_web::test]
async fn test_user_flow() {
    let mongo_db = get_test_db().await;
    let app_state = web::Data::new(mongo_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(config_app)
    ).await;

    let test_user = User {
        id: 888,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // 1. Add User
    let req = test::TestRequest::post()
        .uri("/v2/user")
        .set_json(&test_user)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 2. Login
    let req = test::TestRequest::get()
        .uri("/v2/user/login?username=testuser&password=password123")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. Get User
    let req = test::TestRequest::get()
        .uri("/v2/user/testuser")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 4. Logout
    let req = test::TestRequest::get()
        .uri("/v2/user/logout?username=testuser")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Cleanup
    let _ = app_state.delete_user_by_username("testuser").await;
}

#[actix_web::test]
async fn test_find_pets() {
    let mongo_db = get_test_db().await;
    let app_state = web::Data::new(mongo_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(config_app)
    ).await;

    // Search by status
    let req = test::TestRequest::get()
        .uri("/v2/pet/findByStatus?status=available")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Search by tags
    let req = test::TestRequest::get()
        .uri("/v2/pet/findByTags?tags=tag1,tag2")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
