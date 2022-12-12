#[derive(Debug, Deserialize)]
pub struct HistoryData {
    pub page_id:   i16,
    pub link:      String,
}
pub async fn create_history (
    data: Json<HistoryData>,
    req: HttpRequest,
) -> Result<Json<CookieStat>, Error> {
    use crate::models::CookieStat;
    use crate::schema::cookie_stats::dsl::cookie_stats;
    use crate::utils::plus_page_stat;

    let p_link = data.link.clone();

    let _connection = establish_connection();
    let is_cookie_stats_exists = cookie_stats
        .filter(schema::cookie_stats::user_id.eq(p_id))
        .filter(schema::cookie_stats::link.eq(p_link.clone()))
        .select(schema::cookie_stats::id)
        .load::<i32>(&_connection)
        .expect("E.")
        .len() == 0;

    if is_cookie_stats_exists {
        diesel::update(&user)
            .set ((
                schema::cookie_users::height.eq(user.height + p_height),
                schema::cookie_users::seconds.eq(user.seconds + p_seconds),
            ))
            .get_result::<CookieUser>(&_connection)
            .expect("Error.");
    }
    let _res = block(move || CookieStat::create (
        user.id,
    )).await?;
    let res = _res?;

    Ok(Json(res))
}
