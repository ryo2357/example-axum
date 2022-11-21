[Faroukhamadi/pollme](https://github.com/Faroukhamadi/pollme)を参考にした構成

[sral/raffler at 906be90c08ef7a94be52a8f28fa65fbc55f76e99](https://github.com/sral/raffler/tree/906be90c08ef7a94be52a8f28fa65fbc55f76e99)

[rs-src/models.rs at 1fb954c5ef77f65aa095610b88a0f5d674bccff3 · zxk7516/rs-src](https://github.com/zxk7516/rs-src/blob/1fb954c5ef77f65aa095610b88a0f5d674bccff3/url-shorten-axum/src/models.rs)

[sqlx crate でフィールド毎に独自のデコード処理を挟みたい - eagletmt's blog](https://eagletmt.hateblo.jp/entry/2022/08/06/014429)

[[Rust] sqlxを使ってみる - Qiita](https://qiita.com/yagince/items/ffbff7d15420be1fc411)

バインドの方法はこれが良さそう

[sqlx::query_as - Rust](https://docs.rs/sqlx/0.1.1/sqlx/fn.query_as.html)

```Rust
pub fn query_as<DB, T>(
    query: &str
) -> QueryAs<DB, T, <DB as Database>::Arguments> 
where
    DB: Database,
    T: FromRow<<DB as Database>::Row>, 
```

[sqlx::QueryAs - Rust](https://docs.rs/sqlx/0.1.1/sqlx/struct.QueryAs.html)


```Rust
pub async fn fetch_one<E>(self, executor: &'_ mut E) -> Result<R, Error>
where
    E: Executor<Database = DB>, 
```
Execute a query which should return exactly 1 row　⇒ RはSQLのリターン文字列？

Returns crate::Error::NotFound if 0 rows are returned.

Returns crate::Error::FoundMoreThanOne if more than one row is returned

[blog/task.rs at c58c5f0676919c56ba799d86cdc90596f3e4808d · carlosm27/blog](https://github.com/carlosm27/blog/blob/c58c5f0676919c56ba799d86cdc90596f3e4808d/axum_crud_api/src/controllers/task.rs)

```Rust
pub(crate) async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<axum::Json<User>, (StatusCode, String)> {
    sqlx::query_as::<_, User>(&format!(
        "INSERT INTO public.user (username, password) VALUES('{}', '{}') RETURNING id;",
        payload.username, payload.password
    ))
    .fetch_one(&pool)
    .await
    .map(|user| axum::Json(user))
    .map_err(internal_error)
}
```


[hello_axum/user_handler.rs at 7123846bcfdc9328b28c606e03d37de0608af0d0 · Asura7969/hello_axum](https://github.com/Asura7969/hello_axum/blob/7123846bcfdc9328b28c606e03d37de0608af0d0/web/src/handle/user_handler.rs)
```Rust
pub async fn create_user(
    Extension(state): Extension<AppState>,
    Json(payload): Json<User>,
) -> Result<RestJson<String>> {
    sqlx::query("insert into user(id, name, age, email, create_time) values (?,?,?,?,?)")
        .bind(payload.id)
        .bind(payload.name)
        .bind(payload.age)
        .bind(payload.email)
        .bind(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(&state.pool)
        .await
        .map_err(log_error)?;

    Ok(RestJson::ok("".to_string()))
}
```

[backend-modules/admin_service.rs at 7f5b48140b563c2f84a590bbc4adb03e032ea69b · ming900518/backend-modules](https://github.com/ming900518/backend-modules/blob/7f5b48140b563c2f84a590bbc4adb03e032ea69b/user_info/src/admin_service.rs)改行もできる

