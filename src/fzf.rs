use crate::error::{AppError, Result};
use crate::history::Conversation;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Run fzf to allow the user to select a conversation
pub fn select_conversation(conversations: &[Conversation]) -> Result<PathBuf> {
    let mut child = Command::new("fzf")
        .args([
            "--height",
            "40%",
            "--reverse",
            "--border",
            "--no-multi",
            "--delimiter",
            "\t",
            "--with-nth",
            "2",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppError::FzfExecutionError(e.to_string()))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| AppError::FzfExecutionError("Failed to open stdin".to_string()))?;

        for conv in conversations {
            let timestamp = conv.timestamp.format("%b %d, %H:%M");
            let display_part = format!("[{}] {} | {}", conv.index, timestamp, conv.preview);
            // Format: INDEX<tab>DISPLAY_PART<tab>FULL_TEXT
            writeln!(
                stdin,
                "{}\t{}\t{}",
                conv.index, display_part, conv.full_text
            )?;
        }
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(AppError::SelectionCancelled);
    }

    let selection = String::from_utf8_lossy(&output.stdout);
    let selection = selection.trim();

    if selection.is_empty() {
        return Err(AppError::SelectionCancelled);
    }

    // Extract index from the first tab-separated field
    if let Some(idx_str) = selection.split('\t').next()
        && let Ok(idx) = idx_str.parse::<usize>()
    {
        return conversations
            .get(idx)
            .map(|c| c.path.clone())
            .ok_or(AppError::IndexOutOfRange(idx));
    }

    Err(AppError::FzfSelectionParseError)
}
