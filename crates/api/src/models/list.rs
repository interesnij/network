use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    PgTextExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
    ReactionsJson,
};
use crate::schema::{
    reactions,
    sticker_categories,
    stickers,
    smile_categories,
    smiles,
};
use crate::errors::Error;


/////// StickerCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StickerCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub user_id:     Option<i32>,
    pub description: Option<String>,
    pub avatar:      Option<String>,
}

impl StickerCategorie {
    pub fn create_category(name: String, position: i16,
        user_id: Option<i32>, description: Option<String>,
        avatar: Option<String>) -> StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:        name,
            position:    position,
            user_id:     user_id,
            description: description,
            avatar:      avatar,
        };
        let new_cat = diesel::insert_into(schema::sticker_categories::table)
            .values(&new_form)
            .get_result::<StickerCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        user_id: Option<i32>, description: Option<String>,
        avatar: Option<String>) -> &StickerCategorie {
        let _connection = establish_connection();
        let new_form = NewStickerCategorie {
            name:        name,
            position:    position,
            user_id:     user_id,
            description: description,
            avatar:      avatar,
        };
        diesel::update(self)
            .set(new_form)
            .execute(&_connection)
            .expect("Error.");
        return self;
    }
    pub fn get_image(&self) -> &str {
        if self.avatar.is_some() {
            return self.avatar.as_deref().unwrap();
        }
        else {
            return "/static/images/no_img/smile.gif";
        }
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="sticker_categories"]
pub struct NewStickerCategorie {
    pub name:        String,
    pub position:    i16,
    pub user_id:     Option<i32>,
    pub description: Option<String>,
    pub avatar:      Option<String>,
}

/////// Stickers //////
#[derive(Identifiable, Queryable, Associations)]
pub struct Sticker {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

impl Sticker {
    pub fn create_sticker(name: String, position: i16,
        category_id: i32, image: String) -> Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        let new_sticker = diesel::insert_into(schema::stickers::table)
            .values(&new_form)
            .get_result::<Sticker>(&_connection)
            .expect("Error.");
        return new_sticker;
    }
    pub fn edit_sticker(&self, name: String, position: i16,
        category_id: i32, image: String) -> &Sticker {
        let _connection = establish_connection();
        let new_form = NewSticker {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        diesel::update(self)
            .set(new_form)
            .execute(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="stickers"]
pub struct NewSticker {
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

/////// SmileCategories //////
#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct SmileCategorie {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}

impl SmileCategorie {
    pub fn get_smiles(&self) -> Vec<Smile> {
        use crate::schema::smiles::dsl::smiles;
        let _connection = establish_connection();

        return smiles
            .filter(schema::smiles::category_id.eq(self.id))
            .order(schema::smiles::position.asc())
            .load::<Smile>(&_connection)
            .expect("E.");
    }
    pub fn create_category(name: String, position: i16,
        description: Option<String>) -> SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        let new_cat = diesel::insert_into(schema::smile_categories::table)
            .values(&new_form)
            .get_result::<SmileCategorie>(&_connection)
            .expect("Error.");
        return new_cat;
    }
    pub fn edit_category(&self, name: String, position: i16,
        description: Option<String>) -> &SmileCategorie {
        let _connection = establish_connection();
        let new_form = NewSmileCategorie {
            name:        name,
            position:    position,
            description: description,
        };
        diesel::update(self)
            .set(new_form)
            .execute(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smile_categories"]
pub struct NewSmileCategorie {
    pub name:        String,
    pub position:    i16,
    pub description: Option<String>,
}

/////// Smiles //////
#[derive(Identifiable, Queryable, Associations)]
pub struct Smile {
    pub id:          i32,
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

impl Smile {
    pub fn create_smile(name: String, position: i16,
        category_id: i32, image: String) -> Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        let new_smile = diesel::insert_into(schema::smiles::table)
            .values(&new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return new_smile;
    }
    pub fn edit_smile(&self, name: String, position: i16,
        category_id: i32, image: String) -> &Smile {
        let _connection = establish_connection();
        let new_form = NewSmile {
            name:        name,
            position:    position,
            category_id: category_id,
            image:       image,
        };
        diesel::update(self)
            .set(new_form)
            .get_result::<Smile>(&_connection)
            .expect("Error.");
        return self;
    }
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="smiles"]
pub struct NewSmile {
    pub name:        String,
    pub position:    i16,
    pub category_id: i32,
    pub image:       String,
}

/////// Reaction //////
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Reaction {
    pub id:        i32,
    pub image:     String,
    pub gif:       String,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}
impl Reaction {
    pub fn get_reactions() -> Result<ReactionsJson, Error> {
        use crate::schema::reactions::dsl::reactions;
        use crate::utils::ReactionJson;

        let _connection = establish_connection();
        let _reactions = reactions
            .select((
                schema::reactions::id,
                schema::reactions::image,
                schema::reactions::name,
            ))
            .load::<ReactionJson>(&_connection)?;
        return Ok(ReactionsJson {
            reactions: _reactions,
        });
    }
}
#[derive(Deserialize, Insertable)]
#[table_name="reactions"]
pub struct NewReaction {
    pub image:     String,
    pub gif:       gif,
    pub name:      String,
    pub is_active: bool,
    pub position:  i16,
}
