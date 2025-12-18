import os
from pathlib import Path
from PIL import Image
import subprocess

OUTPUT_FILE = 'src/summon/pool.rs'
BASE_DIR = 'assets/summon_images'
RESOLUTION = (256, 256)


def png_to_webp(folder_path=BASE_DIR):
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.png'):
                png_path = os.path.join(root, file)
                webp_path = os.path.splitext(png_path)[0] + '.webp'

                if os.path.isfile(webp_path):
                    print(f"Skipping {png_path} (already converted)")
                    continue

                try:
                    with Image.open(png_path) as img:
                        img.save(webp_path, 'WEBP')
                        print(f"Converted: {png_path} → {webp_path}")
                except Exception as e:
                    print(f"Failed to convert {png_path}: {e}")


def convert_images_to_resolution(input_folder=BASE_DIR, resolution=RESOLUTION):
    for root, _, files in os.walk(input_folder):
        for file in files:
            if file.lower().endswith('.webp'):
                path = os.path.join(root, file)
                try:
                    with Image.open(path) as img:
                        resized = img.resize(resolution, Image.LANCZOS)
                        resized.save(path)
                        print(f"Resized: {path}")
                except Exception as e:
                    print(f"Failed to resize {path}: {e}")


def delete_png_files(folder_path=BASE_DIR):
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.png'):
                path = os.path.join(root, file)
                try:
                    os.remove(path)
                    print(f"Deleted: {path}")
                except Exception as e:
                    print(f"Failed to delete {path}: {e}")


def generate_pool(folder_path=BASE_DIR, output_file=OUTPUT_FILE):
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    rust_lines = [
        "// Auto-generated summon image pool\n",
        "use std::collections::HashMap;\n"
    ]

    static_defs = []
    lu_entries = []
    groups = {}

    for root, _, files in os.walk(folder_path):
        if root == folder_path:
            continue

        folder_name = os.path.basename(root)
        var_name = folder_name.upper().replace('-', '_')
        groups[var_name] = []

        for file in sorted(files):
            if not file.endswith('.webp'):
                continue

            key = os.path.splitext(file)[0]
            relative_path = os.path.join(root, file).replace("\\", "/")
            const_name = f"{var_name}_{key}".upper()

            static_defs.append(f'pub static {const_name}: &[u8] = include_bytes!("../../{relative_path}");')
            lu_entries.append(f'        ("{key}", ("{file}", {const_name})),')
            groups[var_name].append(f'"{key}"')

    rust_lines.extend(static_defs)
    rust_lines.append("\n")

    rust_lines.append("pub fn summon_image_lu() -> HashMap<&'static str, (&'static str, &'static [u8])> {\n")
    rust_lines.append("    HashMap::from([")
    rust_lines.extend(lu_entries)
    rust_lines.append("    ])\n}\n")

    for group_var, keys in groups.items():
        rust_lines.append(f"pub static {group_var}: &[&str] = &[")
        for key in keys:
            rust_lines.append(f"    {key},")
        rust_lines.append("];\n")

    with open(output_file, 'w') as f:
        f.write("\n".join(rust_lines))

    print(f"Generated Rust file: {output_file}")


def refresh_pool():
    png_to_webp()
    delete_png_files()
    convert_images_to_resolution()
    generate_pool()



gifs = [
    ("wsl", "https://tenor.com/view/robin-robin-hsr-robin-star-rail-robin-honkai-star-rail-robin-disappointed-gif-6780156215518762303"),
    ("nopers", "https://tenor.com/view/nope-twitch-gif-18862714"),
    ("oi", "https://tenor.com/view/billy-butcher-tea-gif-23858792"),
    ("holy moly", "https://tenor.com/view/holy-moly-gif-2379897567703251318"),
    ("boom", "https://tenor.com/view/rizzler-boom-meter-booooooom-costco-gif-2595128906441418079"),
    ("milano", "https://tenor.com/view/mualani-mualani-genshin-mualani-leaks-genshin-genshin-impact-gif-15676509744061870577"),
    ("bust", "https://tenor.com/view/aventurine-star-rail-i-love-gambling-roulette-gif-15600735931808948001"),
    ("skeleton", "https://tenor.com/view/skeleton-spin-fan-rope-gif-25248314"),
    ("are you sure", "https://tenor.com/view/omni-man-omni-man-are-you-sure-are-you-sure-invincible-gif-3935116808772397515"),
]

