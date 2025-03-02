use crate::{context::Context, error::{Error, Result}};
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ticket{
    pub id: u64,
    pub title: String,
    pub content: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub user_id: u64

}

#[derive(Deserialize, Debug)]
pub struct InputTicket{
    pub title: String,
    pub content: String
}

#[derive(Deserialize, Debug)]
pub struct UpdateTicket{
    pub title: Option<String>,
    pub content: Option<String>
}


// Controller
#[derive(Clone)]
pub struct  ModelController {
    // Storing tickets as a vector in the memory
    // TODO: remplace this with a DB connection using db pool
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

//Contstructor
impl ModelController{
    pub async fn new() -> Result<Self>{
        Ok(Self{
            ticket_store: Arc::default()
        })
    }
}

impl ModelController{
    pub async fn create(
        &self,
        context: Context,
        input_ticket: InputTicket
    ) -> Result<Ticket>{

        // take control of the ARC Mutex to edit it 
        let mut store = self.ticket_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket{
            id,
            title: input_ticket.title,
            content: input_ticket.content,
            create_at: chrono::offset::Local::now().to_utc(),
            update_at: chrono::offset::Local::now().to_utc(),
            user_id: context.user_id()
        };
        store.push(Some(ticket.clone()));
        
        Ok(ticket)
    }

    pub async fn get(
        &self,
        _context: Context,
        id: u64
    ) -> Result<Ticket>{

        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.clone());

        ticket.ok_or(Error::TicketNotFound { id })
    }

    pub async fn list(
        &self,
        _context: Context,
    ) -> Result<Vec<Ticket>>{

        let store = self.ticket_store.lock().unwrap();
        let tickets: Vec<Ticket> = store.iter().filter_map(|t: &Option<Ticket>| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn update(
        &self,
        _context: Context,
        id: u64,
        update_ticket: UpdateTicket
    ) -> Result<Ticket>{

        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize)
                .ok_or(Error::TicketNotFound { id })?;

        match ticket {
            Some(t) => {
                if let Some(new_title) = update_ticket.title {
                    t.title = new_title;
                }
                if let Some(new_content) = update_ticket.content {
                    t.content = new_content;
                }
                t.update_at = chrono::offset::Local::now().to_utc();

                Ok(t.clone())
            },
            None => Err(Error::TicketNotFound { id })
        }
    }

    pub async fn delete(
        &self,
        _context: Context,
        id: u64
    ) -> Result<Ticket>{
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketNotFound { id })
    }

}

