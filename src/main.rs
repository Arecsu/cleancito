use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Write};
use chardetng::EncodingDetector;
use regex::Regex;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref REGEX_CLEANUP_LIST: Vec<Regex> = vec![
        // Spanish
        // https://github.com/KBlixt/subcleaner/blob/master/regex_profiles/default/spanish.conf
        Regex::new(r"\b(creado(s)?|subtitu(lo|los|lado|lada|lados)|subtítu(lo|los|lado|lada|lados)|descarg(ado|ar)|(re-?)?sinc(ed|ro(nizado|nizados|nizacion|nización)?)?|modific(ado|ados|ion|iones|ión|iónes)|traduc(e|ido|idos|tora|cion|ciones|ción|ciónes)|correcc(iones|ion|ión|iónes)|correg(ir|ido|idos)|transcri(bido|pcion|pciones|pción|pciónes)|mejor(ado|amientos)|adaptado|ripeo|arreglos|subs|hecha)\W*(por|de|by)?\W*(:|;)\b").expect("Invalid regex"),
        Regex::new(r"\b(Visiontext|Filmtrans|CARLISHIO|HGWizard|LASERFILM|Fhercho06|Cinesotano|jantoniot|Caichac|cemauli|Drakul|Scarlata|laloonda|japezoa|MarcusL|Kikeguate|KIKEGT|Zagon|KingCreole|Mothernatura|MaLTRaiN|FRH|GCas87|maryluzesp|Marenys|ByAlbis02|ana24horas|Fernando355|Zagonsubs|ikerslot|menoyos|Axel7902|vNaru|livinginthepast|patagonikus|Macias Group|EasyTechOficial|mlmlte|LiarsTeam|OnceUponATEAM)\b").expect("Invalid regex"),
        Regex::new(r"\b(juanchojb|shogun87|Rocio190889|darklin01|R@ul|Mabeas|akallabeth|NicoDipaolo|OsirisTSF|Lord Avestruz|LadyJenny|jeslil7|Giobatta SA|MementMori|la_bestia1962|Natuchia|JJ Porto|marchelo64|c\. oper|SHADOW84\Anfegopi|perroubuntero|Kumara|JosephPools|natycuac|ibvil|SwSub|DarKsh|ShalimarFox|R\[H\]ésus AB\+ Team|Mat Productions|S\. C\. Bananas|Bakugan|M-Rok|YYeTTs|robermgs)\b").expect("Invalid regex"),
        Regex::new(r"\b(admitme|argenteam|finalanime|subtitulamos|substeam|subdivx|tusubtitulo|thesubfactory|Open Subtitles|miembro VIP|osdb\.link|TranslatorsInc|Translators, Inc|TranslatorslncSubs\.blogspot\.com\.es|Southparkspanish|SUBTITULOS\.es|SUBITULOS\.es|SouthParkNews\.net|subtitules\.es|ShooCat|YYeTs|TaMaBin|P@bs|gratispeliculas|SubAdictos|SerieCanal|playships\.eu|tusseries\.com|subswiki\.com|Subs-Team|SUBTÍTULOS\.ES|U\-Sub\.net)\b").expect("Invalid regex"),
        // Global
        // https://github.com/KBlixt/subcleaner/blob/master/regex_profiles/default/global.conf
        Regex::new(r"([^Ã]|^)©|==|>>|<<|★|=-|-=| ::| ::|\^\^").expect("Invalid regex"),
        Regex::new(r"\.(tv|tk|xyz|sex|porn|xxx|link|ru)\b|https?\W").expect("Invalid regex"),
        Regex::new(r"\bs(eason)?\W*\d+[^,]\W*e(pisode)?\W*\d+[^,]").expect("Invalid regex"),
        Regex::new(r"\b(tvsubtitle|YTS|YIFY|opensub(titles)?|sub(scene|rip)|podnapisi|addic7ed|ragbear\W{0,2}com|Point\.360)\b").expect("Invalid regex"),
        Regex::new(r"\b(bozxphd|sazu489|psagmeno|anoxmous|9unshofl|BLACKdoor|titlovi|Danishbits|acorn media|hound\W{0,2}org|hunddawgs|iSubDB)\b").expect("Invalid regex"),
        Regex::new(r"\b(jodix|LESAIGNEUR|HighCode|explosiveskull|GoldenBeard|Fingal61|srjanapala|nadielostzilla|IESAIGNEUR|kdwluverz)\b").expect("Invalid regex"),
        Regex::new(r"\b(FilthyRichFutures|celebritysex|shareuniversity|AmericasCardroom|saveanilluminati|MCH2022|ALLIN1BOX|marocas62)\b").expect("Invalid regex"),
        Regex::new(r"\b(ClearwayLaw|SG-66|ShalimarFox|Icefre[@a]k|WGBH|KBS World|SweSUB|koreansubguy|R\[ésus|Barbie_on_Weed)\b").expect("Invalid regex"),
        Regex::new(r"\b(Aldi Arman|void_spell|LnlyHikikomori|wingyee|McEphie|robster38|dw817|zathras69|Thamyris|Dan4Jem|JustCosmin|moviesnipipay|delsxyz)\b").expect("Invalid regex"),
        Regex::new(r"\b(a\. b\. m\. j\.|Altyazı: Conan|SDI Media Group|HaruHaruSubs|@whyuandri|WahyuAndri|TheHeLL|RiKi66|KingJAIN|ADONI@|Jesslataree)\b").expect("Invalid regex"),
        Regex::new(r"\b(OrionDeBmk|TheChaosLegion|COLDFUSION \& BAARO|riri13|KOCOWA|@.?vii?ki|OnDemandKorea|MBC America|globosapien)\b").expect("Invalid regex"),
        Regex::new(r"\b(MSMOVIESBD|fightingfansubs|DLAznMovies|ancientmexicanwisdom|cookcountysheriff|MovieFull|300mbmovie|KoreanDramax)\b").expect("Invalid regex"),
        Regex::new(r"\b(extremesubs|3gpBluray|prijevodi-online|torrentgalaxy|Dramatorrent|torrent\.com|HQCINEMAS|WANNATALKAB[OA]UTIT|italiansubs|1000fr|1TamilMV|HDFREE)\b").expect("Invalid regex"),
        Regex::new(r"\b(chuanloon90|designer_pc|m_fouda97|Mr.Scudester|Shari_Kenzie|U-Sub.net|TCS Subtitling)\b").expect("Invalid regex"),
        Regex::new(r"\b(rate this subtitle|Subtitle(s)? extracted by|Sync(ed)? (&|and) Clean(ed)?|become VIP member|Subs OCR|the best subtitle(s)?|Timing and Subtitle(s)?|rate this subtitle|Free Online Movie(s)?|Subtitle(s)? Transcribed|Re-Sync \&|English Subtitles|Translation(s)? and adaptation:|Captions by Able|Subtitle Rip|Engsub By|Subtitles brought by|Translation \/ Subtitles)\b").expect("Invalid regex"),
        Regex::new(r"\b(Download MyTotal|itfc subtitles|Built Ford Proud|Captioning sponsored|brought to you by Ford|This is a free sub|Custom subtitle by|For more new Episodes visit|Watch Movies and Series|Advertise your product or brand here|Easy Subtitle(s)? Synchronizer|Watch more movies for free|Brought to you by MrsKorea and mily2|Media Access Group at WGBH|Subtitles brought to you by|UNE SÉRIE ORIGINALE NETFLIX|Brought to you by iRiS|Support us and become a VIP member|Advertise your product or brand here|Caption(s|ing)? made possible by|Visit Our Movie Site|Open Subtitle(s)? MKV Player|Translation(s)? and review by|Spell\-Check and Error\-Correction|Subtitles are brought to you|Translation\. Review by Angel\.|Captions by CSI Australia|Timing and Subs by|Subtitles by The World\Ws Finest Team|Watch and Download free|PLEASE DO NOT UPLOAD ANY OF OUR SUBS|Subtitle by CJ Entertainment)\b").expect("Invalid regex"),
        Regex::new(r"\b(Paramartha|Heavens Subbing Squad|DramaFever|Asian Cinema Encoders|Italian Scrubs Addicted|Kevin \& Tyno)\b").expect("Invalid regex"),
        Regex::new(r"\b(Viki\.com|dramafever\.com|GlowGaze\.Com|seriessub\.com|www\.telegram|d\-addicts\.com|NAPiSY\.info|cinetyp\.ch|lauzabo\.blogspot\.com|Laozhabor\.blogspot\.com|MARIO\.MK|captionmax\.com|firebit\.org|popbitch\.com|swsub\.com|sous-titres\.eu|forom\.\W?com|Csi\-teams\. Fr\. St|GreggBraden\.com|inmymelody\.wordpress\.com|serverpartdeals\.com)").expect("Invalid regex"),
        Regex::new(r"\b(Fansub(s)?|Hardsub(s)?|S u b|Sub Rip:|Terjemahan subtitle oleh)").expect("Invalid regex"),
        // English
        // https://github.com/KBlixt/subcleaner/blob/master/regex_profiles/default/english.conf
        Regex::new(r"\bsub(?:caption(?:s|ed)?|subtitl(?:e|ed|es|ing)|fixed|synch(?:ed|ro(?:nized)?)?|rip(?:ped)?|translat(?:e|ed|ion|ions)|correct(?:ions|ed)|transcri(?:be|bed|pt|ption|ptions)|improve(?:d|ments)|subs|provided|encoded|edit(?:ed|s)?)\W*(?:by|from)?\W*(:|;)\b").expect("Invalid regex"),
        Regex::new(r"^present(s|ing)?:$").expect("Invalid regex"),
        Regex::new(r"\b(KKB|EhLaNa|ydy|swsub|divx|playships|empiremedia|metamorfose|sunmenghao|nessundorma|vothaison)\b").expect("Invalid regex"),
        Regex::new(r"\b(anana|cRosKy|misshu|seriestele|DarKsh|Xenzai|argenteam|tiobetonh|chebinhdan)\b").expect("Invalid regex"),
        Regex::new(r"\b(normita|dawaith|MoSub|snuif|Golgi|Linwelin|Malikay|Ricana|Sadgeezer|SourGrass|mstoll|alire2a)\b").expect("Invalid regex"),
        Regex::new(r"\b(admit1\.app|4kvod\.tv)\b").expect("Invalid regex"),
        // Add more patterns as needed
    ];
    static ref REGEX_TIMESTAMP: Regex = 
        Regex::new(r"(?:\d+).*\s+-->\s+[0-5]{2}:.*$").expect("Invalid regex");
}


