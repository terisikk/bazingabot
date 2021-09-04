use serde::Deserialize;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{env, error::Error, fs, path::Path, path::MAIN_SEPARATOR};
use tracing::{debug, error};

static USERLIST_FILE_NAME: &str = "lolcall_users";

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

// TODO extract to file if the set is extended
const LOLCALL: &'static [&'static str] = &[
    "NONNIIN poijat, NYT MENNÄÄN LOLLIA :D",
    "LOL TIEM LOL TIEM LOL TIEM",
    "Destination Kurwatunturi, summoners rift",
    "Midillä metkut mielessä, tulkaas katsomaan ;--)",
    "Oisipa zaih lyönyt gravesia, vielä ei ole liian myöhäistä!!!",
    "LOL LOL LOL LOL LOL LOL",
    "Jaahas, kello näyttää lollia :D",
    "Ei itkeä saa :D Ei meluta saa :D Joku voi tulla :D LOLLIA",
    "NYYYYT MEEEENNÄÄÄÄÄÄN SAATANA",
    "JA MET LÄHIMÄ :D",
    "Aijjai, sehän on nöyryytyksen aika :D",
    "Juuh, elikkäs, lollia?",
    ";OISPA lollia",
    "eikö mentäis yhdessä invadeemaan?",
    "TRUBADUURI TORVEA SOITTAA; LOLLIAIKA KOITTAA; :D",
    "tulukaahan pelaamaan",
    "Pellaamaan nyt",
    "Lollia?",
    "Vitu AWESOME pelata lollia ja kuunnella motörheadii",
    "Janiskeisarin muistolle: NYT PELATAAN",
    "S-senpai.. lol time..",
    "enemmän lollia, vähemmän vastalauseita",
    "lol :D",
    "looolliiiaaaa",
    ";SAISPA lollia",
    "gooby plz, lollia?",
    "WELCOME to SUMMONERS rift",
    "pliiiiiiiiiiiiiiiiiiiiiiiiiis jooko",
    "Vain ne jotka rakastavat, näkevät lollin siellä, missä muut näkevät köllin.",
    "Tulukee, käyään vähään pelaamassa :3",
    "MÄ MUISTAN SEN, KUIN EILISEN, JANISKEISARIN PENTA KILLIN",
    "Pelataan lollia pelataan lollia pelataan lollia",
    "LOLLIA SAATANA",
    "LOL :D",
    "lolcall lolcall lolcall tissit",
];

// TODO: pair with channel in case same bot instance
// is used in multiple channels
#[derive(Debug, Deserialize)]
struct User {
    id: String,
}

#[command]
pub async fn lolcall(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("lolcall");
    let command = _parse_command(&mut args);
    if command == None {
        _send_lolcall(ctx, msg).await;
        return Ok(());
    }
    let command_str = command.unwrap();
    match &command_str as &str {
        "help" => {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "- !lolcall add - register to receive lolcall mentions
- !lolcall remove - deregister and disable lolcall mentions
- !lolcall to mention all registered users and play some lol"
                    ),
                )
                .await?;

            return Ok(());
        }
        "add" => {
            let add_ret = _add_user(&msg.author.id);
            if add_ret {
                msg.reply(&ctx.http, format!("Done.")).await?;
            }
            return Ok(());
        }
        "remove" => {
            let remove_ret = _remove_user(&msg.author.id);
            if remove_ret {
                msg.reply(&ctx.http, format!("Done.")).await?;
            }
            return Ok(());
        }
        _ => {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Unrecognized argument, see !lolcall help"),
                )
                .await?;

            return Ok(());
        }
    }

    // TODO remember to write file header when writing new file!
}

fn _get_user_list_file_path_str() -> String {
    let runtime_path = env::var("RUNTIME_PATH").unwrap_or(String::from("."));
    return runtime_path + &MAIN_SEPARATOR.to_string() + USERLIST_FILE_NAME;
}

fn _init_user_list_file(path: &Path) {
    let header = String::from("id\n");
    fs::write(path, header).expect("Unable to write lolcall user file");
}

fn _is_id_in_user_list(id: &UserId, path: &Path) -> bool {
    let user_list_str = fs::read_to_string(path).expect("Unable to read lolcall user file");
    let id_str = format!("{}", id);
    let find_res = user_list_str.find(&id_str);
    return !find_res.is_none();
}

fn _add_id_to_user_list(id: &UserId, path: &Path) -> bool {
    let id_str = format!("{}", id);
    match fs::OpenOptions::new().write(true).append(true).open(path) {
        Ok(file) => {
            let mut writer = csv::Writer::from_writer(file);
            let res = writer.write_record(&[&id_str]);
            if res.is_err() {
                error!("Unable to write to lolcall user file");
                return false;
            }
        }
        Err(e) => {
            error!("Unable to open lolcall user file for writing, error: {}", e);
            return false;
        }
    }
    return true;
}

fn _remove_id_from_user_list(id: &UserId, path: &Path) -> bool {
    let user_list_str = fs::read_to_string(path).expect("Unable to read lolcall user file");
    let id_str = format!("{}{}", id, LINE_ENDING);
    let modified_user_list_str = user_list_str.replace(&id_str, "");
    if modified_user_list_str == user_list_str {
        // No change
        return false;
    }
    fs::write(path, modified_user_list_str).expect("Unable to rewrite lolcall user file");
    return true;
}

fn _add_user(id: &UserId) -> bool {
    let file_path_str = _get_user_list_file_path_str();
    let file_path = Path::new(&file_path_str);
    let file_path_existed = file_path.exists();
    if !file_path_existed {
        _init_user_list_file(file_path);
    }
    if _is_id_in_user_list(id, file_path) {
        return false;
    }
    let add_ok = _add_id_to_user_list(id, file_path);
    return add_ok;
}

fn _remove_user(id: &UserId) -> bool {
    let file_path_str = _get_user_list_file_path_str();
    let file_path = Path::new(&file_path_str);
    if !file_path.exists() {
        return false;
    }
    let remove_ret = _remove_id_from_user_list(id, file_path);
    return remove_ret;
}

// TODO channel support? Now only contains names regardless of the channel structure
fn _read_user_file(path: &Path) -> Result<Vec<User>, Box<dyn Error>> {
    let mut vec: Vec<User> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let user: User = result?;
        println!("{:?}", user.id);
        vec.push(user);
    }
    return Ok(vec);
}

fn _get_user_list(users: &Vec<User>) -> String {
    let mut res = String::from("");
    for user in users {
        res.push_str("<@");
        res.push_str(&user.id);
        res.push_str(">, ");
    }
    return res[0..res.len() - 2].to_string();
}

async fn _send_lolcall(ctx: &Context, msg: &Message) {
    let file_path_str = _get_user_list_file_path_str();
    let file_path = Path::new(&file_path_str);
    let mut user_list = String::from("");
    match _read_user_file(file_path) {
        Ok(users) => {
            user_list = _get_user_list(&users);
        }
        Err(e) => {
            // Not necessarily an error, if nobody has been added yet (fresh init)
            debug!("lolcall: could not read lolcall users, error: {} ", e);
        }
    }

    let i: u32 = rand::random();

    match msg
        .channel_id
        .say(
            &ctx.http,
            format!("{} - {}", user_list, LOLCALL[i as usize % LOLCALL.len()]),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => error!("lolcall: could not send message to channel, error: {}", e),
    }
}

fn _parse_command(args: &mut Args) -> Option<String> {
    return args.single::<String>().ok();
}

//#[command]
//#[owners_only]
//async fn lolcall_admin(ctx: &Context, msg: &Message) -> CommandResult {
// .. add/remove nick
