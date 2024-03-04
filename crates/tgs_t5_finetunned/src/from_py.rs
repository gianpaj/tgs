use dialoguer::{theme::ColorfulTheme, Select};
use pyo3::{prelude::*, types::PyModule};
use std::path::PathBuf;
use std::{env, fs};
use tgs_colors::custom;
use tgs_loader::LoadingIndicator;

fn call_dialoger(result: String) -> String {
    let multiselected = &[
        "\u{1f680} Execute command",
        "\u{270f} Edit and execute command",
        "\u{2718} Cancel",
    ];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt(result.clone())
        .items(&multiselected[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        if selection == 0 {
            result
        } else {
            //TODO allow user to edit the command
            //TODO send edited command to server for model improvement
            String::from("TODO")
        }
    } else {
        String::from("TODO")
    }
}

fn find_inference_model() -> Result<PathBuf, String> {
    // Path relative to the crate's location in the workspace
    let dev_path = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
        .join("inference_model.py");
    if dev_path.exists() {
        return Ok(dev_path);
    }

    // Path relative to the executable's location
    let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
    exe_path.pop();
    exe_path.push("inference_model.py");

    println!("Path: {:?}", exe_path);
    if exe_path.exists() {
        return Ok(exe_path);
    }

    Err("Failed to find inference_model.py in any known location.".to_string())
}

pub fn execute(input_text: &str) -> PyResult<String> {
    let loader = LoadingIndicator::new(custom::DARK_WHITE);
    pyo3::prepare_freethreaded_python();
    let executable_path = find_inference_model().expect("Failed to find inference_model.py");

    println!("Path: {:?}", executable_path);
    loader.start(input_text);
    Python::with_gil(|py| {
        let code = fs::read_to_string(&executable_path)?;

        let module = PyModule::from_code(py, &code, "inference_model.py", "inference_model")?;

        println!("Module: {:?}", module);

        let result: PyResult<String> = module
            .getattr("generate_answer")?
            .call1((input_text,))?
            .extract();
        loader.stop();
        Ok(call_dialoger(result?))
    })
}
