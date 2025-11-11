use crate::{challenge::Challenge, errors::CoreError};
use scraper::{Html, Selector};

pub struct SearchParams {
    pub name: Option<String>,
    pub author: Option<String>,
    pub difficulty_range: Option<[i32; 2]>,
    pub quality_range: Option<[i32; 2]>,
    pub language: Option<Language>,
    pub arch: Option<Arch>,
    pub platform: Option<Platform>,
    pub token: String,
}

impl std::fmt::Display for SearchParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_deref().unwrap_or("");
        let author = self.author.as_deref().unwrap_or("");

        let (difficulty_min, difficulty_max) = match &self.difficulty_range {
            Some(range) => (range[0].to_string(), range[1].to_string()),
            None => (String::new(), String::new()),
        };

        let (quality_min, quality_max) = match &self.quality_range {
            Some(range) => (range[0].to_string(), range[1].to_string()),
            None => (String::new(), String::new()),
        };

        let language = self
            .language
            .as_ref()
            .map(|l| l.to_string())
            .unwrap_or_default();
        let arch = self
            .arch
            .as_ref()
            .map(|a| a.to_string())
            .unwrap_or_default();
        let platform = self
            .platform
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default();

        write!(
            f,
            "name={}&author={}&difficulty-min={}&difficulty-max={}&quality-min={}&quality-max={}&token={}&language={}&arch={}&platform={}",
            name,
            author,
            difficulty_min,
            difficulty_max,
            quality_min,
            quality_max,
            self.token,
            language,
            arch,
            platform
        )
    }
}

