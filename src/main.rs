use actix_web::{
    get, patch, post,
    web::Path,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use rust_actix_surrealdb::{
    db::database::Database,
    error::pizza_error::PizzaError,
    models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaUrl},
};
use uuid;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;

    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = db
                .add_pizza(Pizza::new(String::from(new_uuid), pizza_name))
                .await;

            match new_pizza {
                Some(created) => {
                    HttpResponse::Ok().body(format!("Created new pizza: {:?}", created))
                }
                None => HttpResponse::Ok().body("Error buying pizza"),
            }
        }
        Err(_) => HttpResponse::Ok().body("pizza name required"),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaUrl>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("updating apizza {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connection to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            // Register service here
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}