copypastas = [
    ("XIANGLING", "xiangling", '''I can't take it anymore. I'm sick of xiangling.
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
Nothing but pure, no icd pyro application. What a cruel world.'''),

    ("ROBIN", "robin", ''':musical_note: RISE UP INTO MY WOOOOOOOOOOORLD :musical_note:
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
Side by side'''),

    ("SIGMA", "sigma", "I, as a sigma, can assure you that no matter how much you stick your gyatt out for the rizzler while hitting the griddy like quandale dingle, evade fanum tax when Kai Cenat demands it, or resist the sussy grimace shake to not break your 69 day mewing streak, you are NOT the most skibidi dop dop yes yes imposter in skibidi Ohio. -69,420 aura for you, you Ishowspeed wannabe NPC"),
    ("PENACONY", "penacony", "One day, after dinner, while my younger sister and I were lounging about in Mr. Gopher Wood's yard, we spotted a fledgling Charmony Dove all on its own. That baby bird was tiny, it didn't even have all of its feathers, and it couldn't sing. When we found it, it was already on its last breath, having fallen into a shrub — probably abandoned by its parents. We decided to build a nest for it right there and then. However, thinking back, that winter was unusually cold, with fierce winds at night in the yard, not to mention the many poisonous bugs and wild beasts in the vicinity... It was clear that if we left the fledgling in the yard, it stood no chance of surviving until spring. So, I suggested we take it inside, place it on the shelf by the window, and asked the adults to fashion a cage for it. We decided that when it regained its strength enough to spread its wings, we would release it back into the wild. The tragic part — something that we'd never considered — was that this bird's fate had already been determined long before this moment... Its destiny was determined by our momentary whim. Now, I pass the power of choice to you all. Faced with this situation, what choice would you make? Stick to the original plan, and build a nest with soft net where the Charmony Dove fell? Or build a cage for it, and feed it, giving it the utmost care from within the warmth of a home? I eagerly await your answer"),
    ("GEPARD", "gepard", "I HATE Gepard. He is so useless, I don't understand why anyone would like him by choice. All he is is an attractive, sweet, caring, sexy, sweaty muscular hot jock-type that reminds me all too much of the feelings I used to get from watching the football team in high school. How could anyone appreciate him when all he does is bring a big shield? I was able to clear Simulated Universe World 1 with NO shielder, and people tell me that I need a defensive unit? Yes I died to World 1 over 20 times, but I wasn't trying to be fair. Why would I need a shielder if my team of Sampo/Hook/Dan Heng/Tingyun is clearly one of the BEST teams in the game? I don't have skill issue and I don't like hot men. I especially don't like hot sexy men like Gepard, who want to do nothing but protect me. I don't need protection and I certainly DON'T need a hot sexy man like Gepard on my roster. What do people even see in him? A kind, compassionate, and caring individual who looks after his family and city? How could anyone actually be ok with getting Gepard off of their Departure Warp? It's simply logical that NO ONE should appreciate Gepard. It's such a ludicrous idea that anyone would want Gepard's muscular arms wrapped around them while they cuddle in bed at night after a wonderful night out in Belobog. What a silly concept"),
    ("HATER", "hater", "I hope both sides of your pillow are warm tonight. May the chocolate chips in your cookies always turn out to be raisins. May every sock you wear be slightly rotated, just enough for it to be uncomfortable. May your mother come to talk to you, and then leave your door slightly ajar, so that you have to get up and close it. May you forever feel your cellphone vibrating in the pocket it's not even in. May your spoon always slip and sink into the hot soup you eat. May both sides of your pillow be warm. May every step you take be on Lego. May your phone always be at 1% when you need it most. May all your favorite shows have cliffhanger endings that never get resolved. May your shoelaces forever come untied. May you always have a song stuck in your head that you can't remember the lyrics to. May all your pens run out of ink mid-sentence. May every pair of socks you own have a hole in the toe. May your headphones get tangled every time you pull them out of your pocket. May your favorite snack always be slightly stale. May you always forget why you walked into a room. May your Wi-Fi always be just a little too slow for streaming. May you always hit a red light when you're running late. May your coffee always be too hot or too cold. May your favorite shirt always have a stain you can't get out. May your autocorrect always change your words to something embarrassing. May your batteries always be just short of full charge. May your ice cream always have freezer burn. May you always step in something sticky right after cleaning your shoes. May every piece of toast you make be slightly burnt. May your keys always be in the last place you look. May all your books have the last page missing. May you always get the noisy cart at the grocery store. May every zipper you use get stuck halfway up. May your favorite song always be skipped on shuffle")
]

