#!(allow(unused))
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use sqlx::SqlitePool;
use uuid::Uuid;

fn seed_users(pool: Pool<Sqlite>)
{
    let users = vec!{
        ("valyisandor", "Temp1234"),
        ("valyibalint", "Temp55565"),
        ("valyiboldizsar", "Temp99511234"),
        ("makonyijudit", "Temp8987656"),
        ("moresandor", "Temp1233333"),
        ("szathmaryzsuzsanna", "Temp76767584"),
        ("gesztizsuzsanna", "Temp555546322"),
        ("erdeilaszlo", "Temp666551991"),
        ("erdeiveronika", "Temp9999999"),
        ("moni", "Temp876765543")
    };

    for (username, temp_password) in users {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(temp_password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            "INSERT INTO users (id, username, password_hash, must_change_password) VALUES (?, ?, ?, 1)",
            id, username, hash
        )
        .execute(&pool)
        .await
        .unwrap();

        println!("Created {username} — temp password: {temp_password}");
    }

}