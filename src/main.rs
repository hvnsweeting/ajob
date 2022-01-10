use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Label {
    name: String,
}
#[derive(Debug, Deserialize)]
struct Job {
    title: String,
    created_at: String,
    html_url: String,
    labels: Vec<Label>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = reqwest::blocking::Client::builder()
        .user_agent("reqwest")
        .build()?;
    let r = c
        .get("https://api.github.com/repos/awesome-jobs/vietnam/issues")
        .send()?;
    assert_eq!(r.status(), 200);
    let d: Vec<Job> = r.json()?;
    for (idx, job) in d.iter().rev().enumerate() {
        println!("{}. {} {}", idx + 1, &job.created_at[0..10], job.title);

        let labels: Vec<String> = job.labels.iter().map(|l| l.name.clone()).collect();
        println!("\tLabels: {}", labels.join(", "));
        println!("\t{}", job.html_url);
        println!()
    }
    Ok(())
}
