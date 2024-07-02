//! Utilities for running commands.
use crossbeam::queue::{ArrayQueue, SegQueue};
use std::io::{Read, Write};
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use std::sync::Arc;
use tracing::{debug, warn};

fn _just_read_stdout(tx: &Arc<SegQueue<u8>>, stop: &Arc<ArrayQueue<()>>, mut fd: ChildStdout) {
    while stop.is_empty() {
        let mut buf = [0];
        match fd.read(&mut buf) {
            Ok(1) => tx.push(buf[0]),
            Err(error) => return warn!(?error, "Fail to read stdout."),
            _ => continue,
        }
    }
}

fn _just_read_stderr(tx: &Arc<SegQueue<u8>>, stop: &Arc<ArrayQueue<()>>, mut fd: ChildStderr) {
    while stop.is_empty() {
        let mut buf = [0];
        match fd.read(&mut buf) {
            Ok(1) => tx.push(buf[0]),
            Err(error) => return warn!(?error, "Fail to read stderr."),
            _ => continue,
        }
    }
}

#[tracing::instrument]
pub fn read_while_show_output(
    cmd: &mut Command,
    prefix: Option<&str>,
) -> std::io::Result<(Option<i32>, String, String)> {
    let prefix = prefix.unwrap_or("");
    let (newline, newrline) = (format!("\n{prefix}"), format!("\r{prefix}"));
    let (outq, errq): (Arc<SegQueue<u8>>, Arc<SegQueue<u8>>) = (Arc::default(), Arc::default());
    let (outstop, errstop) = (Arc::new(ArrayQueue::new(1)), Arc::new(ArrayQueue::new(1)));
    // clone the arcs for putting into closure
    let (outqc, errqc, outstopc, errstopc) =
        (outq.clone(), errq.clone(), outstop.clone(), errstop.clone());

    let mut hdl = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let (stdout, stderr) = (hdl.stdout.take().unwrap(), hdl.stderr.take().unwrap());
    let outhdl = std::thread::spawn(move || _just_read_stdout(&outqc, &outstopc, stdout));
    let errhdl = std::thread::spawn(move || _just_read_stderr(&errqc, &errstopc, stderr));
    let (mut out, mut err) = (String::new(), String::new());
    let (mut tmpoutbytes, mut tmperrbytes) = (vec![], vec![]);

    let pid = process_alive::Pid::from(hdl.id());
    while process_alive::state(pid).is_alive() {
        while let Some(c) = outq.pop() {
            tmpoutbytes.push(c);
        }
        if let Ok(sout) = core::str::from_utf8(&tmpoutbytes) {
            out.push_str(sout);
            let s = sout.replace('\n', &newline).replace('\r', &newrline);
            std::io::stdout().write_all(s.as_bytes())?;
            tmpoutbytes.clear();
        }

        while let Some(c) = errq.pop() {
            tmperrbytes.push(c);
        }
        if let Ok(serr) = core::str::from_utf8(&tmperrbytes) {
            err.push_str(serr);
            let s = serr.replace('\n', &newline).replace('\r', &newrline);
            std::io::stderr().write_all(s.as_bytes())?;
            tmperrbytes.clear();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    debug!("Command execution finished, joining threads");
    outstop.push(()).unwrap();
    errstop.push(()).unwrap();
    outhdl.join().expect("Fail to join stdout handle thread.");
    errhdl.join().expect("Fail to join stderr handle thread.");
    Ok((hdl.wait()?.code(), out, err))
}
