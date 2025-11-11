use crate::search::{Arch, Language, Platform, SearchParams};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "cracked")]
#[command(about = "Browse and download crackmes from crackmes.one", long_about = None)]
pub struct Cli {
    /// Challenge name to search for
    #[arg(short, long)]
    pub name: Option<String>,

    /// Challenge author to search for
    #[arg(short, long)]
    pub author: Option<String>,

    /// Difficulty level
    #[arg(short, long)]
    pub difficulty: Option<DifficultyLevel>,

    /// Quality level
    #[arg(short, long)]
    pub quality: Option<QualityLevel>,

    /// Programming language
    #[arg(short, long)]
    pub language: Option<LanguageArg>,

    /// Architecture
    #[arg(long)]
    pub arch: Option<ArchArg>,

    /// Platform
    #[arg(short, long)]
    pub platform: Option<PlatformArg>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DifficultyLevel {
    #[value(name = "easy")]
    Easy,
    #[value(name = "medium")]
    Medium,
    #[value(name = "hard")]
    Hard,
    #[value(name = "hardcore")]
    Hardcore,
}

impl DifficultyLevel {
    pub fn to_range(self) -> [i32; 2] {
        match self {
            DifficultyLevel::Easy => [1, 2],
            DifficultyLevel::Medium => [2, 3],
            DifficultyLevel::Hard => [3, 4],
            DifficultyLevel::Hardcore => [4, 5],
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum QualityLevel {
    Poor,
    Flaky,
    Good,
    Mint,
}

impl QualityLevel {
    pub fn to_range(self) -> [i32; 2] {
        match self {
            QualityLevel::Poor => [1, 2],
            QualityLevel::Flaky => [2, 3],
            QualityLevel::Good => [3, 4],
            QualityLevel::Mint => [4, 5],
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LanguageArg {
    #[value(name = "c")]
    Ccpp,
    #[value(name = "asm")]
    Assembler,
    #[value(name = "java")]
    Java,
    #[value(name = "go")]
    Go,
    #[value(name = "rust")]
    Rust,
    #[value(name = "wasm")]
    Wasm,
    #[value(name = "basic")]
    Basic,
    #[value(name = "delphi")]
    Borland,
    #[value(name = "pascal")]
    Pascal,
    #[value(name = "dotnet")]
    Dotnet,
    #[value(name = "other")]
    Other,
}

impl From<LanguageArg> for Language {
    fn from(lang: LanguageArg) -> Self {
        match lang {
            LanguageArg::Ccpp => Language::Ccpp,
            LanguageArg::Assembler => Language::Assembler,
            LanguageArg::Java => Language::Java,
            LanguageArg::Go => Language::Go,
            LanguageArg::Rust => Language::Rust,
            LanguageArg::Wasm => Language::Wasm,
            LanguageArg::Basic => Language::Basic,
            LanguageArg::Borland => Language::Borland,
            LanguageArg::Pascal => Language::Pascal,
            LanguageArg::Dotnet => Language::Dotnet,
            LanguageArg::Other => Language::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ArchArg {
    #[value(name = "x86")]
    X86,
    #[value(name = "x64")]
    X8664,
    #[value(name = "java")]
    Java,
    #[value(name = "arm")]
    Arm,
    #[value(name = "mips")]
    Mips,
    #[value(name = "riscv")]
    Riscv,
    #[value(name = "other")]
    Other,
}

impl From<ArchArg> for Arch {
    fn from(arch: ArchArg) -> Self {
        match arch {
            ArchArg::X86 => Arch::X86,
            ArchArg::X8664 => Arch::X8664,
            ArchArg::Java => Arch::Java,
            ArchArg::Arm => Arch::Arm,
            ArchArg::Mips => Arch::Mips,
            ArchArg::Riscv => Arch::RiscV,
            ArchArg::Other => Arch::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PlatformArg {
    #[value(name = "dos")]
    Dos,
    #[value(name = "macos")]
    MacOSX,
    #[value(name = "multiplatform")]
    Multiplatform,
    #[value(name = "unix")]
    Unix,
    #[value(name = "windows")]
    Windows,
    #[value(name = "winxp")]
    WindowsXP,
    #[value(name = "win7")]
    Windows7,
    #[value(name = "android")]
    Android,
    #[value(name = "ios")]
    Ios,
    #[value(name = "other")]
    Other,
}

impl From<PlatformArg> for Platform {
    fn from(platform: PlatformArg) -> Self {
        match platform {
            PlatformArg::Dos => Platform::Dos,
            PlatformArg::MacOSX => Platform::MacOSX,
            PlatformArg::Multiplatform => Platform::Multiplatform,
            PlatformArg::Unix => Platform::Unix,
            PlatformArg::Windows => Platform::Windows,
            PlatformArg::WindowsXP => Platform::WindowsXP,
            PlatformArg::Windows7 => Platform::Windows7,
            PlatformArg::Android => Platform::Android,
            PlatformArg::Ios => Platform::Ios,
            PlatformArg::Other => Platform::Other,
        }
    }
}

impl Cli {
    pub fn to_search_params(&self, token: String) -> SearchParams {
        // If no difficulty or quality is specified, default to full range (1-6)
        let difficulty_range = Some(self.difficulty.map(|d| d.to_range()).unwrap_or([1, 6]));
        let quality_range = Some(self.quality.map(|q| q.to_range()).unwrap_or([1, 6]));

        SearchParams {
            name: self.name.clone(),
            author: self.author.clone(),
            difficulty_range,
            quality_range,
            language: self.language.map(|l| l.into()),
            arch: self.arch.map(|a| a.into()),
            platform: self.platform.map(|p| p.into()),
            token,
        }
    }
}
