use std::collections::HashMap;

pub static AMOGUS_BYTES: &[u8] = include_bytes!("../../assets/misc_images/amogus.png");
pub static AMONGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/amonge.webp");
pub static BALLS_BYTES: &[u8] = include_bytes!("../../assets/misc_images/balls.webp");
pub static BEDGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/bedge.webp");
pub static BRUHMONGUS_BYTES: &[u8] = include_bytes!("../../assets/misc_images/bruhmongus.webp");
pub static CAUGHT_BYTES: &[u8] = include_bytes!("../../assets/misc_images/caught.webp");
pub static CHADGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/chadge.webp");
pub static DEADASS_BYTES: &[u8] = include_bytes!("../../assets/misc_images/deadass.webp");
pub static GENSHIT_BYTES: &[u8] = include_bytes!("../../assets/misc_images/genshit.jpg");
pub static GLORP_BYTES: &[u8] = include_bytes!("../../assets/misc_images/glorp.png");
pub static HANDSUP_BYTES: &[u8] = include_bytes!("../../assets/misc_images/handsup.webp");
pub static HMM_BYTES: &[u8] = include_bytes!("../../assets/misc_images/hmm.webp");
pub static HUH_BYTES: &[u8] = include_bytes!("../../assets/misc_images/huh.webp");
pub static ICANT_BYTES: &[u8] = include_bytes!("../../assets/misc_images/icant.webp");
pub static LIONS_BYTES: &[u8] = include_bytes!("../../assets/misc_images/lions.jpg");
pub static MADGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/madge.webp");
pub static MONKAE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/monkaE.webp");
pub static PEPEGA_BYTES: &[u8] = include_bytes!("../../assets/misc_images/pepega.webp");
pub static PRAYGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/prayge.webp");
pub static QUALITY_BYTES: &[u8] = include_bytes!("../../assets/misc_images/quality.jpg");
pub static SADGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/sadge.webp");
pub static TA_BYTES: &[u8] = include_bytes!("../../assets/misc_images/ta.webp");
pub static TA7_BYTES: &[u8] = include_bytes!("../../assets/misc_images/ta7.webp");
pub static TAO_BYTES: &[u8] = include_bytes!("../../assets/misc_images/taO.webp");
pub static TORTURE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/torture.webp");
pub static TUH_BYTES: &[u8] = include_bytes!("../../assets/misc_images/tuh.webp");
pub static TUH7_BYTES: &[u8] = include_bytes!("../../assets/misc_images/tuh7.webp");
pub static WIDEPEEPOHAPPY_BYTES: &[u8] =
    include_bytes!("../../assets/misc_images/widepeepoHappy.webp");
pub static WOAH_BYTES: &[u8] = include_bytes!("../../assets/misc_images/woah.webp");
pub static WOKEGE_BYTES: &[u8] = include_bytes!("../../assets/misc_images/wokege.webp");
pub static YEP_BYTES: &[u8] = include_bytes!("../../assets/misc_images/yep.webp");
pub static ZYONIX_BYTES: &[u8] = include_bytes!("../../assets/misc_images/zyonix.jpg");

pub fn misc_image_lu() -> HashMap<&'static str, (&'static str, &'static [u8])> {
    HashMap::from([
        ("amogus", ("amogus.png", AMOGUS_BYTES)),
        ("amonge", ("amonge.webp", AMONGE_BYTES)),
        ("balls", ("balls.webp", BALLS_BYTES)),
        ("bedge", ("bedge.webp", BEDGE_BYTES)),
        ("bruhmongus", ("bruhmongus.webp", BRUHMONGUS_BYTES)),
        ("caught", ("caught.webp", CAUGHT_BYTES)),
        ("chadge", ("chadge.webp", CHADGE_BYTES)),
        ("deadass", ("deadass.webp", DEADASS_BYTES)),
        ("genshit", ("genshit.jpg", GENSHIT_BYTES)),
        ("glorp", ("glorp.png", GLORP_BYTES)),
        ("handsup", ("handsup.webp", HANDSUP_BYTES)),
        ("hmm", ("hmm.webp", HMM_BYTES)),
        ("huh", ("huh.webp", HUH_BYTES)),
        ("icant", ("icant.webp", ICANT_BYTES)),
        ("lions", ("lions.jpg", LIONS_BYTES)),
        ("madge", ("madge.webp", MADGE_BYTES)),
        ("monkaE", ("monkaE.webp", MONKAE_BYTES)),
        ("pepega", ("pepega.webp", PEPEGA_BYTES)),
        ("prayge", ("prayge.webp", PRAYGE_BYTES)),
        ("quality", ("quality.jpg", QUALITY_BYTES)),
        ("sadge", ("sadge.webp", SADGE_BYTES)),
        ("ta", ("ta.webp", TA_BYTES)),
        ("ta7", ("ta7.webp", TA7_BYTES)),
        ("taO", ("taO.webp", TAO_BYTES)),
        ("torture", ("torture.webp", TORTURE_BYTES)),
        ("tuh", ("tuh.webp", TUH_BYTES)),
        ("tuh7", ("tuh7.webp", TUH7_BYTES)),
        (
            "widepeepoHappy",
            ("widepeepoHappy.webp", WIDEPEEPOHAPPY_BYTES),
        ),
        ("woah", ("woah.webp", WOAH_BYTES)),
        ("wokege", ("wokege.webp", WOKEGE_BYTES)),
        ("yep", ("yep.webp", YEP_BYTES)),
        ("zyonix", ("zyonix.jpg", ZYONIX_BYTES)),
    ])
}

