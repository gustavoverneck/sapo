#![allow(unused_imports)]

use std::sync::Mutex;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, guard, http, post, web::{self, get}};
use serde::{Serialize, Deserialize};

struct AppState {
    logged_in: Mutex<bool>,
}

#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().body(r#"<!doctype html>
    <html lang="pt-BR">
    <head>
        <meta charset="utf-8">
        <title>Home</title>
    </head>
    <body>
        <h1>Bem-Vindo(a)!</h1>
        <a href="/login">Login</a>
    </body>
    </html>
    "#)
}


async fn login() -> HttpResponse {
    HttpResponse::Ok().body(r#"<!doctype html>
    <html lang="pt-BR">
    <head>
        <meta charset="utf-8">
        <title>Login</title>
    </head>
    <body>
        <h1>Entrar</h1>
        <form action="/login" method="post">
            <label>Usuário:<br><input type="text" name="username" required></label><br><br>
            <label>Senha:<br><input type="password" name="password" required></label><br><br>
            <button type="submit">Entrar</button>
        </form>
    </body>
    </html>
    "#)
}
#[derive(serde::Deserialize)]
struct LoginData {
    username: String,
    password: String,
}


async fn process_login(
    form: web::Form<LoginData>,
    state: web::Data<AppState>
) -> HttpResponse {
    if form.username == "admin" && form.password == "1234" {
        let mut logged_in = state.logged_in.lock().unwrap();
        *logged_in = true;
        HttpResponse::Found()
            .append_header(("Location", "/main"))
            .finish()
    } else {
        HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Acesso negado!")
    }
}

#[get("/main")]
async fn main_area(state: web::Data<AppState>) -> HttpResponse {
    let logged_in = state.logged_in.lock().unwrap();
    if *logged_in {
        HttpResponse::Ok().body("Área privada: Bem-vindo(a)!")
    } else {
        HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body("<h2> Acesso negado! </h2>")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("API started successfully.");

    // Estado compartilhado entre requisições
    let app_state = web::Data::new(AppState {
        logged_in: Mutex::new(false),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // <-- ADICIONE ISSO
            .service(home)
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .service(main_area)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}