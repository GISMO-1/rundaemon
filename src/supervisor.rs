use crate::config::{Service, Restart};
use anyhow::{anyhow, Result};
use tokio::{process::Command, io::{AsyncBufReadExt, BufReader}, select, signal};
use tracing::{info, warn, error, Level};
use std::process::Stdio;

pub async fn run(services: Vec<Service>) -> Result<()> {
    let mut tasks = Vec::new();
    for svc in services {
        tasks.push(tokio::spawn(run_one(svc)));
    }
    // Wait for Ctrl+C then propagate shutdown by dropping tasks
    signal::ctrl_c().await?;
    info!("shutdown signal received");
    Ok(())
}

async fn run_one(svc: Service) -> Result<()> {
    loop {
        info!(svc=%svc.name, "starting");
        let mut cmd = Command::new(&svc.cmd);
        cmd.args(&svc.args);
        if let Some(dir) = &svc.working_dir { cmd.current_dir(dir); }
        for kv in &svc.env { cmd.env(&kv.key, &kv.value.clone()); }
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| anyhow!("spawn failed: {e}"))?;
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let name = svc.name.clone();
        let out_task = tokio::spawn(pipe_logs(name.clone(), stdout, Level::INFO));
        let err_task = tokio::spawn(pipe_logs(name.clone(), stderr, Level::ERROR));

        let status = child.wait().await?;
        out_task.abort(); let _ = out_task.await;
        err_task.abort(); let _ = err_task.await;

        match (status.success(), svc.restart) {
            (true, Restart::Always) => { info!(svc=%svc.name, "exited ok, restarting"); }
            (true, Restart::OnFailure) => { info!(svc=%svc.name, "exited ok, not restarting"); break; }
            (true, Restart::Never) => break,
            (false, Restart::Never) => { error!(svc=%svc.name, code=?status.code(), "crashed, not restarting"); break; }
            (false, _) => { warn!(svc=%svc.name, code=?status.code(), "crashed, restarting"); }
        }
    }
    Ok(())
}

async fn pipe_logs(name: String, stream: impl tokio::io::AsyncRead + Unpin, level: tracing::Level) {
    let mut reader = BufReader::new(stream).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        match level {
            tracing::Level::INFO => tracing::info!(svc=%name, "{}", line),
            tracing::Level::ERROR => tracing::error!(svc=%name, "{}", line),
            _ => tracing::debug!(svc=%name, "{}", line),
        }
    }
}