fn detect_encoding(data: &[u8]) -> &'static encoding_rs::Encoding {
    let mut detector = EncodingDetector::new();
    detector.feed(data, true);
    detector.guess(None, true)
}

fn trash_match(line: &str) -> bool {
    REGEX_CLEANUP_LIST.iter().any(|regex| regex.is_match(line))
}

fn timestamp_match(line: &str) -> bool {
    REGEX_TIMESTAMP.is_match(line)
}

fn process_subtitle(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut content = Vec::new();
    File::open(input_path)?.read_to_end(&mut content)?;

    let encoding = detect_encoding(&content);
    println!("Detected encoding: {}", encoding.name());
    let (decoded, _, _) = encoding.decode(&content);

    let output_file = File::create(output_path)?;
    let mut output_writer = io::BufWriter::new(output_file);

    let mut block_to_write = Some(String::new());
    let mut block_counter = 1;
    let mut regex_match_detected = false;

    for (i, line) in BufReader::new(decoded.as_bytes()).lines().enumerate() {
        let line = line?;

        if !line.is_empty() && regex_match_detected {
            continue
        };

        if !line.is_empty() && line.chars().all(|c| c.is_numeric()) {
            // needs !line.is_empty, otherwise is_numeric matches an empty line
            continue;
        };

        if line.is_empty() && regex_match_detected {
            regex_match_detected = false;
            continue;
        };

        if line.is_empty() && block_to_write.is_none() {
            continue
        }

        if line.is_empty() {
            writeln!(output_writer, "{}", block_counter)?;
            writeln!(output_writer, "{}", block_to_write.unwrap())?;
            block_counter += 1;
            block_to_write = Some(String::new());
            continue
        }

        if !timestamp_match(&line) && trash_match(&line) {
            regex_match_detected = true;
            // clean block_to_write
            block_to_write = Some(String::new());
            println!("Line {} matches pattern", i + 1);
        } else {
            block_to_write.as_mut().unwrap().push_str(&line);
            block_to_write.as_mut().unwrap().push('\n');
        }
    }

    Ok(())
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_path> <output_path>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    if let Err(err) = process_subtitle(input_path, output_path) {
        eprintln!("Error: {}", err);
    } else {
        println!("Subtitle cleaned successfully!");
    }
}






