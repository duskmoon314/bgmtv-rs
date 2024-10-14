use super::Client;
use crate::types::*;

/// # Subjects Resource (条目资源)
///
/// - `GET /v0/subjects/{subject_id}`: 获取条目 [`get_subject`](Client::get_subject)
/// - `GET /v0/subjects/{subject_id}/image`: 获取条目图片 [`get_subject_image`](Client::get_subject_image)
/// - `GET /v0/subjects/{subject_id}/persons`: 获取条目相关人物 [`get_subject_persons`](Client::get_subject_persons)
/// - `GET /v0/subjects/{subject_id}/characters`: 获取条目相关角色 [`get_subject_characters`](Client::get_subject_characters)
/// - `GET /v0/subjects/{subject_id}/subjects`: 获取条目相关条目 [`get_subject_subjects`](Client::get_subject_subjects)
impl Client {
    /// # 获取条目 `GET /v0/subjects/{subject_id}`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let subject = client.get_subject(3559).await.expect("Failed to get subject");
    ///
    /// assert_eq!(subject.name, "とある魔術の禁書目録");
    /// # });
    /// ```
    pub async fn get_subject(&self, subject_id: u64) -> Result<Subject, Error> {
        let url = format!("{}/v0/subjects/{}", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subject: Subject = res.json().await?;

        Ok(subject)
    }

    /// # 获取条目图片 `GET /v0/subjects/{subject_id}/image`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    /// * `type` - 图片类型
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let image: Vec<u8> = client.get_subject_image(3559, ImageType::Small).await.expect("Failed to get subject image");
    ///
    /// assert_eq!(image.len(), 12020);
    /// # });
    /// ```
    pub async fn get_subject_image(
        &self,
        subject_id: u64,
        image_type: ImageType,
    ) -> Result<Vec<u8>, Error> {
        let url = format!("{}/v0/subjects/{}/image", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .query(&[("type", image_type)])
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let image = res.bytes().await?;

        Ok(image.to_vec())
    }

    /// # 获取条目相关人物 `GET /v0/subjects/{subject_id}/persons`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let persons = client.get_subject_persons(3559).await.expect("Failed to get subject persons");
    ///
    /// let person = persons.iter().find(|p| p.id == 3608);
    /// assert_eq!(person.map(|p| p.name.as_str()), Some("鎌池和馬"));
    /// # });
    /// ```
    pub async fn get_subject_persons(&self, subject_id: u64) -> Result<Vec<RelatedPerson>, Error> {
        let url = format!("{}/v0/subjects/{}/persons", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let persons: Vec<RelatedPerson> = res.json().await?;

        Ok(persons)
    }

    /// # 获取条目相关角色 `GET /v0/subjects/{subject_id}/characters`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let characters = client.get_subject_characters(3559).await.expect("Failed to get subject characters");
    ///
    /// let character = characters.iter().find(|c| c.id == 3498);
    /// assert_eq!(character.map(|c| c.name.as_str()), Some("上条当麻"));
    /// # });
    /// ```
    pub async fn get_subject_characters(
        &self,
        subject_id: u64,
    ) -> Result<Vec<RelatedCharacter>, Error> {
        let url = format!("{}/v0/subjects/{}/characters", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let characters: Vec<RelatedCharacter> = res.json().await?;

        Ok(characters)
    }

    /// # 获取条目相关条目 `GET /v0/subjects/{subject_id}/subjects`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let subjects = client.get_subject_subjects(3559).await.expect("Failed to get subject subjects");
    ///
    /// let subject = subjects.iter().find(|s| s.id == 3582);
    /// assert_eq!(subject.map(|s| s.name.as_str()), Some("とある魔術の禁書目録外伝 とある科学の超電磁砲"));
    /// # });
    /// ```
    pub async fn get_subject_subjects(
        &self,
        subject_id: u64,
    ) -> Result<Vec<SubjectRelation>, Error> {
        let url = format!("{}/v0/subjects/{}/subjects", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: Vec<SubjectRelation> = res.json().await?;

        Ok(subjects)
    }
}
