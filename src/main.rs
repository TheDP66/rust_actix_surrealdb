use actix_web::{
    get, patch, post,
    web::Path,
    web::{Data, Json},
    App, HttpServer,
};
use rust_actix_surrealdb::{
    db::{database::Database, pizza_data_traits::PizzaDataTrait},
    error::pizza_error::PizzaError,
    models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaUrl},
};
use uuid;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = Database::get_all_pizzas(&db).await;

    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/buypizza")]
async fn buy_pizza(
    body: Json<BuyPizzaRequest>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza =
                Database::add_pizza(&db, Pizza::new(String::from(new_uuid), pizza_name)).await;

            match new_pizza {
                Some(created) => Ok(Json(created)),
                None => Err(PizzaError::PizzaCreationFailure),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailure),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
    update_pizza_url: Path<UpdatePizzaUrl>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = Database::update_pizza(&db, uuid).await;

    match update_result {
        Some(updated_pizza) => Ok(Json(updated_pizza)),
        None => Err(PizzaError::NoSuchPizzaFound),
    }
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
