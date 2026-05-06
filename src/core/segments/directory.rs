use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectorySegment {
    path_depth: usize,
}

impl DirectorySegment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path_depth(mut self, depth: usize) -> Self {
        self.path_depth = depth;
        self
    }

    /// Extract directory name from path, handling both Unix and Windows separators.
    ///
    /// - `path_depth == 0`: show the full path as-is.
    /// - `path_depth == 1`: show the last component only (original behavior).
    /// - `path_depth > 1`: show the last N components, prefixed with "…/" (or "…\\")
    ///   when the path is deeper than N levels.
    fn extract_directory_name(path: &str, path_depth: usize) -> String {
        if path_depth == 0 {
            return path.to_string();
        }

        // Detect the dominant path separator
        let has_unix = path.contains('/');
        let has_windows = path.contains('\\');
        let sep = if has_windows && !has_unix { '\\' } else { '/' };

        let parts: Vec<&str> = path.split(sep).filter(|s| !s.is_empty()).collect();

        if parts.is_empty() {
            return "root".to_string();
        }

        if path_depth == 1 {
            // Preserve original single-component logic for full compatibility
            let last = parts.last().unwrap();
            if last.is_empty() {
                "root".to_string()
            } else {
                last.to_string()
            }
        } else if parts.len() <= path_depth {
            // Path is shallow enough — show it unchanged
            path.to_string()
        } else {
            // Truncate to the last N components
            let tail = parts[parts.len() - path_depth..].join(&sep.to_string());
            format!("…{}{}", sep, tail)
        }
    }
}

impl Segment for DirectorySegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let current_dir = &input.workspace.current_dir;

        let dir_name = Self::extract_directory_name(current_dir, self.path_depth);

        let mut metadata = HashMap::new();
        metadata.insert("full_path".to_string(), current_dir.clone());

        Some(SegmentData {
            primary: dir_name,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Directory
    }
}
