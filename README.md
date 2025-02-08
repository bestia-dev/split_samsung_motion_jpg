[//]: # (auto_md_to_doc_comments segment start A)

# split_samsung_motion_jpg

[//]: # (auto_cargo_toml_to_md start)

**Split Samsung 3 seconds motion jpg into still photo and video(2025-02)**  
***version: 1.0.7 date: 2025-02-08 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/split_samsung_motion_jpg)***

 ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
 ![maintained](https://img.shields.io/badge/maintained-green)
 ![samsung](https://img.shields.io/badge/samsung-orange)
 ![motion](https://img.shields.io/badge/motion-orange)
 ![jpg](https://img.shields.io/badge/jpg-orange)

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-46-green.svg)]()
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-2-blue.svg)]()
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-10-purple.svg)]()
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)]()

[//]: # (auto_lines_of_code end)

  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/split_samsung_motion_jpg/blob/main/LICENSE)
  [![Rust](https://github.com/bestia-dev/split_samsung_motion_jpg/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/split_samsung_motion_jpg/)
 ![split_samsung_motion_jpg](https://bestia.dev/webpage_hit_counter/get_svg_image/1649120701.svg)

Hashtags: #rustlang #tutorial #cli  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

My Samsung S23 phone can take `motion photos`. It is a combination of a jpg photo with attached a short 3 seconds video. Most computers cannot play this strange format yet.

I want to split these files into a normal jpg and a normal mpf file.

# Original code

The original script is in python from <https://github.com/chlete/samsung-motion-photo-splitter>.

```original

Samsung's S7 Motion Photo splitter.
S7 generates a container which encapsulates picture and video. The first part
is a JPEG with its usual footer plus Samsung's own (MotionPhoto_Data). Second
part is the video.

Algorithm:
Count the bytes for each offset and write the files.
JPEG: byte zero to samsung footer end
MP4: JPEG's footer + 1 to end of file (size of the file)
```

I just translated it to Rust. I even used an AI translator to help with the initial translation. But it didn't work in the first try. I needed to correct it a bit.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
