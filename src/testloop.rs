#[feature(non_ascii_idents)];
extern mod rustc;
extern mod extra;
use std::io;
use std::io::fs;
use std::os;
use std::run;
use std::str;

#[main]
fn testloop () {
   let args = os::args();

   let dir = ".";
   let mut latest = 0;

   loop {
      let (has_changed, latest_) = modified_since(latest, dir);
      latest = latest_;

      if has_changed {
         request_build(args)
      }

      io::timer::sleep(200);
   }

   //os::set_exit_status(0);
}

fn modified(path: Path) -> u64 {
   let st = fs::stat(&path);
   return st.modified;
}

fn last_modified(path: &str) -> u64 {
   let pp = Path::new(path);
   let wi: fs::WalkIterator = fs::walk_dir(&pp);

   let mut rs_files = wi.filter(|path| {
      match path.extension_str() {
         None      => { false }
         Some(ext) => { ext == "rs" }
      }
   });

   let latest: Option<Path> =
      rs_files.max_by(|p| modified(p.clone()));

   match latest {
      None       => { 0 }
      Some(path) => { modified(path) }
   }
}

fn modified_since(latest: u64, dir: &str) -> (bool, u64) {
   let new_latest = last_modified(dir);
   (last_modified(dir) > latest, new_latest)
}

// run and selectively print results of `process_output`
fn run(exe: &str, args: &[~str])
   -> Option<io::process::ProcessExit> {

   match run::process_output(exe, args) {
      None => {
         println!("Failed to run `{}` with args: {:?}", exe, args);
         return None;
      }
      Some(ps) => {
         let out: &str = str::from_utf8(ps.output);
         let err: &str = str::from_utf8(ps.error);
         let stat: io::process::ProcessExit = ps.status;

         if out.len() > 0 {
            println!("STDOUT:\n{}", out);
         }
         if err.len() > 0 {
            println!("STDERR:\n{}", err);
         }
         if stat != io::process::ExitStatus(0) {
            println!("STATUS: {:?}", stat);
         }
         return Some(stat);
      }
   }
}

// test if a path is a file (and not missing or something else)
fn is_file(path: &str) -> bool {
   let p = Path::new(path);
   match io::result(|| fs::stat(&p)) {
      Ok(stat) => {
         match stat.kind {
            io::TypeFile => { return true; }
            _            => { return false; }
         }
      }
      Err(_e) => {
         // println!("Error calling `stat` on path `{}`: {:?}", path, e)
         return false;
      }
   }
}

fn request_build(args: &[~str]) {
   let crate    : ~str    = args[1].to_str();
   let test_bin : ~str    = ~"./.tests_in_loop.exe";
   let test_args: &[~str] = args.slice_from(2);

   if is_file(crate) {
      // cleanup
      if is_file(test_bin) {
         let p = Path::new(test_bin.clone());
         fs::unlink(&p);
      }
      
      // build
      println("<<<< building tests >>>>");
      //run("/usr/local/bin/rustc",
      rustc::main_args(
         [~"-o", test_bin.clone(),
          ~"--test", crate,
          ~"--allow", ~"dead_code",
          ~"--opt-level", ~"0"]);

      // run
      if is_file(test_bin) {
         println("<<<< running tests >>>>");
         run(test_bin, test_args);
      }
      else {
         println("<<<< NOT running tests >>>>");
      }
   }
}



