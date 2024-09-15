#[cfg(test)]
mod run_tests {
    use std::fs;
     use colored::Colorize;
    use crate::runw;

    #[derive(Debug, Clone)]
    struct FileCase {
        fname: String,
        run_code: String,
        expected: String,
    }

    impl FileCase {
        fn new() -> Self {
            FileCase {
                fname: "".to_string(),
                run_code: "".to_string(),
                expected: "".to_string(),
            }
        }

        fn update_case(&mut self, case_suffix: String, p: std::borrow::Cow<'_, str>) {
            let data = &fs::read_to_string(p.as_ref())
                .expect(format!("Should be abe to read from {}", p).as_str());

            match case_suffix.as_str() {
                "lox" => self.run_code.push_str(data),
                "out" => self.expected.push_str(data),
                _ => (),
            }
            eprint!(
                "update case {:?}: case_name:{}, case suffix: {} ,data: {}\n",
                self, self.fname, case_suffix, data
            );
        }
    }

    fn prepare(dir_path: &str) -> Result<Vec<FileCase>, String> {
        use std::{
            collections::HashMap,
            fs::{self, DirEntry},
        };
        let mut map: HashMap<String, FileCase> = HashMap::new();

        let mut update_cases = |de: std::io::Result<DirEntry>| match de {
            Err(_e) => (),
            Ok(entry) => {
                let pat = entry.path();
                let p = pat.to_string_lossy();

                let case_split = p.split(".").collect::<Vec<_>>();
                let case_name = case_split
                    .get(0)
                    .expect(format!("Extract file name should suceed with {}", p).as_str())
                    .to_string();
                let case_suffix = case_split
                    .get(1)
                    .expect(format!("Extract file name should suceed with {}", p).as_str())
                    .to_string();
                if vec!["lox".to_string(), "out".to_string()].contains(&case_suffix) {
                    eprint!(
                        "split: {:?}, pat: {:?}, case_name: {}, case_suffix: {}\n",
                        case_split,
                        pat.to_str(),
                        case_name,
                        case_suffix
                    );
                    map.entry(case_name)
                        .or_insert(FileCase::new())
                        .update_case(case_suffix, p);
                    eprint!("Map len: {:?}\n", map.len());
                }
            }
        };
        let _ = fs::read_dir(dir_path).map(|f| {
            f.for_each(|dirent: std::io::Result<DirEntry>| {
                update_cases(dirent);
            })
        });

        Ok(map
            .iter_mut()
            .map(|(fname, fc)| {
                fc.fname = fname.to_string();
                fc.clone()
            })
            .collect::<Vec<_>>())
    }

    #[test]
    fn run_all_lox_and_out_files() {
        if let Ok(v) = prepare("src/tests") {
            let mut copy = Vec::from(v);
            copy.sort_by_key(|fc|fc.fname.clone());
            for el in copy {
                eprint!("Test {}", el.fname);
                let mut out = std::io::BufWriter::new(Vec::new());
                let mut err = std::io::BufWriter::new(Vec::new());

                runw(&mut out, &mut err, &el.run_code);
                let out_result = String::from_utf8(out.into_inner().unwrap()).ok().unwrap();
                let err_result = String::from_utf8(err.into_inner().unwrap()).ok().unwrap();
                eprintln!(
                    "-----> Case {}:\ntext:{}\nexp:{}\nout:{}\nerr:{}\n",
                    el.fname.green(),
                    el.run_code.yellow(),
                    el.expected.magenta(),
                    out_result.green(),
                    err_result.red(),
                );
                let actual_output = format!("{}{}", out_result, err_result);
                println!("-->{:?}\n-->{:?}", actual_output, el.expected);
                //assert!(actual_output.eq(&el.expected), "Error in {}", el.fname);

            }
        }
    }
}
