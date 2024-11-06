use movie_server::service::{app_router, Movie};

const ENDPOINT: &str = "0.0.0.0:3000";

#[tokio::test]
async fn test_movies() {
    let client = reqwest::Client::new();
    let app = app_router();

    let listener = tokio::net::TcpListener::bind(ENDPOINT).await.unwrap();

    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let result = client
        .get(format!("http://{}/movie/{}", ENDPOINT, "a_movie"))
        .send()
        .await
        .unwrap();

    let body: Option<Movie> = result.json().await.unwrap();
    assert!(body.is_none(), "Movie should not exist.");

    let movie = Movie {
        id: "movie-1".into(),
        name: "a-movie".into(),
        year: 2022,
        was_good: false,
    };

    let result = client
        .post(format!("http://{}/movie", ENDPOINT))
        .json(&movie)
        .send()
        .await
        .unwrap();

    assert!(result.status().is_success());

    let result = client
        .get(format!("http://{}/movie/{}", ENDPOINT, "movie-1"))
        .send()
        .await
        .unwrap();

    let body: Option<Movie> = result.json().await.unwrap();
    assert!(body.is_some(), "Movie should exist.");

    assert_eq!(movie, body.unwrap());
}

// REPO NAME: 0039050
