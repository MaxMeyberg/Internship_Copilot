# LinkedIn Outreach Automation Tool

A Rust-based tool that automates personalized LinkedIn outreach by analyzing profiles, finding email addresses, and generating custom messages using GPT.

## What This Thing Does

Takes a LinkedIn URL, gets the person's profile data, finds their email, analyzes how you might connect with them based on your background, and spits out a personalized email ready to send.

## How It Works

### The Pipeline

1. **Profile Analysis**: Uses Apify to gather LinkedIn profile data (experience, education, skills, etc.)
2. **Email Discovery**: Tries to find their email using Zeliq API (or you can type it manually)
3. **Connection Analysis**: GPT compares their background with yours to find genuine connection points
4. **Email Generation**: Creates a personalized outreach email based on shared experiences
5. **Final Polish**: Adds personality and formats as a mailto link

### The LLM Chain

- **LLM1**: Parses the raw LinkedIn JSON into readable format
- **LLM2**: Finds connections between their profile and your background
- **LLM3**: Writes the actual email based on connection analysis
- **LLM4**: Adds personality and converts to mailto URL

## Setup:

- **TBA**

### Prerequisites

- Rust (obviously)
- API keys for:
  - OpenAI (for GPT calls)
  - Apify (for LinkedIn data)
  - Zeliq (for email finding)

### Environment Variables

Create a `.env` file:

```env
OPENAI_API_KEY=your_openai_key
APIFY_API_KEY=your_apify_key
ZELIQ_API_KEY=your_zeliq_key
```

### Personal Context

Create `personal_context.txt` with your background:

```txt
**EDUCATION:**
- Stanford University (Current Student)
  * Master of Science - Computer Science (AI Focus) | Apr 2024 - Jun 2026
  * Transfer student background

**TECHNICAL SKILLS:**
- Programming: Python, Rust, C++, JavaScript
- Focus Areas: AI/ML, ETL pipelines, data processing

**EXPERIENCE:**
- Healthcare tech internships
- Cross-functional engineering + PM work
```

## Usage

```bash
cargo run -- "https://www.linkedin.com/in/someone/"
```

The tool will:
1. Ask if you want to manually enter an email or auto-find it
2. Analyze their LinkedIn profile
3. Generate a personalized outreach email
4. Output a mailto link ready to use

## Code Example

Here's how the main pipeline works:

## Project Structure

```
src/
├── main.rs              # Main pipeline coordination
├── apis/
│   ├── mod.rs
│   ├── apify_call.rs    # LinkedIn profile data
│   ├── zeliq.rs         # Email discovery
│   ├── appollo.rs       # Alternative email finder
│   └── gpt.rs           # OpenAI API calls
├── personal_context.txt # Your background info
└── prompt files/        # LLM prompts for each step
    ├── llm1_parse_json.txt
    ├── llm2_summarize_info.txt
    ├── llm3_compose_letter.txt
    └── llm4_add_personality_mailto.txt
```

## The Anti-Sycophancy System

The prompts are specifically designed to avoid generic, overly flattering language. Instead of "I was blown away by your incredible journey," it generates stuff like "I noticed your work on React optimization at Google."

## Dependencies

Check `Cargo.toml` for the full list, but main ones:
- `tokio` (async runtime)
- `reqwest` (HTTP client)
- `serde_json` (JSON handling)
- `anyhow` (error handling)
- `dotenv` (environment variables)

## Current Limitations

- Only works with public LinkedIn profiles
- Email finding depends on third-party APIs
- GPT costs money for each run
- No response tracking (yet)

## Future Ideas

- Vector database integration for learning from successful outreach
- Multiple email provider fallbacks
- Response tracking and analytics
- Batch processing multiple profiles

## Why Rust?

Because why not? Also it's fast, handles async well, and the error handling with `anyhow` makes the API calls pretty clean.

---

Built for automating the tedious parts of networking while keeping the personal touch. Don't use this to spam people.