pub fn gif_lu() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "are you sure",
            "https://tenor.com/view/omni-man-omni-man-are-you-sure-are-you-sure-invincible-gif-3935116808772397515",
        ),
        (
            "boom",
            "https://tenor.com/view/rizzler-boom-meter-booooooom-costco-gif-2595128906441418079",
        ),
        (
            "bust",
            "https://tenor.com/view/aventurine-star-rail-i-love-gambling-roulette-gif-15600735931808948001",
        ),
        (
            "holy moly",
            "https://tenor.com/view/holy-moly-gif-2379897567703251318",
        ),
        (
            "milano",
            "https://tenor.com/view/mualani-mualani-genshin-mualani-leaks-genshin-genshin-impact-gif-15676509744061870577",
        ),
        ("nopers", "https://tenor.com/view/nope-twitch-gif-18862714"),
        (
            "oi",
            "https://tenor.com/view/billy-butcher-tea-gif-23858792",
        ),
        (
            "skeleton",
            "https://tenor.com/view/skeleton-spin-fan-rope-gif-25248314",
        ),
        (
            "wsl",
            "https://tenor.com/view/robin-robin-hsr-robin-star-rail-robin-honkai-star-rail-robin-disappointed-gif-6780156215518762303",
        ),
    ])
}

pub static GEPARD: &str = r#"I HATE Gepard. He is so useless, I don't understand why anyone would like him by choice. All he is is an attractive, sweet, caring, sexy, sweaty muscular hot jock-type that reminds me all too much of the feelings I used to get from watching the football team in high school. How could anyone appreciate him when all he does is bring a big shield? I was able to clear Simulated Universe World 1 with NO shielder, and people tell me that I need a defensive unit? Yes I died to World 1 over 20 times, but I wasn't trying to be fair. Why would I need a shielder if my team of Sampo/Hook/Dan Heng/Tingyun is clearly one of the BEST teams in the game? I don't have skill issue and I don't like hot men. I especially don't like hot sexy men like Gepard, who want to do nothing but protect me. I don't need protection and I certainly DON'T need a hot sexy man like Gepard on my roster. What do people even see in him? A kind, compassionate, and caring individual who looks after his family and city? How could anyone actually be ok with getting Gepard off of their Departure Warp? It's simply logical that NO ONE should appreciate Gepard. It's such a ludicrous idea that anyone would want Gepard's muscular arms wrapped around them while they cuddle in bed at night after a wonderful night out in Belobog. What a silly concept"#;

pub static HATER: &str = r#"I hope both sides of your pillow are warm tonight. May the chocolate chips in your cookies always turn out to be raisins. May every sock you wear be slightly rotated, just enough for it to be uncomfortable. May your mother come to talk to you, and then leave your door slightly ajar, so that you have to get up and close it. May you forever feel your cellphone vibrating in the pocket it's not even in. May your spoon always slip and sink into the hot soup you eat. May both sides of your pillow be warm. May every step you take be on Lego. May your phone always be at 1% when you need it most. May all your favorite shows have cliffhanger endings that never get resolved. May your shoelaces forever come untied. May you always have a song stuck in your head that you can't remember the lyrics to. May all your pens run out of ink mid-sentence. May every pair of socks you own have a hole in the toe. May your headphones get tangled every time you pull them out of your pocket. May your favorite snack always be slightly stale. May you always forget why you walked into a room. May your Wi-Fi always be just a little too slow for streaming. May you always hit a red light when you're running late. May your coffee always be too hot or too cold. May your favorite shirt always have a stain you can't get out. May your autocorrect always change your words to something embarrassing. May your batteries always be just short of full charge. May your ice cream always have freezer burn. May you always step in something sticky right after cleaning your shoes. May every piece of toast you make be slightly burnt. May your keys always be in the last place you look. May all your books have the last page missing. May you always get the noisy cart at the grocery store. May every zipper you use get stuck halfway up. May your favorite song always be skipped on shuffle"#;

