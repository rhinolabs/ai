use rhinolabs_core::skills::{SkillSchema, SkillSource, SkillSourceType, Skills};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Generate mock skills.sh HTML with embedded JSON skills data (regular format)
fn mock_skills_sh_html(skills_count: usize) -> String {
    let mut skills_json = Vec::new();
    for i in 0..skills_count {
        skills_json.push(format!(
            r#"{{"source":"owner-{repo}/repo-{repo}","skillId":"skill-{id}","name":"Skill {id}","installs":{installs}}}"#,
            repo = i / 5, // 5 skills per repo
            id = i,
            installs = (skills_count - i) * 100
        ));
    }

    format!(
        r#"<!DOCTYPE html><html><head><title>skills.sh</title></head><body>
<script>self.__next_f.push([1,"a:["$","$L18",null,{{"allTimeSkills":[{}]}}]"])</script>
</body></html>"#,
        skills_json.join(",")
    )
}

/// Generate mock skills.sh HTML with escaped JSON format (legacy)
fn mock_skills_sh_html_escaped(skills_count: usize) -> String {
    let mut skills_json = Vec::new();
    for i in 0..skills_count {
        skills_json.push(format!(
            r#"{{\"source\":\"owner/repo\",\"skillId\":\"skill-{id}\",\"name\":\"Skill {id}\",\"installs\":{installs}}}"#,
            id = i,
            installs = i * 10
        ));
    }

    format!(
        r#"<html><script>data=[{}]</script></html>"#,
        skills_json.join(",")
    )
}

fn make_source(url: &str) -> SkillSource {
    // Append trailing slash so fetch_from_skills_sh appends "hot" → /hot
    let url_with_slash = if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{}/", url)
    };
    SkillSource {
        id: "test-skills-sh".to_string(),
        name: "Test Skills.sh".to_string(),
        source_type: SkillSourceType::Community,
        url: url_with_slash,
        description: "Integration test source".to_string(),
        enabled: true,
        fetchable: true,
        schema: SkillSchema::SkillsSh,
        skill_count: None,
    }
}

#[tokio::test]
async fn test_fetch_skills_sh_regular_json_format() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_skills_sh_html(25)))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let result = Skills::fetch_from_skills_sh(&source).await;

    assert!(result.is_ok(), "Should fetch skills: {:?}", result);
    let skills = result.unwrap();
    assert_eq!(skills.len(), 25);

    // Verify first skill has correct fields
    assert_eq!(skills[0].id, "owner-0/repo-0/skill-0");
    assert_eq!(skills[0].name, "Skill 0");
    assert!(skills[0].url.contains("github.com/owner-0/repo-0"));
}

#[tokio::test]
async fn test_fetch_skills_sh_escaped_json_fallback() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_skills_sh_html_escaped(10)))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let result = Skills::fetch_from_skills_sh(&source).await;

    assert!(
        result.is_ok(),
        "Escaped format should work as fallback: {:?}",
        result
    );
    let skills = result.unwrap();
    assert_eq!(skills.len(), 10);
}

#[tokio::test]
async fn test_fetch_skills_sh_deduplicates_across_pages() {
    let server = MockServer::start().await;

    // HTML with duplicate skillIds
    let html = r#"<html><script>self.__next_f.push([1,"data:[
{"source":"a/b","skillId":"dup-skill","name":"Dup 1","installs":100},
{"source":"a/b","skillId":"dup-skill","name":"Dup 2","installs":200},
{"source":"c/d","skillId":"unique-skill","name":"Unique","installs":50}
]"])</script></html>"#;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let skills = Skills::fetch_from_skills_sh(&source).await.unwrap();

    assert_eq!(
        skills.len(),
        2,
        "Should deduplicate: got {:?}",
        skills.iter().map(|s| &s.id).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_fetch_skills_sh_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let result = Skills::fetch_from_skills_sh(&source).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("500"));
}

#[tokio::test]
async fn test_fetch_skills_sh_empty_html() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let result = Skills::fetch_from_skills_sh(&source).await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Could not find skills data"));
}

#[tokio::test]
async fn test_fetch_skills_sh_marks_installed_skills() {
    let server = MockServer::start().await;

    let html = r#"<html>{"source":"test/repo","skillId":"already-installed","name":"Installed","installs":1}</html>"#;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let skills = Skills::fetch_from_skills_sh(&source).await.unwrap();

    // installed field depends on what's actually installed locally,
    // in test env nothing is installed so all should be false
    assert!(!skills[0].installed);
}

#[tokio::test]
async fn test_fetch_skills_sh_large_dataset() {
    let server = MockServer::start().await;

    // Simulate a large response like real skills.sh (500 skills)
    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_skills_sh_html(500)))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    let skills = Skills::fetch_from_skills_sh(&source).await.unwrap();

    assert_eq!(skills.len(), 500);

    // Verify skills have proper GitHub URLs
    for skill in &skills {
        assert!(
            skill.url.contains("github.com"),
            "URL should be GitHub: {}",
            skill.url
        );
        assert!(!skill.id.is_empty());
        assert!(!skill.name.is_empty());
    }
}

#[tokio::test]
async fn test_fetch_from_source_dispatches_skills_sh() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hot"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_skills_sh_html(3)))
        .mount(&server)
        .await;

    let source = make_source(&server.uri());
    // Use fetch_from_source (dispatcher) instead of fetch_from_skills_sh directly
    let result = Skills::fetch_from_source(&source).await;

    assert!(
        result.is_ok(),
        "Dispatcher should route to skills_sh: {:?}",
        result
    );
    assert_eq!(result.unwrap().len(), 3);
}
