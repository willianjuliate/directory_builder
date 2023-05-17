use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct NewFolder {
    name_project: String,
    quantity_modules: u32,
    folders_default: Vec<String>,
}
impl NewFolder {
    pub fn new() -> NewFolder {
        NewFolder {
            name_project: String::from("SEM_NOME"),
            quantity_modules: 0,
            folders_default: vec![
                String::from("IMAGENS"),
                String::from("MASCARAS"),
                String::from("AUDIOS"),
                String::from("DOCUMENTOS"),
                String::from("ROTEIROS"),
                String::from("VIDEOS"),
                String::from("ZIP"),
                String::from("PROJETOS FINAIS"),
            ],
        }
    }
    pub fn set_name_project(&mut self, name_project: String) {
        if !name_project.is_empty() {
            self.name_project = name_project;
            println!("NOME DO PROJETO: {}", self.name_project);
        } else {
            println!("NOME DO PROJETO: {}", self.name_project);
        }
    }
    pub fn set_quantity_modules(&mut self, quantity_modules: u32) {
        if quantity_modules > 0 {
            self.quantity_modules = quantity_modules;
        }
    }
    pub fn add_new_folders(&mut self, names_folder: String) {
        if !names_folder.is_empty() {
            for names_f in names_folder.split(",") {
                let name = names_f.trim().to_string();
                if name != " ".to_string() && name != ",".to_string() {
                    self.folders_default.push(name);
                }
            }
        }
    }
    pub fn remove_folders(&mut self, names_folder: String) {
        if !names_folder.is_empty() {
            for names_f in names_folder.split(",") {
                let list_dir = &self.folders_default;
                let name = names_f.trim().to_string();

                let index = match self.get_index(&list_dir, name) {
                    Some(value) => format!(
                        "O DIRETÓRIO '{}' FOI REMOVIDO",
                        self.folders_default.remove(value)
                    ),
                    None => format!("ESSE DIRETÓRIO '{}' NÃO EXISTE!", names_f),
                };
                println!("{}", index);
            }
        }
    }
    pub fn create_dir_and_files(&self) -> String {
        let name_project = &self.name_project;
        match fs::read_dir(format!("./{}", name_project)) {
            Ok(_) => return format!("# PROJETO '{}' JÁ EXISTE! #", name_project),
            Err(_) => {
                for dir in &self.folders_default {
                    let root = format!("./{}/{}", name_project, dir);
                    fs::create_dir_all(root).expect("# NÃO FOI POSSÍVEL CRIAR OS DIR's! #");
                }
                self.create_file_tscproj();
                return format!("# PROJETO '{}' CONSTRUÍDO #", name_project);
            }
        }
    }
    pub fn show_dirs(&mut self) {
        self.folders_default.sort();

        println!("\n├─ {}", self.name_project);
        for dir in &self.folders_default {
            if !dir.is_empty() {
                println!("│     ├─ {}", dir);
            }
        }
    }

    fn get_index(&self, vect: &Vec<String>, name_dir: String) -> Option<usize> {
        let mut cont = 0;
        for dirs in vect {
            if dirs == &name_dir {
                return Some(cont);
            }
            cont += 1;
        }
        None
    }

    fn create_file_tscproj(&self) {
        let string = r#"{
    "title": "",
    "description": "",
    "author": "",
    "targetLoudness": -18.0,
    "shouldApplyLoudnessNormalization": true,
    "videoFormatFrameRate": 30,
    "audioFormatSampleRate": 44100,
    "width": 1440.0,
    "height": 900.0,
    "version": "5.0",
    "editRate": 30,
    "authoringClientName": {
        "name": "Camtasia",
        "platform": "Windows",
        "version": "21.0"
    },
    "timeline": {
        "id": 1,
        "sceneTrack": {
            "scenes": [
                {
                    "csml": {
                        "tracks": [
                            {
                                "trackIndex": 0,
                                "medias": []
                            },
                            {
                                "trackIndex": 1,
                                "medias": []
                            }
                        ]
                    }
                }
            ]
        },
        "captionAttributes": {
            "enabled": true,
            "fontName": "Arial",
            "fontSize": 53,
            "backgroundColor": [
                0,
                0,
                0,
                191
            ],
            "foregroundColor": [
                255,
                255,
                255,
                255
            ],
            "lang": "pt",
            "alignment": 0,
            "defaultFontSize": true,
            "opacity": 0.5,
            "backgroundEnabled": true,
            "backgroundOnlyAroundText": true
        },
        "gain": 1.0,
        "legacyAttenuateAudioMix": true,
        "backgroundColor": [
            0,
            0,
            0,
            255
        ]
    },
    "metadata": {
        "AutoSaveFile": "",
        "CanvasZoom": 75,
        "Date": "",
        "IsAutoSave": false,
        "Language": "PTB",
        "ProfileName": "",
        "Title": "",
        "audioNarrationNotes": "",
        "calloutStyle": "Basic"
    }
}"#;

        for i in 0..self.quantity_modules {
            let root = format!(
                "{}/{}_#{}.tscproj",
                &self.name_project,
                &self.name_project,
                i + 1
            );
            let mut tscproj = File::create(root).expect("# NÃO FOI POSSÍVEL CRIAR OS FILE's! #");
            write!(tscproj, "{}", string).expect("Err");
        }
    }
}
