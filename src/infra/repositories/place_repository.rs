use diesel::prelude::*;
use std::error::Error;

use crate::infra::models::{PlaceRow, NewPlaceRow, EditPlaceRow};
use crate::infra::schema::places;
use crate::domain::place::Place;

use chrono::Utc;

const NAIVE_DATE_TIME_PATTERN: &str =  "%Y-%m-%d %H:%M:%S";

pub fn list_places(conn: &mut SqliteConnection, search: &str) -> Result<Vec<Place>, Box<dyn Error>> {
    let search_like = format!("%{}%", search);

  	let mut places_query= places::table.filter(places::deleted_at.is_null()).into_boxed();

    let filter_expression =  places::name.like(&search_like);

    if let Ok(search_number) = search.parse::<i32>(){
        places_query = places_query.filter(
            filter_expression
                .or(places::id.eq(search_number))
        );
    } else {
        places_query = places_query.filter(filter_expression);
    }
        
    let place_list: Vec<PlaceRow>  = places_query.load(conn).expect("Error loading places");

  	let prods = place_list.into_iter()
    	.map(|place| place.try_into())
		.collect::<Result<Vec<_>, _>>()?;

    Ok(prods)
}

pub fn create_place(conn: &mut SqliteConnection, new_place: NewPlaceRow) -> Result<Place, Box<dyn Error>> {
    diesel::insert_into(places::table)
        .values(&new_place)
        .execute(conn)
        .expect("Failed to insert place");

    let created_place = places::table
        .order(places::id.desc())
        .first::<PlaceRow>(conn)
        .expect("Failed to retrieve created place");

    let place_item = created_place.try_into()?;

    Ok(place_item)
}

pub fn create_places(conn: &mut SqliteConnection, new_places: &[NewPlaceRow]) -> Result<Vec<Place>, Box<dyn Error>> {
    diesel::insert_into(places::table)
        .values(new_places)
        .execute(conn)
        .expect("Failed to insert places");

    let created_places: Vec<PlaceRow> = places::table
        .order(places::id.desc())
        .limit(new_places.len() as i64)
        .load(conn)
        .expect("Failed to retrieve created places");

    let new_places = created_places.into_iter()
        .rev()
        .map(|p| p.try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(new_places)
}

pub fn edit_place(conn: &mut SqliteConnection, place: EditPlaceRow) -> Result<Place, Box<dyn Error>> {
    let place_id = place.id;

    diesel::update(places::table.find(place_id))
        .set((
            &place,
            places::updated_at.eq(Utc::now().format(NAIVE_DATE_TIME_PATTERN).to_string())
        ))
        .execute(conn)
        .expect("Failed to update place");

    
    let updated_place = places::table
        .filter(places::id.eq(place_id))
        .first::<PlaceRow>(conn)
        .expect("Failed to retrieve updated place");

    let place_item = updated_place.try_into()?;

    Ok(place_item)
}

pub fn delete_place(conn: &mut SqliteConnection, place_id: i32) -> Result<bool, Box<dyn Error>> {
    let deleted = diesel::update(places::table.find(place_id))
        .set(places::deleted_at.eq(Some(Utc::now().naive_utc().format(NAIVE_DATE_TIME_PATTERN).to_string())))
        .execute(conn)?;

    Ok(deleted > 0)
}