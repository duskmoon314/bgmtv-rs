use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Header value is not str: {0}")]
    HeaderValueToStr(#[from] reqwest::header::ToStrError),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Cannot serialize: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Character Type (角色类型)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum CharacterType {
    /// 角色
    Character = 1,

    /// 机体
    Mechanic = 2,

    /// 舰船
    Ship = 3,

    /// 组织
    Organization = 4,
}

/// Images (图片)
///
/// 存储不同尺寸的图片链接。
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Images {
    pub large: String,

    pub common: String,

    pub medium: String,

    pub small: String,

    pub grid: String,
}

/// Image Type (图片类型)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    Small,
    Common,
    Medium,
    Large,
    Grid,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Infobox {
    pub key: String,
    pub value: InfoboxValue,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InfoboxValue {
    Single(String),
    List(Vec<InfoboxValueItem>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InfoboxValueItem {
    KV { k: String, v: String },
    V { v: String },
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PagedSubject {
    pub total: u64,

    pub limit: u64,

    pub offset: u64,

    pub data: Vec<Subject>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    pub id: usize,

    pub name: String,

    pub r#type: PersonType,

    pub career: Vec<PersonCareer>,

    pub images: Option<PersonImages>,

    pub short_summary: String,

    pub locked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonCareer {
    Producer,
    Mangaka,
    Artist,
    Seiyu,
    Writer,
    Illustrator,
    Actor,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PersonImages {
    pub large: String,

    pub medium: String,

    pub small: String,

    pub grid: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PersonType {
    /// 个人
    Individual = 1,

    /// 公司
    Corporation = 2,

    /// 组合
    Association = 3,
}

/// Related Character (条目相关角色)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelatedCharacter {
    pub id: usize,

    pub name: String,

    pub r#type: CharacterType,

    pub images: Option<PersonImages>,

    pub relation: String,

    pub actors: Vec<Person>,
}

/// Related Person (条目相关人物)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RelatedPerson {
    /// ID
    pub id: u64,

    /// 名称
    pub name: String,

    /// 类型
    pub r#type: PersonType,

    pub career: Vec<PersonCareer>,

    pub images: Option<PersonImages>,

    pub relation: String,

    pub eps: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchSubjects {
    /// 搜索结果数量
    pub total: u64,

    /// 当前分页数量
    pub limit: u64,

    /// 当前分页参数
    pub offset: u64,

    /// 数据
    pub data: Vec<SearchSubjectsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchSubjectsBody {
    pub keyword: String,

    pub filter: SearchSubjectsFilter,

    pub sort: SortType,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchSubjectsFilter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<SubjectType>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub air_date: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rating: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rank: Vec<String>,

    pub nsfw: bool,
}

/// Search Subjects Item (搜索条目数据)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchSubjectsItem {
    pub id: u64,

    pub r#type: SubjectType,

    pub date: String,

    pub image: String,

    pub summary: String,

    pub name: String,

    pub name_cn: String,

    pub tags: Vec<SubjectTag>,

    pub score: f64,

    pub rank: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortType {
    /// 匹配程度，meilisearch 默认排序
    #[default]
    Match,
    /// 收藏人数
    Heat,
    /// 排名由高到低
    Rank,
    /// 评分由高到低
    Score,
}

/// Subject (条目)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Subject {
    /// ID
    pub id: u64,

    /// type
    pub r#type: SubjectType,

    /// 名称
    pub name: String,

    /// 中文名称
    pub name_cn: String,

    pub summary: String,

    /// 是否为书籍系列的主条目
    pub series: bool,

    /// None Safe For Work
    pub nsfw: bool,

    pub locked: bool,

    pub date: Option<String>,

    pub platform: String,

    /// 图片
    pub images: Images,

    pub infobox: Vec<Infobox>,

    /// 书籍条目的册数
    pub volumes: u64,

    /// 对于书籍条目为话数
    pub eps: u64,

    pub total_episodes: u64,

    /// 评分
    pub rating: SubjectRating,

    /// 收藏
    pub collection: SubjectCollection,

    /// 标签
    pub tags: Vec<SubjectTag>,
}

/// Subject Category (条目分类)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubjectCategory {
    Book(SubjectBookCategory),
    Anime(SubjectAnimeCategory),
    Game(SubjectGameCategory),
    Real(SubjectRealCategory),
}

/// Subject Book Category (书籍条目分类)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum SubjectBookCategory {
    Other = 0,
    Comic = 1001,
    Novel = 1002,
    Illustration = 1003,
}

/// Subject Anime Category (动画条目分类)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum SubjectAnimeCategory {
    TV = 1,
    OVA = 2,
    Movie = 3,
    Web = 4,
}

/// Subject Game Category (游戏条目分类)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum SubjectGameCategory {
    Other = 0,
    Games = 4001,
    Software = 4002,
    DLC = 4003,
    Tabletop = 4005,
}

/// Subject Real Category (三次元条目分类)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum SubjectRealCategory {
    Other = 0,
    /// 日剧
    JP = 1,
    /// 欧美剧
    EN = 2,
    /// 华语剧
    CN = 3,
    /// 电视剧
    TV = 6001,
    /// 电影
    Movie = 6002,
    /// 演出
    Live = 6003,
    /// 综艺
    Show = 6004,
}

/// Subject Collection (条目收藏)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubjectCollection {
    /// 想看
    pub wish: usize,

    /// 看过
    pub collect: usize,

    /// 在看
    pub doing: usize,

    /// 搁置
    pub on_hold: usize,

    /// 抛弃
    pub dropped: usize,
}

