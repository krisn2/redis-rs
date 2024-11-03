use redis::{Connection, RedisResult};

fn store_driver_location(con: &mut Connection, driver_id: &str, longitude: f64, latitude: f64) -> RedisResult<()> {
    redis::cmd("GEOADD")
        .arg("drivers")
        .arg(longitude)
        .arg(latitude)
        .arg(driver_id)
        .exec(con)?;
    println!("Stored location for driver '{}'", driver_id);

    Ok(())
}

fn get_driver_location(con: &mut Connection, driver_id: &str) -> RedisResult<Option<(f64, f64)>> {
    // Query the location using GEOPOS
    let location: Option<Vec<Option<Vec<String>>>> = redis::cmd("GEOPOS")
        .arg("drivers")
        .arg(driver_id)
        .query(con)?;

    // Parse the nested vector structure to extract longitude and latitude
    if let Some(Some(coords)) = location.and_then(|v| v.into_iter().next()) {
        if coords.len() == 2 {
            if let (Ok(longitude), Ok(latitude)) = (coords[0].parse::<f64>(), coords[1].parse::<f64>()) {
                return Ok(Some((longitude, latitude)));
            }
        }
    }

    Ok(None)
}

fn main() -> RedisResult<()> {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Store the driver's location
    store_driver_location(&mut con, "driver_1", 12.9716, 77.5946)?;

    // Get the location of the driver
    if let Some((longitude, latitude)) = get_driver_location(&mut con, "driver_1")? {
        println!("Driver location: longitude = {}, latitude = {}", longitude, latitude);
    } else {
        println!("Driver location not found");
    }

    Ok(())
}
