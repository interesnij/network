use crate::schema;
use crate::schema::{
    moderateds,
    moderated_reports,
    moderated_penalties,
    moderated_logs,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{
    User,
    //Community,
};


/////// Moderated //////

////////// Тип модерируемого объекта
    // 1 Пользователь
    // 2 Сообщество
    // 3 Список
    // 4 Запись
    // 5 Коммент к записи
////////// Статус
    // 1 На рассмотрении
    // 2 Объект заморожен
    // 3 Объект закрыт
    // 4 Объекту присвоен баннер
    // 5 Отвергнутый

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Moderated {
    pub id:          i32,
    pub description: Option<String>,
    pub verified:    bool,
    pub status:      i16,
    pub types:       i16,
    pub object_id:   i32,
    pub created:     chrono::NaiveDateTime,
    pub count:       i32,
}

impl Moderated {
    pub fn get_or_create_moderated_object (
        object_id: i32,
        types: i16,
    ) -> Moderated {
        use crate::schema::moderateds::dsl::moderateds;

        let _connection = establish_connection();
        let some_moderateds = moderateds
            .filter(schema::moderateds::object_id.eq(object_id))
            .filter(schema::moderateds::types.eq(types))
            .first::<Moderated>(&_connection);
        if some_moderateds.is_ok() {
            return some_moderateds.expect("E");
        }
        else {
            let new_form = NewModerated {
                description: None,
                verified:    false,
                status:      1,
                types:       types,
                object_id:   object_id,
                created:     chrono::Local::now().naive_utc(),
                count:       0,
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
        return self.types == 2;
    }
    pub fn is_pending(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_closed(&self) -> bool {
        return self.types == 3;
    }
    pub fn is_rejected(&self) -> bool {
        return self.types == 5;
    }
    pub fn create_suspend (
        &self,
        manager_id: i32,
        duration: chrono::NaiveDateTime
    ) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::moderateds::types.eq(2),
                schema::moderateds::verified.eq(true)
            ))
            .execute(&_connection);

        ModeratedPenalty::create_suspension_penalty (
            self.id,
            manager_id,
            self.types,
            self.object_id,
            duration
        );
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          1,
            description:     None,
            types:           self.types,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: Some(duration),
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");

        return true;
    }
    pub fn create_close (
        &self,
        manager_id: i32,
    ) -> bool {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::moderateds::types.eq(3),
                schema::moderateds::verified.eq(true)
            ))
            .execute(&_connection);

        ModeratedPenalty::create_close_penalty (
            self,
            manager_id,
            self.types,
            self.object_id
        );

        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          2,
            description:     None,
            types:           self.types,
            created:         chrono::Local::now().naive_utc(),
            time_to_suspend: None,
        };
        let _new = diesel::insert_into(schema::moderated_logs::table)
            .values(&new_log_form)
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn delete_close (
        &self,
        manager_id: i32,
    ) -> bool {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
            moderateds::dsl::moderateds,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          4,
            description:     None,
            types:           self.types,
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

        return true;
    }
    pub fn delete_suspend (
        &self,
        manager_id: i32,
    ) -> bool {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
            moderateds::dsl::moderateds,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          3,
            description:     None,
            types:           self.types,
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

        return true;
    }
    pub fn unverify (
        &self,
        manager_id: i32,
    ) -> bool {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          5,
            description:     None,
            types:           self.types,
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
            .set((
                schema::moderateds::verified.eq(false)
            ))
            .execute(&_connection);
        return true;
    }
    pub fn reject (
        &self,
        manager_id: i32,
    ) -> bool {
        use crate::schema::{
            moderated_penalties::dsl::moderated_penalties,
            moderated_reports::dsl::moderated_reports,
        };

        let _connection = establish_connection();
        let new_log_form = NewModeratedLog {
            user_id:         manager_id,
            object_id:       self.id,
            action:          6,
            description:     None,
            types:           self.types,
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
            .set((
                schema::moderateds::verified.eq(true)
            ))
            .execute(&_connection);
        return true;
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
    pub fn get_report_users(&self) -> Vec<User> {
        use crate::utils::get_users_from_ids;

        return get_users_from_ids(self.get_reporters_ids());
    }

}


#[derive(Deserialize, Insertable)]
#[table_name="moderateds"]
pub struct NewModerated {
    pub description: Option<String>,
    pub verified:    bool,
    pub status:      i16,
    pub types:       i16,
    pub object_id:   i32,
    pub created:     chrono::NaiveDateTime,
    pub count:       i32,
}

/////// ModeratedReport //////

////////// Тип жалобы
    // 1 Порнография
    // 2 Для взрослых
    // 3 Оскорбительное содержание
    // 4 Мошенничество
    // 5 Наркотики
    // 6 Продажа оружия
    // 7 Насилие
    // 8 Призыв к травле
    // 9 Призыв к суициду
    // 10 Жестокое обращение c животными
    // 11 Введение в заблуждение
    // 12 Экстремизм
    // 13 Риторика ненависти

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
        reporter_id: i32,
        types: i16,
        object_id: i32,
        description: Option<String>,
        repost_types: i16
    ) -> bool {
        use crate::schema::moderated_reports::dsl::moderated_reports;

        let _connection = establish_connection();
        let moderated_obj = Moderated::get_or_create_moderated_object(object_id, types);
        if moderated_obj.get_reporters_ids().iter().any(|&i| i==reporter_id) {
            return false;
        }

        let new_form = NewModeratedReport {
            user_id:      reporter_id,
            moderated_id: moderated_obj.id,
            description:  description,
            types:        repost_types,
            created:      chrono::Local::now().naive_utc(),
            };
            diesel::insert_into(schema::moderated_reports::table)
                .values(&new_form)
                .execute(&_connection)
                .expect("Error.");
            return true;
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

/////// ModeratedPenalty //////

////////// Статус штрафа
    // 1 Приостановлено
    // 2 Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct ModeratedPenaltie {
    pub id:           i32,
    pub user_id:      i32,
    pub moderated_id: i32,
    pub expiration:   Option<chrono::NaiveDateTime>,
    pub types:        i16, // описан в самом начале, одно и то же - объект.
    pub object_id:    i32,
    pub status:       i16,
    pub created:      chrono::NaiveDateTime,
}

