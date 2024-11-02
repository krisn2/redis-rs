use redis::{Commands, Connection};

//fn do_something(con: &mut Connection) -> redis::RedisResult<()> {
    // Use the raw Redis command style with `cmd` and `execute`
  //redis::cmd("SET").arg("my_key").arg(42).execute(con);
//    println!("Set key 'my_key' to 42");

  //  Ok(())
//}

fn store_location(con: &mut Connection,
    key: &str, name: &str, longitude:f64, 
    latitude:f64) -> redis::RedisResult<()> {

    redis::cmd("GEOADD")
        .arg(key)
        .arg(longitude)
        .arg(latitude)
        .arg(name)
        .execute(con);

        println!("Stored location for '{}'", name);

        Ok(())
}

fn main() -> redis::RedisResult<()> {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Call the `do_something` function with the connection
   // do_something(&mut con)?;

    // Optionally, retrieve and print the value to verify
    //let value: i32 = con.get("my_key")?;
    //println!("The value of 'my_key' is: {}", value);

    store_location(&mut con, "location", "driver", 12.9716, 77.5946)?;

    Ok(())
}
