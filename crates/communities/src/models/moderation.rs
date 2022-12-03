use crate::schema;
use crate::schema::{
    moderateds,
    moderated_reports,
    moderated_penalties,
    moderated_logs,
    owner_services,
    owners,
    owner_services_items,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    get_community,
    EditTokenPageResp,
};

#[derive(Serialize)]
pub struct TokenServiceJson {
    pub id:   i32,
    pub name: String,
}

/*
Moderated
Статус
1 На рассмотрении
2 Объект заморожен
3 Объект закрыт
4 Объекту присвоен баннер
5 Отвергнутый
*/
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Moderated {
    pub id:           i32,
    pub description:  Option<String>,
    pub verified:     bool,
    pub status:       i16,
    pub community_id: i32,
    pub created:      chrono::NaiveDateTime,
    pub count:        i32,
}

impl Moderated {
    pub fn get_or_create_moderated_object (
        community_id: i32,
    ) -> Moderated {
        use crate::schema::moderateds::dsl::moderateds;

        let _connection = establish_connection();
        let some_moderateds = moderateds
            .filter(schema::moderateds::community_id.eq(community_id))
            .first::<Moderated>(&_connection);
        if some_moderateds.is_ok() {
            return some_moderateds.expect("E");
        }
        else {
            let new_form = NewModerated {
                description:  None,
                verified:     false,
                status:       1,
                community_id: community_id,
                created:      chrono::Local::now().naive_utc(),
                count:        0,
            };
            let _new = diesel::insert_into(schema::moderateds::table)
                .values(&new_form)
                .get_result::<Moderated>(&_connection)
                .expect("Error.");
            return _new;
        }
    }
    pub fn reports_count_ru(&self) -> String {
        use crate::utils::get_count_for_ru;

        return get_count_for_ru (
            self.count,
            " жалоба".to_string(),
            " жалобы".to_string(),
            " жалоб".to_string(),
        );
    }
    pub fn is_verified(&self) -> bool {
        return self.verified;
    }
    pub fn is_suspend(&self) -> bool {
        return self.status == 2;
    }
    pub fn is_pending(&self) -> bool {
        return self.status == 1;
    }
    pub fn is_closed(&self) -> bool {
        return self.status == 3;
    }
    pub fn is_rejected(&self) -> bool {
        return self.status == 5;
    }
    pub fn create_suspend (
        &self,
        manager_id:  i32,
        duration:    Option<chrono::NaiveDateTime>,
        description: Option<String>
    ) -> i16 {
        let _connection = establish_connection();

        diesel::update(self)
            .set((
                schema::moderateds::status.eq(2),
                schema::moderateds::verified.eq(true)
            ))
            .execute(&_connection)
            .expect("E");

        ModeratedPenaltie::create_suspension_penalty (
            self.id,
            manager_id,
            self.community_id,
            duration
        );
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          1,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: duration,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        let item = get_community(self.community_id).expect("E.");
        item.suspend_item();
        return 1;
    }
    pub fn create_close (
        &self,
        manager_id:  i32,
        description: Option<String>
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::moderateds::status.eq(3),
                schema::moderateds::verified.eq(true)
            ))
            .execute(&_connection)
            .expect("E");

        ModeratedPenaltie::create_close_penalty (
            self.id,
            manager_id,
            self.community_id
        );

        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          2,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");
        let item = get_community(self.community_id).expect("E.");
        item.close_item();
        return 1;
    }
    pub fn delete_close (
        &self,
        manager_id:  i32,
        description: Option<String>
    ) -> i16 {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
            moderateds::dsl::moderateds,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          4,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        diesel::delete (
            moderated_penalties
                .filter(schema::moderated_penalties::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderated_reports
                .filter(schema::moderated_reports::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderateds
                .filter(schema::moderateds::id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        let item = get_community(self.community_id).expect("E.");
        item.unclose_item();
        return 1;
    }
    pub fn delete_suspend (
        &self,
        manager_id:  i32,
        description: Option<String>
    ) -> i16 {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
            moderateds::dsl::moderateds,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          3,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        diesel::delete (
            moderated_penalties
                .filter(schema::moderated_penalties::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderated_reports
                .filter(schema::moderated_reports::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderateds
                .filter(schema::moderateds::id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        let item = get_community(self.community_id).expect("E.");
        item.unsuspend_item();
        return 1;
    }
    pub fn unverify (
        &self,
        manager_id:  i32,
        description: Option<String>
    ) -> i16 {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          5,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        diesel::delete (
            moderated_penalties
                .filter(schema::moderated_penalties::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderated_reports
                .filter(schema::moderated_reports::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::update(self)
            .set(schema::moderateds::verified.eq(false))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn reject (
        &self,
        manager_id: i32,
        description: Option<String>
    ) -> i16 {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    self.community_id,
            action:          6,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        diesel::delete (
            moderated_penalties
                .filter(schema::moderated_penalties::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::delete (
            moderated_reports
                .filter(schema::moderated_reports::moderated_id.eq(self.id))
        )
        .execute(&_connection)
        .expect("E");

        diesel::update(self)
            .set(schema::moderateds::verified.eq(true))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn get_reports(&self) -> Vec<ModeratedReport> {
        use crate::schema::moderated_reports::dsl::moderated_reports;

        let _connection = establish_connection();
        return moderated_reports
            .filter(schema::moderated_reports::moderated_id.eq(self.id))
            .load::<ModeratedReport>(&_connection)
            .expect("E");
    }
    pub fn get_reporters_ids(&self) -> Vec<i32> {
        use crate::schema::moderated_reports::dsl::moderated_reports;
        let _connection = establish_connection();
        return moderated_reports
            .filter(schema::moderated_reports::moderated_id.eq(self.id))
            .select(schema::moderated_reports::user_id)
            .load::<i32>(&_connection)
            .expect("E");

    }
    //pub fn get_report_users(&self) -> Vec<User> {
    //    use crate::utils::get_users_from_ids;

    //    return get_users_from_ids(self.get_reporters_ids());
    //}

}


#[derive(Deserialize, Insertable)]
#[table_name="moderateds"]
pub struct NewModerated {
    pub description:  Option<String>,
    pub verified:     bool,
    pub status:       i16,
    pub community_id: i32,
    pub created:      chrono::NaiveDateTime,
    pub count:        i32,
}

/*
ModeratedReport

Тип жалобы
1 Порнография
2 Для взрослых
3 Оскорбительное содержание
4 Мошенничество
5 Наркотики
6 Продажа оружия
7 Насилие
8 Призыв к травле
9 Призыв к суициду
10 Жестокое обращение c животными
11 Введение в заблуждение
12 Экстремизм
13 Риторика ненависти
*/
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct ModeratedReport {
    pub id:           i32,
    pub user_id:      i32,
    pub moderated_id: i32,
    pub description:  Option<String>,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
}
impl ModeratedReport {
    pub fn create (
        reporter_id:  i32,
        types:        i16,
        community_id: i32,
        description:  Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let moderated_obj = Moderated::get_or_create_moderated_object(community_id);
        if moderated_obj.get_reporters_ids().iter().any(|&i| i==reporter_id) {
            return 0;
        }

        let new_form = NewModeratedReport {
            user_id:      reporter_id,
            moderated_id: moderated_obj.id,
            description:  description,
            types:        types,
            created:      chrono::Local::now().naive_utc(),
            };
            diesel::insert_into(schema::moderated_reports::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
            return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="moderated_reports"]
pub struct NewModeratedReport {
    pub user_id:      i32,
    pub moderated_id: i32,
    pub description:  Option<String>,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
}

/*
ModeratedPenaltie

Статус штрафа
1 Приостановлено
2 Закрыто
*/
#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct ModeratedPenaltie {
    pub id:           i32,
    pub user_id:      i32,
    pub moderated_id: i32,
    pub expiration:   Option<chrono::NaiveDateTime>,
    pub community_id: i32,
    pub status:       i16,
    pub created:      chrono::NaiveDateTime,
}

impl ModeratedPenaltie {
    pub fn is_suspend(&self) -> bool {
        return self.status == 1;
    }
    pub fn is_closed(&self) -> bool {
        return self.status == 2;
    }
    pub fn create_suspension_penalty (
        moderated_id: i32,
        manager_id: i32,
        community_id: i32,
        duration: Option<chrono::NaiveDateTime>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewModeratedPenaltie {
            user_id:      manager_id,
            moderated_id: moderated_id,
            expiration:   duration,
            community_id: community_id,
            status:       1,
            created:      chrono::Local::now().naive_utc(),
            };
        diesel::insert_into(schema::moderated_penalties::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn create_close_penalty (
        moderated_id: i32,
        manager_id:   i32,
        community_id: i32,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewModeratedPenaltie {
            user_id:      manager_id,
            moderated_id: moderated_id,
            expiration:   None,
            community_id: community_id,
            status:       2,
            created:      chrono::Local::now().naive_utc(),
            };
        diesel::insert_into(schema::moderated_penalties::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="moderated_penalties"]
pub struct NewModeratedPenaltie {
    pub user_id:      i32,
    pub moderated_id: i32,
    pub expiration:   Option<chrono::NaiveDateTime>,
    pub community_id: i32,
    pub status:       i16,
    pub created:      chrono::NaiveDateTime,
}

/*
ModeratedLogs
action:
1 приостановлено
2 закрыто
3 отмена приостановки
4 отмена закрытия
5 отмена верификации
6 отклонение жалоб
*/
#[derive(Debug, Queryable, Serialize, Identifiable,  Associations)]
pub struct ModeratedLog {
    pub id:              i32,
    pub user_id:         i32,
    pub community_id:    i32,
    pub action:          i16,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}

impl ModeratedLog {
    pub fn create (
        manager_id:   i32,
        community_id: i32,
        action:       i16,
        description:  Option<String>,
        time_to_suspend: Option<chrono::NaiveDateTime>
    ) -> i16 {
        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            community_id:    community_id,
            action:          action,
            description:     description,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: time_to_suspend,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
}
#[derive(Deserialize, Insertable)]
#[table_name="moderated_logs"]
pub struct NewModeratedLog {
    pub user_id:         i32,
    pub community_id:    i32,
    pub action:          i16,
    pub description:     Option<String>,
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}


/*
OwnerService
сервисы токенов и их разрешения. Работа с данными
только для владельцев токенов
types:
4 Записи
34 Записи
*/
#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable)]
pub struct OwnerService {
    pub id:    i32,
    pub types: i16,
    pub name:  String,
}

impl OwnerService {
    pub fn get_all() -> Vec<OwnerService> {
        use crate::schema::owner_services::dsl::owner_services;

        let _connection = establish_connection();
        return owner_services
            .load::<OwnerService>(&_connection)
            .expect("E.");
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="owner_services"]
pub struct NewOwnerService {
    pub types: i16,
    pub name:  String,
}

/*
Owner
Тип владельца
1 Приложение
2 Пользователь
3. Сообщество
*/
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Owner {
    pub id:           i32,
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub name:         String,
    pub description:  Option<String>,
    pub types:        i16,
    pub secret_key:   String,
    pub service_key:  String,
    pub is_active:    bool,
}

#[derive(Serialize)]
pub struct TokenDetailJson {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub is_active:   bool,
    pub services:    Vec<TokenServiceJson>,
}
#[derive(Serialize)]
pub struct TokenJson {
    pub id:        i32,
    pub name:      String,
    pub is_active: bool,
    pub services:  Vec<TokenServiceJson>,
}
#[derive(Serialize)]
pub struct EditedOwnerData {
    pub name:        String,
    pub description: Option<String>,
}

impl Owner {
    pub fn get_edit_data(&self) -> EditTokenPageResp {
        return EditTokenPageResp {
            id:            self.id,
            name:          self.name.clone(),
            description:   self.description.clone(),
            is_active:     self.is_active,
            item_services: self.get_services(),
            all_services:  OwnerService::get_all(),
        }
    }
    pub fn is_service_types_ok(&self, types: i16) -> bool {
        use crate::schema::{
            owner_services::dsl::owner_services,
            owner_services_items::dsl::owner_services_items,
        };

        let _connection = establish_connection();
        let items_ids = owner_services_items
            .filter(schema::owner_services_items::owner_id.eq(self.id))
            .select(schema::owner_services_items::id)
            .load::<i32>(&_connection)
            .expect("E.");
        let types_vec = owner_services
            .filter(schema::owner_services::id.eq_any(items_ids))
            .select(schema::owner_services::types)
            .load::<i16>(&_connection)
            .expect("E.");
        return types_vec.iter().any(|&i| i==types);
    }

    pub fn get_services(&self) -> Vec<OwnerService> {
        use crate::schema::{
            owner_services::dsl::owner_services,
            owner_services_items::dsl::owner_services_items,
        };

        let _connection = establish_connection();
        let items_ids = owner_services_items
            .filter(schema::owner_services_items::owner_id.eq(self.id))
            .select(schema::owner_services_items::id)
            .load::<i32>(&_connection)
            .expect("E.");

        return owner_services
            .filter(schema::owner_services::id.eq_any(items_ids))
            .load::<OwnerService>(&_connection)
            .expect("E.");
    }
    pub fn get_token_detail(&self) -> TokenDetailJson {
        let _connection = establish_connection();
        let mut services = Vec::new();
        for s in self.get_services().iter() {
            services.push (TokenServiceJson {
                id:   s.id,
                name: s.name.clone(),
            });
        }

        return TokenDetailJson {
            id:          self.id,
            name:        self.name.clone(),
            description: self.description.clone(),
            is_active:   self.is_active,
            services:    services,
        }
    }

    pub fn create (
        user_id:      i32,
        community_id: Option<i32>,
        name:         String,
        description:  Option<String>,
        services_ids: Vec<i32>
    ) -> TokenDetailJson {
        use uuid::Uuid;

        let _connection = establish_connection();
        let new_form = NewOwner {
            user_id:      user_id,
            community_id: community_id,
            name:         name,
            description:  description,
            types:        3,
            secret_key:   Uuid::new_v4().to_string(),
            service_key:  Uuid::new_v4().to_string() + &"-".to_string() + &Uuid::new_v4().to_string(),
            is_active:    true,
        };
        let new_token = diesel::insert_into(schema::owners::table)
            .values(&new_form)
            .get_result::<Owner>(&_connection)
            .expect("E.");
    
        for id in services_ids.iter() {
            let new_item = NewOwnerServicesItem {
                owner_id:   new_token.id,
                service_id: *id,
            };
            let _new_item = diesel::insert_into(schema::owner_services_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
        }
        return new_token.get_token_detail();
    }
    pub fn delete(&self) -> i16 {
        use crate::models::moderation::owners::dsl::owners;
        let _connection = establish_connection();
        diesel::delete (
            owners
                .filter(schema::owners::user_id.eq(self.user_id))
        )
        .execute(&_connection)
        .expect("E.");
        return 1;
    }
    pub fn edit (
        &self,
        name:        String,
        description: Option<String>,
        services_ids: Vec<i32>
    ) -> TokenDetailJson {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::owners::name.eq(name.clone()),
                schema::owners::description.eq(description.clone()),
            ))
            .execute(&_connection)
            .expect("E.");

        for id in services_ids.iter() {
            let new_item = NewOwnerServicesItem {
                owner_id:   self.id,
                service_id: *id,
            };
            diesel::insert_into(schema::owner_services_items::table)
                .values(&new_item)
                .execute(&_connection)
                .expect("Error.");
        }
        return self.get_token_detail();
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="owners"]
pub struct NewOwner {
    pub user_id:      i32,
    pub community_id: Option<i32>,
    pub name:         String,
    pub description:  Option<String>,
    pub types:        i16,
    pub secret_key:   String,
    pub service_key:  String,
    pub is_active:    bool,
}


// связь сервисов токенов с токенами
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct OwnerServicesItem {
    pub id:         i32,
    pub owner_id:   i32,
    pub service_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="owner_services_items"]
pub struct NewOwnerServicesItem {
    pub owner_id:   i32,
    pub service_id: i32,
}