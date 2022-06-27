use colored::Colorize;
use serde::Serialize;
use std::{fmt, time::Duration};

#[derive(Serialize)]
pub struct FunctionRunResult {
    pub name: String,
    pub runtime: Duration,
    pub size: u64,
    pub memory_usage: u64,
    pub logs: String,
    pub output: serde_json::Value,
}

impl FunctionRunResult {
    pub fn new(
        name: String,
        runtime: Duration,
        size: u64,
        memory_usage: u64,
        output: serde_json::Value,
        logs: String,
    ) -> Self {
        FunctionRunResult {
            name,
            runtime,
            size,
            memory_usage,
            output,
            logs,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|error| error.to_string())
    }
}

impl fmt::Display for FunctionRunResult {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let title = "      Benchmark Results      ".black().on_bright_green();
        write!(formatter, "{}\n\n", title)?;
        writeln!(formatter, "Name: {}", self.name)?;
        writeln!(formatter, "Runtime: {:?}", self.runtime)?;
        writeln!(formatter, "Memory Usage: {}KB", self.memory_usage)?;
        writeln!(formatter, "Size: {}KB\n", self.size / 1024)?;

        writeln!(
            formatter,
            "{}\n\n{}",
            "            Logs             ".black().on_bright_blue(),
            self.logs
        )?;

        writeln!(
            formatter,
            "Output:\n{}",
            serde_json::to_string_pretty(&self.output).unwrap_or_else(|error| error.to_string())
        )?;

        Ok(())
    }
}