pub static PENACONY: &str = r#"One day, after dinner, while my younger sister and I were lounging about in Mr. Gopher Wood's yard, we spotted a fledgling Charmony Dove all on its own. That baby bird was tiny, it didn't even have all of its feathers, and it couldn't sing. When we found it, it was already on its last breath, having fallen into a shrub — probably abandoned by its parents. We decided to build a nest for it right there and then. However, thinking back, that winter was unusually cold, with fierce winds at night in the yard, not to mention the many poisonous bugs and wild beasts in the vicinity... It was clear that if we left the fledgling in the yard, it stood no chance of surviving until spring. So, I suggested we take it inside, place it on the shelf by the window, and asked the adults to fashion a cage for it. We decided that when it regained its strength enough to spread its wings, we would release it back into the wild. The tragic part — something that we'd never considered — was that this bird's fate had already been determined long before this moment... Its destiny was determined by our momentary whim. Now, I pass the power of choice to you all. Faced with this situation, what choice would you make? Stick to the original plan, and build a nest with soft net where the Charmony Dove fell? Or build a cage for it, and feed it, giving it the utmost care from within the warmth of a home? I eagerly await your answer"#;

pub static ROBIN: &str = r#":musical_note: RISE UP INTO MY WOOOOOOOOOOORLD :musical_note:
Mend your pace, sway to the beat
Hands up, embrace who you wanna be
We're reachin' heights unseen, ooh-whoa, whoa-oh-ho
Feel the fire deep within
You're the key to where my trust begins
Join my dream, it's just the right time, whoa, whoa-ho
Leave it all behind (all behind), get ready now
Renew your definition
World so high, let me show
And hear my declaration
No more ties
Off the ground and tap your feet
Look, stars are near when you feel the beat
We're bound for greater height, ooh-whoa, whoa-oh-ho
Take a leap into the blaze
Don't lose yourself in mundanity
Join my dream, it is the right time, whoa, whoa-ho
Leave it all behind, get ready now (sing along)
:musical_note: WELCOME TO MY WOOOOOOOOOOORLD :musical_note:
Renew your definition
World so high, let me show
And hear my declaration
No more ties
(Whoa, whoa)
Leave it all behind (whoa, whoa)
Welcome to my world
Renew your definition
World so high, let me show
Now hear my declaration
:musical_note: WELCOME TO MY WOOOOOOOOOOORLD :musical_note:
Renew your definition
World so high, we will show
And go beyond horizon
Side by side"#;

pub static SIGMA: &str = r#"I, as a sigma, can assure you that no matter how much you stick your gyatt out for the rizzler while hitting the griddy like quandale dingle, evade fanum tax when Kai Cenat demands it, or resist the sussy grimace shake to not break your 69 day mewing streak, you are NOT the most skibidi dop dop yes yes imposter in skibidi Ohio. -69,420 aura for you, you Ishowspeed wannabe NPC"#;

pub static XIANGLING: &str = r#"I can't take it anymore. I'm sick of xiangling.
I try to play diluc. My xiangling deals more damage.
I try to play yoimiya. My xiangling deals more damage.
I try to play Hu tao. My xiangling deals more damage.
I want to play Klee. Her best team has xiangling.
I want to play raiden, childe. They both want xiangling.

She grabs me by the throat. I fish for her. I cook for her.I give her the catch. She isn't satisfied. I pull engulfing lightning.
"I don't need this much er" She tells me. "Give me more field time."
She grabs bennett and forces him to throw himself off enemies.
"You just need to funnel me more. I can deal more damage with homa."

I can't pull for homa, I don't have enough primogems.
She grabs my credit card. It declines. "Guess this is the end."
She grabs guoba. She says "Gouba, get them."
There is no hint of sadness in his eyes.
Nothing but pure, no icd pyro application. What a cruel world."#;

pub static COPYPASTA_KEYS: &[&str] =
    &["gepard", "hater", "penacony", "robin", "sigma", "xiangling"];

pub fn copypasta_lu() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("gepard", GEPARD),
        ("hater", HATER),
        ("penacony", PENACONY),
        ("robin", ROBIN),
        ("sigma", SIGMA),
        ("xiangling", XIANGLING),
        ("guoba", XIANGLING),
    ])
}

pub static NAUGHTY_KEYS: &[&str] = &["come", "hard", "long", "shaft", "finish", "coming", "came"];

pub fn find<'a>(needles: &[&'a str], haystack: &str) -> Option<&'a str> {
    needles.iter().cloned().find(|&key| haystack.contains(key))
}