impl ModeratedPenaltie {
    pub fn is_suspend(&self) -> bool {
        return self.types == 1;
    }
    pub fn is_closed(&self) -> bool {
        return self.types == 2;
    }
    pub fn create_suspension_penalty (
        moderated_id: i32,
        manager_id: i32,
        types: i16,
        object_id: i32,
        duration: chrono::NaiveDateTime,
    ) -> bool {
        let _connection = establish_connection();
        let new_form = NewModeratedPenaltie {
            user_id:      manager_id,
            moderated_id: moderated_id,
            expiration:   Some(duration),
            types:        types,
            object_id:    object_id,
            status:       1,
            created:      chrono::Local::now().naive_utc(),
            };
        diesel::insert_into(schema::moderated_penalties::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
    pub fn create_close_penalty (
        moderated_id: i32,
        manager_id: i32,
        types: i16,
        object_id: i32,
    ) -> bool {
        let _connection = establish_connection();
        let new_form = NewModeratedPenaltie {
            user_id:      manager_id,
            moderated_id: moderated_id,
            expiration:   None,
            types:        types,
            object_id:    object_id,
            status:       2,
            created:      chrono::Local::now().naive_utc(),
            };
        diesel::insert_into(schema::moderated_penalties::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        return true;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="moderated_penalties"]
pub struct NewModeratedPenaltie {
    pub user_id:      i32,
    pub moderated_id: i32,
    pub expiration:   Option<chrono::NaiveDateTime>,
    pub types:        i16, // описан в самом начале, одно и то же - объект.
    pub object_id:    i32,
    pub status:       i16,
    pub created:      chrono::NaiveDateTime,
}

/////// ModeratedLogs //////
// action:
// 1 приостановлено
// 2 закрыто
// 3 отмена приостановки
// 4 отмена закрытия

#[derive(Debug, Queryable, Serialize, Identifiable,  Associations)]
pub struct ModeratedLog {
    pub id:              i32,
    pub user_id:         i32,
    pub object_id:       i32,
    pub action:          i16,
    pub description:     Option<String>,
    pub types:           i16,            // описан в самом начале, одно и то же - объект.
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}
#[derive(Deserialize, Insertable)]
#[table_name="moderated_logs"]
pub struct NewModeratedLog {
    pub user_id:         i32,
    pub object_id:       i32,
    pub action:          i16,
    pub description:     Option<String>,
    pub types:           i16,                 // описан в самом начале, одно и то же - объект.
    pub created:         chrono::NaiveDateTime,
    pub time_to_suspend: Option<chrono::NaiveDateTime>,
}
