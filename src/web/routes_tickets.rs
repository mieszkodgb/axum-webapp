use crate::context::Context;
use crate::error::Result;
use crate::model::{ModelController, Ticket, InputTicket, UpdateTicket};
use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json, Router};


pub fn routes(mc: ModelController) -> Router{
    Router::new()
    .route("/tickets", get(list_ticket))
    .route("/ticket", post(create_ticket))
    .route("/ticket/{id}", get(get_ticket))
    .route("/ticket/{id}/update", patch(update_ticket))
    .route("/ticket/{id}/delete", delete(delete_ticket))
    .with_state(mc)
}


async fn create_ticket(
    State(mc): State<ModelController>,
    context: Context,
    Json(input_ticket): Json<InputTicket>
) -> Result<Json<Ticket>>{

    println!("Create ticket endpoint");
    let ticket = mc.create(context, input_ticket).await?;
    Ok(Json(ticket))
}

async fn update_ticket(
    State(mc): State<ModelController>,
    context: Context,
    Path(id): Path<u64>,
    Json(update_ticket): Json<UpdateTicket>
) -> Result<Json<Ticket>>{
    println!("Update ticket endpoint");
    let ticket = mc.update(context, id, update_ticket).await?;
    Ok(Json(ticket))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    context: Context,
    Path(id): Path<u64>
) -> Result<Json<Ticket>>{
    println!("Delete ticket endpoint");
    let ticket = mc.delete(context, id).await?;
    Ok(Json(ticket))
}

async fn get_ticket(
    State(mc): State<ModelController>,
    context: Context,
    Path(id): Path<u64>
) -> Result<Json<Ticket>>{
    println!("Get ticket endpoint");
    let ticket = mc.get(context, id).await?;
    Ok(Json(ticket))
}

async fn list_ticket(
    State(mc): State<ModelController>,
    context: Context,
) -> Result<Json<Vec<Ticket>>>{
    println!("List tickets endpoint");
    //TODO add filtering
    let tickets = mc.list(context).await?;
    Ok(Json(tickets))
}
