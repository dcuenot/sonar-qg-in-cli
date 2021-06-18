use crate::domain::gitlab::merge_requests::MergeRequests;
use crate::domain::gitlab::note::Note;
use crate::domain::sonar::QualityStatus;
use crate::infra::api_call_remote::ApiCallRemoteAdapter;
use reqwest::header::HeaderValue;

#[derive(Clone)]
pub(crate) struct GitlabClient {
    url: String,
    token: String,
    ci_project_id: i64,
}

impl GitlabClient {
    pub fn new(url: &str, token: &str, ci_project_id: i64) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            ci_project_id,
        }
    }

    pub async fn list_opened_merge_requests(
        self,
        ci_commit_ref_name: &str,
    ) -> anyhow::Result<MergeRequests> {
        let request = reqwest::Client::new()
            .get(&format!("{}/api/v4/projects/{}/merge_requests", self.url, &self.ci_project_id.to_string()))
            .query(&[
                ("source_branch", ci_commit_ref_name),
                ("state", "opened"),
            ])
            .header("PRIVATE-TOKEN", self.header_authorization())
            .build()?;

        let res = ApiCallRemoteAdapter {}
            .send::<MergeRequests>(request)
            .await?;
        debug!("{:?}", res);
        Ok(res)
    }

    pub async fn write_quality_gate_report(
        self,
        project_id: i64,
        ci_merge_request_iid: i64,
        qualtiy_status: QualityStatus,
    ) -> anyhow::Result<()> {
        let note = Note::from_quality_status(qualtiy_status, project_id, ci_merge_request_iid);
        let request = reqwest::Client::new()
            .post(&format!(
                "{}/api/v4/projects/{}/merge_requests/{}/notes",
                self.url, project_id, ci_merge_request_iid
            ))
            .json(&note)
            .header("PRIVATE-TOKEN", self.header_authorization())
            .build()?;

        let _res = ApiCallRemoteAdapter {}.send::<()>(request).await;
        debug!("Note successfully written in Gitlab: {:?}", note);
        Ok(())
    }

    // Dirty workaround could be removed once PR will be validated
    // https://github.com/seanmonstar/reqwest/pull/916
    fn header_authorization(&self) -> HeaderValue {
        let mut token =
            HeaderValue::from_str(&self.token).expect("Issue during HeaderValue creation");
        token.set_sensitive(true);
        token
    }
}
