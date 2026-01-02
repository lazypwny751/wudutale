pub struct TextResources {
    pub combat_actions: CombatActionTexts,
    pub gaster_dialogues: Vec<String>,
    pub game_over_messages: GameOverMessages,
    pub ui: UiTexts,
}

pub struct CombatActionTexts {
    pub munafik: Vec<String>,
    pub kafir: Vec<String>,
    pub zindik: Vec<String>,
    pub tagut: Vec<String>,
    pub deccal: Vec<String>,
    pub ebu_cehil: Vec<String>,
    pub yecuc: Vec<String>,
}

pub struct GameOverMessages {
    pub teblig_high: Vec<String>,
    pub tekfir_high: Vec<String>,
    pub equal: Vec<String>,
}

pub struct UiTexts {
    pub game_over_title: String,
    pub return_menu: String,
    pub quit_game: String,
    pub user_label: String,
}

impl TextResources {
    pub fn new_turkish() -> Self {
        Self {
            combat_actions: CombatActionTexts {
                munafik: vec![
                    "Ona Münafık dedin.\nOmuz silkti.".to_string(),
                    "Ona Münafık dedin.\n'Kanıtın var mı?' dedi.".to_string(),
                    "Ona Münafık dedin.\nGülüp geçti.".to_string(),
                ],
                kafir: vec![
                    "Ona Kafir dedin.\nSırıttı.".to_string(),
                    "Ona Kafir dedin.\n'Bunu iltifat sayarım' dedi.".to_string(),
                    "Ona Kafir dedin.\nSeni ciddiye almadı.".to_string(),
                ],
                zindik: vec![
                    "Ona Zındık dedin.\nKahkaha attı.".to_string(),
                    "Ona Zındık dedin.\n'Eski moda bir hakaret' dedi.".to_string(),
                    "Ona Zındık dedin.\nSadece başını salladı.".to_string(),
                ],
                tagut: vec![
                    "Ona Tağut dedin.\nGöz kırptı.".to_string(),
                    "Ona Tağut dedin.\n'Gücümü kabul ediyorsun' dedi.".to_string(),
                    "Ona Tağut dedin.\nSana tepeden baktı.".to_string(),
                ],
                deccal: vec![
                    "Ona Deccal dedin.\n'Tek gözüm bile yeter' dedi.".to_string(),
                    "Ona Deccal dedin.\nAlnını gösterdi.".to_string(),
                    "Ona Deccal dedin.\n'Daha zamanı gelmedi' dedi.".to_string(),
                ],
                ebu_cehil: vec![
                    "Ona Ebu Cehil dedin.\n'Cehalet mutluluktur' dedi.".to_string(),
                    "Ona Ebu Cehil dedin.\nKarpuz fırlattı.".to_string(),
                    "Ona Ebu Cehil dedin.\n'Bedir'de görüşürüz' dedi.".to_string(),
                ],
                yecuc: vec![
                    "Ona Yecüc dedin.\n'Mecüc nerede?' diye sordu.".to_string(),
                    "Ona Yecüc dedin.\nDuvarı kemirmeye başladı.".to_string(),
                    "Ona Yecüc dedin.\nSürü halinde saldırdı.".to_string(),
                ],
            },
            gaster_dialogues: vec![
                "çakar çakmaz çakan çakmak...".to_string(),
                "bir berber bir berbere...".to_string(),
                "şu köşe yaz köşesi...".to_string(),
            ],
            game_over_messages: GameOverMessages {
                teblig_high: vec![
                    "Tebliğin yetersiz kaldı...".to_string(),
                    "Daha fazla anlatmalıydın.".to_string(),
                    "Sözlerin kalplere ulaşmadı.".to_string(),
                    "Hidayet Allah'tandır ama sen de çabalamadın.".to_string(),
                ],
                tekfir_high: vec![
                    "Herkesi tekfir ettin, yalnız kaldın.".to_string(),
                    "Aşırılık seni bitirdi.".to_string(),
                    "Tekfir kılıcı seni de kesti.".to_string(),
                    "Hariciler bile senden korkardı.".to_string(),
                ],
                equal: vec![
                    "Nice try, maybe next time.".to_string(),
                    "Klavye kullanmayı yeni mi öğrendin?".to_string(),
                    "Ne emmeye ne gömmeye...".to_string(),
                    "Ortada kaldın, arafta yandın.".to_string(),
                    "Skill issue.".to_string(),
                ],
            },
            ui: UiTexts {
                game_over_title: "OYUN BİTTİ".to_string(),
                return_menu: "Menüye Dön".to_string(),
                quit_game: "Oyundan Çık".to_string(),
                user_label: "Kullanıcı: ".to_string(),
            },
        }
    }

