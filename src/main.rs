use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Write};
use chardetng::EncodingDetector;
use regex::Regex;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref REGEX_LIST: Vec<Regex> = vec![
        Regex::new(r"\b(creado(s)?|subtitu(lo|los|lado|lada|lados)|subtítu(lo|los|lado|lada|lados)|descarg(ado|ar)|(re-?)?sinc(ed|ro(nizado|nizados|nizacion|nización)?)?|modific(ado|ados|ion|iones|ión|iónes)|traduc(e|ido|idos|tora|cion|ciones|ción|ciónes)|correcc(iones|ion|ión|iónes)|correg(ir|ido|idos)|transcri(bido|pcion|pciones|pción|pciónes)|mejor(ado|amientos)|adaptado|ripeo|arreglos|subs|hecha)\W*(por|de|by)?\W*(:|;)\b").expect("Invalid regex"),
        Regex::new(r"\b(Visiontext|Filmtrans|CARLISHIO|HGWizard|LASERFILM|Fhercho06|Cinesotano|jantoniot|Caichac|cemauli|Drakul|Scarlata|laloonda|japezoa|MarcusL|Kikeguate|KIKEGT|Zagon|KingCreole|Mothernatura|MaLTRaiN|FRH|GCas87|maryluzesp|Marenys|ByAlbis02|ana24horas|Fernando355|Zagonsubs|ikerslot|menoyos|Axel7902|vNaru|livinginthepast|patagonikus|Macias Group|EasyTechOficial|mlmlte|LiarsTeam|OnceUponATEAM)\b").expect("Invalid regex"),
        Regex::new(r"\b(juanchojb|shogun87|Rocio190889|darklin01|R@ul|Mabeas|akallabeth|NicoDipaolo|OsirisTSF|Lord Avestruz|LadyJenny|jeslil7|Giobatta SA|MementMori|la_bestia1962|Natuchia|JJ Porto|marchelo64|c\. oper|SHADOW84\Anfegopi|perroubuntero|Kumara|JosephPools|natycuac|ibvil|SwSub|DarKsh|ShalimarFox|R\[H\]ésus AB\+ Team|Mat Productions|S\. C\. Bananas|Bakugan|M-Rok|YYeTTs|robermgs)\b").expect("Invalid regex"),
        Regex::new(r"\b(admitme|argenteam|finalanime|subtitulamos|substeam|subdivx|tusubtitulo|thesubfactory|Open Subtitles|miembro VIP|osdb\.link|TranslatorsInc|Translators, Inc|TranslatorslncSubs\.blogspot\.com\.es|Southparkspanish|SUBTITULOS\.es|SUBITULOS\.es|SouthParkNews\.net|subtitules\.es|ShooCat|YYeTs|TaMaBin|P@bs|gratispeliculas|SubAdictos|SerieCanal|playships\.eu|tusseries\.com|subswiki\.com|Subs-Team|SUBTÍTULOS\.ES|U\-Sub\.net)\b").expect("Invalid regex"),
        // Add more patterns as needed
    ];
}


fn detect_encoding(data: &[u8]) -> &'static encoding_rs::Encoding {
    let mut detector = EncodingDetector::new();
    detector.feed(data, true);
    detector.guess(None, true)
}

fn process_line(line: &str, regex_list: &[Regex]) -> bool {
    let mut any_match = false;
    for regex in regex_list {
        if regex.is_match(line) {
            println!("Matched line: {}", line);
            any_match = true;
        }
    }
    any_match
}

fn convert_to_utf8(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut content = Vec::new();
    File::open(input_path)?.read_to_end(&mut content)?;

    let encoding = detect_encoding(&content);
    println!("Detected encoding: {}", encoding.name());
    let (decoded, _, _) = encoding.decode(&content);

    let output_file = File::create(output_path)?;
    let mut output_writer = io::BufWriter::new(output_file);

    for line in BufReader::new(decoded.as_bytes()).lines() {
        let line = line?;
        process_line(&line, &REGEX_LIST);
        writeln!(output_writer, "{}", line)?;
    }

    Ok(())
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() != 3 {
        eprintln!("Usage: {} <input_path> <output_path>", args[0]);
        std::process::exit(1);
    }

    // Get input and output paths from command line arguments
    let input_path = &args[1];
    let output_path = &args[2];

    // Call the convert_to_utf8 function with the provided paths
    if let Err(err) = convert_to_utf8(input_path, output_path) {
        eprintln!("Error: {}", err);
    } else {
        println!("File converted successfully!");
    }
}