#[derive(Debug)]
pub enum Language {
    Ccpp,
    Assembler,
    Java,
    Go,
    Rust,
    Wasm,
    Basic,
    Borland,
    Pascal,
    Dotnet,
    Other,
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "C/C++" => Language::Ccpp,
            "Assembler" => Language::Assembler,
            "Java" => Language::Java,
            "Go" => Language::Go,
            "Rust" => Language::Rust,
            "WebAssembly" => Language::Wasm,
            "(Visual) Basic" => Language::Basic,
            "Borland Delphi" => Language::Borland,
            "Turbo Pascal" => Language::Pascal,
            ".NET" => Language::Dotnet,
            _ => Language::Other,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Ccpp => "C/C++",
            Language::Assembler => "Assembler",
            Language::Java => "Java",
            Language::Go => "Go",
            Language::Rust => "Rust",
            Language::Wasm => "WebAssembly",
            Language::Basic => "(Visual) Basic",
            Language::Borland => "Borland Delphi",
            Language::Pascal => "Turbo Pascal",
            Language::Dotnet => ".NET",
            Language::Other => "Unspecified/other",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Arch {
    X86,
    X8664,
    Java,
    Arm,
    Mips,
    RiscV,
    Other,
}

impl From<&str> for Arch {
    fn from(s: &str) -> Self {
        match s {
            "x86" => Arch::X86,
            "x86-64" => Arch::X8664,
            "java" => Arch::Java,
            "ARM" => Arch::Arm,
            "MIPS" => Arch::Mips,
            "RISC-V" => Arch::RiscV,
            _ => Arch::Other,
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Arch::X86 => "x86",
            Arch::X8664 => "x86-64",
            Arch::Java => "java",
            Arch::Arm => "ARM",
            Arch::Mips => "MIPS",
            Arch::RiscV => "RISC-V",
            Arch::Other => "other",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Platform {
    Dos,
    MacOSX,
    Multiplatform,
    Unix,
    Windows,
    WindowsXP,
    Windows7,
    Android,
    Ios,
    Other,
}

impl From<&str> for Platform {
    fn from(s: &str) -> Self {
        match s {
            "DOS" => Platform::Dos,
            "Mac OS X" => Platform::MacOSX,
            "Multiplatform" => Platform::Multiplatform,
            "Unix/linux etc." => Platform::Unix,
            "Windows" => Platform::Windows,
            "Windows 2000/XP only" => Platform::WindowsXP,
            "Windows 7 Only" => Platform::Windows7,
            "Android" => Platform::Android,
            "iOS" => Platform::Ios,
            _ => Platform::Other,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::Dos => "DOS",
            Platform::MacOSX => "Mac OS X",
            Platform::Multiplatform => "Multiplatform",
            Platform::Unix => "Unix/linux etc.",
            Platform::Windows => "Windows",
            Platform::WindowsXP => "Windows 2000/XP only",
            Platform::Windows7 => "Windows 7 Only",
            Platform::Android => "Android",
            Platform::Ios => "iOS",
            Platform::Other => "Unspecified/other",
        };
        write!(f, "{}", s)
    }
}

// curl -X GET -I 'https://crackmes.one/search'
pub async fn get_search_token() -> Result<(String, String), CoreError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://crackmes.one/search")
        .send()
        .await
        .map_err(|_| CoreError::GetToken)?;

    let set_cookie = response
        .headers()
        .get("set-cookie")
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(CoreError::GetToken)?;

    let cookies: Vec<&str> = set_cookie.split(";").collect();
    let token_parts: Vec<&str> = cookies[0].split("=").collect();
    let cookie_token = token_parts[1].to_string();

    let body = response.text().await.map_err(|_| CoreError::GetToken)?;

    // Find the input tag with id="token" and extract its value attribute
    let html_token = body
        .lines()
        .find(|line| line.contains(r#"id="token""#))
        .and_then(|line| {
            line.split("value=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
        })
        .ok_or(CoreError::GetToken)?;

    Ok((cookie_token.to_string(), html_token.to_string()))
}

pub async fn get_challenge_list(
    params: SearchParams,
    cookie_token: String,
) -> Result<Vec<Challenge>, CoreError> {
    let client = reqwest::Client::new();

    let body = params.to_string();
    let cookie_header = format!("gosess={}", cookie_token);

    let response = client
        .post("https://crackmes.one/search")
        .header("Cookie", cookie_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Request error: {:?}", e);
            CoreError::GetChallengeList
        })?;

    let html_text = response.text().await.map_err(|e| {
        eprintln!("Response text error: {:?}", e);
        CoreError::GetChallengeList
    })?;

    // Debug: save HTML to file for inspection
    eprintln!("Response HTML length: {}", html_text.len());

    // Parse the HTML
    let document = Html::parse_document(&html_text);
    let row_selector = Selector::parse(r#"tbody#content-list tr"#).unwrap();

    let mut challenges = Vec::new();
    let mut row_count = 0;

    for row in document.select(&row_selector) {
        row_count += 1;
        if let Some(challenge) = parse_challenge_row(&row) {
            challenges.push(challenge);
        } else {
            eprintln!("Failed to parse row {}", row_count);
        }
    }

    eprintln!(
        "Found {} rows, parsed {} challenges",
        row_count,
        challenges.len()
    );

    Ok(challenges)
}

fn parse_challenge_row(row: &scraper::ElementRef) -> Option<Challenge> {
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let tds: Vec<_> = row.select(&td_selector).collect();

    if tds.len() < 9 {
        return None;
    }

    // Extract name and URL from first td
    let first_td = tds.first()?;
    let link = first_td.select(&a_selector).next()?;
    let url = link.value().attr("href")?.to_string();
    let name = link.text().collect::<String>().trim().to_string();

    // Extract author from second td
    let author_td = tds.get(1)?;
    let author = author_td
        .select(&a_selector)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    // Extract language from third td
    let language_td = tds.get(2)?;
    let language_str = language_td.text().collect::<String>().trim().to_string();
    let language = Language::from(language_str.as_str());

    // Extract arch from fourth td
    let arch_td = tds.get(3)?;
    let arch_str = arch_td.text().collect::<String>().trim().to_string();
    let arch = Arch::from(arch_str.as_str());

    // Extract difficulty from fifth td
    let difficulty_td = tds.get(4)?;
    let difficulty = difficulty_td
        .text()
        .collect::<String>()
        .trim()
        .parse::<f32>()
        .ok()?;

    // Extract quality from sixth td
    let quality_td = tds.get(5)?;
    let quality = quality_td
        .text()
        .collect::<String>()
        .trim()
        .parse::<f32>()
        .ok()?;

    // Extract platform from seventh td
    let platform_td = tds.get(6)?;
    let platform_str = platform_td.text().collect::<String>().trim().to_string();
    let platform = Platform::from(platform_str.as_str());

    // Extract writeups from ninth td (skip eighth which is date)

    Some(Challenge {
        name,
        url,
        author,
        language,
        arch,
        difficulty,
        quality,
        platform,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_search_token_returns_something() {
        let result = get_search_token().await;
        assert!(result.is_ok(), "Expected Ok result, got error");

        let (cookie_token, html_token) = result.unwrap();
        assert!(!cookie_token.is_empty(), "Expected non-empty cookie token");
        assert!(!html_token.is_empty(), "Expected non-empty HTML token");
    }
}