    #[allow(dead_code)]
    pub fn new_english() -> Self {
        Self {
            combat_actions: CombatActionTexts {
                munafik: vec![
                    "You called him Hypocrite.\nHe shrugged.".to_string(),
                    "You called him Hypocrite.\n'Got proof?' he asked.".to_string(),
                    "You called him Hypocrite.\nHe laughed it off.".to_string(),
                ],
                kafir: vec![
                    "You called him Infidel.\nHe grinned.".to_string(),
                    "You called him Infidel.\n'I take that as a compliment' he said.".to_string(),
                    "You called him Infidel.\nHe didn't take you seriously.".to_string(),
                ],
                zindik: vec![
                    "You called him Heretic.\nHe laughed out loud.".to_string(),
                    "You called him Heretic.\n'Old fashioned insult' he said.".to_string(),
                    "You called him Heretic.\nHe just nodded.".to_string(),
                ],
                tagut: vec![
                    "You called him Tyrant.\nHe winked.".to_string(),
                    "You called him Tyrant.\n'You acknowledge my power' he said.".to_string(),
                    "You called him Tyrant.\nHe looked down on you.".to_string(),
                ],
                deccal: vec![
                    "You called him Antichrist.\n'One eye is enough' he said.".to_string(),
                    "You called him Antichrist.\nHe pointed to his forehead.".to_string(),
                    "You called him Antichrist.\n'Not time yet' he said.".to_string(),
                ],
                ebu_cehil: vec![
                    "You called him Abu Jahl.\n'Ignorance is bliss' he said.".to_string(),
                    "You called him Abu Jahl.\nHe threw a watermelon.".to_string(),
                    "You called him Abu Jahl.\n'See you at Badr' he said.".to_string(),
                ],
                yecuc: vec![
                    "You called him Gog.\n'Where is Magog?' he asked.".to_string(),
                    "You called him Gog.\nHe started gnawing the wall.".to_string(),
                    "You called him Gog.\nHe attacked in a swarm.".to_string(),
                ],
            },
            gaster_dialogues: vec![
                "darker darker yet darker...".to_string(),
                "photon readings negative...".to_string(),
                "this next experiment...".to_string(),
            ],
            game_over_messages: GameOverMessages {
                teblig_high: vec![
                    "Your preaching was insufficient...".to_string(),
                    "You should have explained more.".to_string(),
                    "Your words did not reach hearts.".to_string(),
                    "Guidance is from God, but you didn't try enough.".to_string(),
                ],
                tekfir_high: vec![
                    "You excommunicated everyone, now you are alone.".to_string(),
                    "Extremism finished you.".to_string(),
                    "The sword of Takfir cut you too.".to_string(),
                    "Even Kharijites would fear you.".to_string(),
                ],
                equal: vec![
                    "Nice try, maybe next time.".to_string(),
                    "Did you just learn to use a keyboard?".to_string(),
                    "Neither here nor there...".to_string(),
                    "Stuck in the middle, burned in limbo.".to_string(),
                    "Skill issue.".to_string(),
                ],
            },
            ui: UiTexts {
                game_over_title: "GAME OVER".to_string(),
                return_menu: "Return to Menu".to_string(),
                quit_game: "Quit Game".to_string(),
                user_label: "User: ".to_string(),
            },
        }
    }
}