/// Subject Rating (条目评分)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubjectRating {
    pub rank: u64,

    pub total: u64,

    pub count: SubjectRatingCount,

    pub score: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubjectRatingCount {
    #[serde(rename = "1")]
    pub one: u64,

    #[serde(rename = "2")]
    pub two: u64,

    #[serde(rename = "3")]
    pub three: u64,

    #[serde(rename = "4")]
    pub four: u64,

    #[serde(rename = "5")]
    pub five: u64,

    #[serde(rename = "6")]
    pub six: u64,

    #[serde(rename = "7")]
    pub seven: u64,

    #[serde(rename = "8")]
    pub eight: u64,

    #[serde(rename = "9")]
    pub nine: u64,

    #[serde(rename = "10")]
    pub ten: u64,
}

/// Subject Relation (条目相关条目)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubjectRelation {
    pub id: u64,

    pub r#type: SubjectType,

    pub name: String,

    pub name_cn: String,

    pub relation: String,
}

/// Subject Tag (条目标签)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubjectTag {
    pub name: String,

    pub count: u64,
}

/// Subject Type (条目类型)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SubjectType {
    /// 书籍
    Book = 1,

    /// 动画
    Anime = 2,

    /// 音乐
    Music = 3,

    /// 游戏
    Game = 4,

    /// 三次元
    Real = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infobox_value() {
        let data = r#"
        [
          {"v":"魔法禁書目錄"},
          {"v":"某魔术的禁书目录"},
          {"v":"传说中魔术的禁书目录"},
          {"v":"传说中的魔法禁书目录"},
          {"v":"とあるまじゅつのインデックス"}
        ]"#;

        let value: InfoboxValue = serde_json::from_str(data).unwrap();

        assert_eq!(
            value,
            InfoboxValue::List(vec![
                InfoboxValueItem::V {
                    v: "魔法禁書目錄".to_string()
                },
                InfoboxValueItem::V {
                    v: "某魔术的禁书目录".to_string()
                },
                InfoboxValueItem::V {
                    v: "传说中魔术的禁书目录".to_string()
                },
                InfoboxValueItem::V {
                    v: "传说中的魔法禁书目录".to_string()
                },
                InfoboxValueItem::V {
                    v: "とあるまじゅつのインデックス".to_string()
                },
            ])
        );
    }

    #[test]
    fn test_infobox() {
        let data = r#"
        [
          {"key":"中文名","value":"魔法禁书目录"},
          {"key":"别名","value":[
            {"v":"魔法禁書目錄"},
            {"v":"某魔术的禁书目录"},
            {"v":"传说中魔术的禁书目录"},
            {"v":"传说中的魔法禁书目录"},
            {"v":"とあるまじゅつのインデックス"}
          ]},
          {"key":"出版社","value":"KADOKAWA/アスキー・メディアワークス、台灣角川、湖南美术出版社"},
          {"key":"发售日","value":"2004-04-24"},
          {"key":"册数","value":"24(22+2)卷完结"},
          {"key":"作者","value":"鎌池和馬"},
          {"key":"插图","value":"灰村キヨタカ"},
          {"key":"开始","value":"2004-04-24"},
          {"key":"结束","value":"2010-10-10"},
          {"key":"文库","value":"电击文库"},
          {"key":"出品方","value":"天闻角川（大陆）"}
        ]"#;

        let infoboxes: Vec<Infobox> = serde_json::from_str(data).unwrap();

        assert_eq!(infoboxes.len(), 11);
        assert_eq!(infoboxes[0].key, "中文名");
        assert_eq!(
            infoboxes[0].value,
            InfoboxValue::Single("魔法禁书目录".to_string())
        );
    }

    #[test]
    fn test_subject() {
        // Subject data from https://bgm.tv/subject/3559 on 2024-10-10
        let data = r#"{"date":"2004-04-24","platform":"小说","images":{"small":"https://lain.bgm.tv/r/200/pic/cover/l/f1/1b/3559_rrwkw.jpg","grid":"https://lain.bgm.tv/r/100/pic/cover/l/f1/1b/3559_rrwkw.jpg","large":"https://lain.bgm.tv/pic/cover/l/f1/1b/3559_rrwkw.jpg","medium":"https://lain.bgm.tv/r/800/pic/cover/l/f1/1b/3559_rrwkw.jpg","common":"https://lain.bgm.tv/r/400/pic/cover/l/f1/1b/3559_rrwkw.jpg"},"summary":"　　故事开始于进行“超能力开发”的学园都市中，这是个人口里八成都是学生，由很多学园和各种研究机构组成的科学都市。都市中的学生们除了接受一般的教学课程外，还会进行开发超能力的学习。根据能力高低不同，测定的超能力可以分为6级，从无能力者（Level 0）到超能力者（Level 5），而Level 6则为绝对能力者。\r\n　　居住其中的高中生上条当麻虽然是一个无能力者，但并非完全没有能力，他的能力是可以用右手将一切异能效果无效化，他给自己这种也许连上帝的奇迹都能抹消的能力取名为“幻想杀手”。而正因为他的右手似乎是把神的祝福都给抹杀掉的缘故，导致自己一直过着“不幸”的生活。\r\n　　某一个暑假的日子里，在自家的阳台上，上条当麻遇见了挂在栏杆上的白衣修女。少女自称为“禁书目录”（Index），是从魔法侧的世界里逃出来的，正在被魔法师追赶。从此上条当麻踏入了科学和魔法交错的世界中，和掌握着十万三千册魔导书的禁书目录Index以及其他各式各样的人物一起，开始了一系列故事……\r\n\r\n\r\n　　《魔法禁书目录》（とある魔術の禁書目録）为镰池和马所撰写的轻小说系列，插画为灰村清孝。\r\n　　小说的第一部分《魔法禁书目录》全22卷于2010年10月10日完结，第二部分《新约魔法禁书目录》于2011年3月10日开始发售，另外还有多篇短篇和未收录作品。此外还有由近木野中哉作画的同名漫画作品。\r\n　　另外，由东川基作画的派生漫画作品，以小说中的角色御坂美琴为主人公的《某科学的超电磁炮》（とある科学の超電磁砲）也在连载中。\r\n　　《禁书目录》和《超电磁炮》都有改编为动画，禁书目录已经改编为两季的动画，分别于2008年和2010年播出，超电磁炮的动画于2009年播出。\r\n在2011年10月宣布了制作剧场版动画的消息。","name":"とある魔術の禁書目録","name_cn":"魔法禁书目录","tags":[{"name":"魔法禁书目录","count":296},{"name":"镰池和马","count":291},{"name":"轻小说","count":281},{"name":"把妹之手","count":101},{"name":"科学超电磁炮","count":71},{"name":"一方通行","count":59},{"name":"存在感0的女主","count":49},{"name":"咦女主不是美琴么","count":43},{"name":"当妈表示太受欢迎很辛苦","count":32},{"name":"鎌池和馬","count":20},{"name":"战斗","count":17},{"name":"科幻","count":16},{"name":"泡妹之右手","count":16},{"name":"奇幻","count":13},{"name":"小说","count":12},{"name":"上条当麻","count":12},{"name":"校园","count":12},{"name":"后宫","count":10},{"name":"电击文库","count":10},{"name":"灰村キヨタカ","count":10},{"name":"魔法","count":10},{"name":"2004","count":9},{"name":"超能力","count":9},{"name":"電撃文庫","count":8},{"name":"宇宙神作","count":8},{"name":"电磁炮","count":7},{"name":"系列","count":6},{"name":"哔哩哔哩","count":6},{"name":"把妹御手","count":6},{"name":"魔禁","count":4}],"infobox":[{"key":"中文名","value":"魔法禁书目录"},{"key":"别名","value":[{"v":"魔法禁書目錄"},{"v":"某魔术的禁书目录"},{"v":"传说中魔术的禁书目录"},{"v":"传说中的魔法禁书目录"},{"v":"とあるまじゅつのインデックス"}]},{"key":"出版社","value":"KADOKAWA/アスキー・メディアワークス、台灣角川、湖南美术出版社"},{"key":"发售日","value":"2004-04-24"},{"key":"册数","value":"24(22+2)卷完结"},{"key":"作者","value":"鎌池和馬"},{"key":"插图","value":"灰村キヨタカ"},{"key":"开始","value":"2004-04-24"},{"key":"结束","value":"2010-10-10"},{"key":"文库","value":"电击文库"},{"key":"出品方","value":"天闻角川（大陆）"}],"rating":{"rank":1824,"total":1032,"count":{"1":2,"2":3,"3":3,"4":9,"5":36,"6":120,"7":291,"8":366,"9":123,"10":79},"score":7.6},"total_episodes":0,"collection":{"on_hold":165,"dropped":87,"wish":274,"collect":1109,"doing":327},"id":3559,"eps":0,"volumes":24,"series":true,"locked":false,"nsfw":false,"type":1}"#;

        let subject: Subject = serde_json::from_str(data).unwrap();

        assert_eq!(subject.id, 3559);
        assert_eq!(subject.r#type, SubjectType::Book);
        assert_eq!(subject.name, "とある魔術の禁書目録");
        assert_eq!(subject.name_cn, "魔法禁书目录");
        assert_eq!(subject.series, true);
        assert_eq!(subject.nsfw, false);
        assert_eq!(subject.locked, false);
        assert_eq!(subject.date, Some("2004-04-24".to_string()));
        assert_eq!(subject.platform, "小说");
        assert_eq!(subject.volumes, 24);
        assert_eq!(subject.eps, 0);
        assert_eq!(subject.total_episodes, 0);
        assert_eq!(subject.rating.rank, 1824);
        assert_eq!(subject.collection.wish, 274);
        assert!(subject.tags.len() > 0);
    }

    #[test]
    fn test_subject_category() {
        let cat = SubjectCategory::Book(SubjectBookCategory::Comic);

        assert_eq!(serde_json::to_string(&cat).unwrap(), r#"1001"#);
    }
}