def generate_lus(directory: str):
    dir_path = Path(directory).resolve()
    cwd = Path.cwd()

    if not dir_path.is_dir():
        raise ValueError(f"{directory} is not a valid directory")

    entries = []
    for file in dir_path.iterdir():
        if file.is_file():
            stem = file.stem
            filename = file.name
            abs_path = file.resolve()
            rel_path = abs_path.relative_to(cwd)
            entries.append((stem, filename, str(rel_path).replace("\\", "/")))

    with open("src/utils/lus.rs", 'w', encoding='utf-8') as f:
        f.write("use std::collections::HashMap;\n\n")

        for stem, filename, rel_path in sorted(entries):
            var_name = f"{stem.upper()}_BYTES"
            f.write(f'pub static {var_name}: &[u8] = include_bytes!("../../{rel_path}");\n')

        f.write("\n")

        f.write("pub fn misc_image_lu() -> HashMap<&'static str, (&'static str, &'static [u8])> {\n")
        f.write("    HashMap::from([\n")
        for stem, filename, _ in sorted(entries):
            var_name = f"{stem.upper()}_BYTES"
            f.write(f'        ("{stem}", ("{filename}", {var_name})),\n')
        f.write("    ])\n")
        f.write("}\n")

        f.write("\n")

        f.write("pub fn gif_lu() -> HashMap<&'static str, &'static str> {\n")
        f.write("    HashMap::from([\n")
        for key, link in sorted(gifs):
            f.write(f'        ("{key}", "{link}"),\n')
        f.write("    ])\n")
        f.write("}\n")

        f.write("\n")

        for var_name, _, text in sorted(copypastas):
            f.write(f'pub static {var_name}: &str = r#"{text}"#;\n\n')

        f.write('pub static COPYPASTA_KEYS: &[&str] = &[\n')
        for _, lu_key, _ in sorted(copypastas):
            f.write(f"    \"{lu_key}\",\n")
        f.write('];\n\n')
            
        f.write("pub fn copypasta_lu() -> HashMap<&'static str, &'static str> {\n")
        f.write("    HashMap::from([\n")
        for var_name, lu_key, _ in sorted(copypastas):
            f.write(f'        ("{lu_key}", {var_name}),\n')
        f.write('        ("guoba", XIANGLING),\n')
        f.write("    ])\n")
        f.write("}\n")

        f.write("\n")

        f.write("pub static NAUGHTY_KEYS: &[&str] = &[\n")
        f.write('    "come", "hard", "long", "shaft", "finish", "coming", "came",\n')
        f.write('];\n')

        f.write("\n")

        f.write("pub fn find<'a>(needles: &[&'a str], haystack: &str) -> Option<&'a str> {")
        f.write("    needles.iter().cloned().find(|&key| haystack.contains(key))")
        f.write("}\n")

def refresh_lus():
    generate_lus("assets/misc_images")


if __name__ == "__main__":
    # refresh_pool()
    refresh_lus()

    try:
        subprocess.run(["cargo", "fmt"], check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    except FileNotFoundError:
        print("`cargo fmt` not found, skipping formatting.")

